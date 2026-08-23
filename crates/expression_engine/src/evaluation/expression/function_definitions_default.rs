use crate::ComputedItem;
use crate::evaluation::expression::function_definition::{
    ArgumentCount, FunctionDefinition, FunctionDefinitions,
};
use keys::store_key;
use message::message::{Message, MessageCategory};
use shareable_string::ShareableString;

/// Creates a translated error for a failed built-in function.
#[hotpath::measure]
fn function_error(
    key: impl Into<ShareableString>,
    params: impl IntoIterator<Item = (ShareableString, ShareableString)>,
) -> Message {
    crate::evaluation::create_error_message(
        MessageCategory::ExpressionEvaluation,
        key.into(),
        params.into_iter().collect(),
        None,
        None,
    )
}

/// Creates a translated error that identifies the built-in function that failed.
#[hotpath::measure]
fn function_argument_error(key: impl Into<ShareableString>, function_name: &str) -> Message {
    function_error(key, [("function".into(), function_name.into())])
}

/// The largest integer that can be represented exactly as an `f64` (2^53).
const MAX_EXACT_INTEGER_IN_F64: i64 = 9_007_199_254_740_992;
/// The minimum `i64` value expressed as an `f64` literal, used for range checks before casting.
const I64_MIN_F64: f64 = -9_223_372_036_854_775_808.0;
/// The maximum `i64` value expressed as an `f64` literal, used for range checks before casting.
const I64_MAX_F64: f64 = 9_223_372_036_854_775_807.0;

/// Truncates `value` to an `i64` via `trunc()`, returning an error if `value` is non-finite
/// or outside the representable `i64` range.
#[hotpath::measure]
fn truncated_f64_to_i64(value: f64, function_name: &str) -> Result<i64, Message> {
    if !value.is_finite() {
        return Err(function_argument_error(
            "expression_engine_function_argument_must_be_finite",
            function_name,
        ));
    }

    let truncated = value.trunc();
    if !(I64_MIN_F64..=I64_MAX_F64).contains(&truncated) {
        return Err(function_argument_error(
            "expression_engine_function_argument_out_of_integer_range",
            function_name,
        ));
    }

    format!("{truncated:.0}").parse::<i64>().map_err(|_| {
        function_argument_error(
            "expression_engine_function_argument_integer_conversion_failed",
            function_name,
        )
    })
}

/// Converts `value` to an `f64`, returning an error if it is outside the range that
/// can be represented exactly (i.e., beyond `±MAX_EXACT_INTEGER_IN_F64`).
#[hotpath::measure]
fn i64_to_f64(value: i64, function_name: &str) -> Result<f64, Message> {
    if !(-MAX_EXACT_INTEGER_IN_F64..=MAX_EXACT_INTEGER_IN_F64).contains(&value) {
        return Err(function_argument_error(
            "expression_engine_function_argument_float_precision_loss",
            function_name,
        ));
    }

    value.to_string().parse::<f64>().map_err(|_| {
        function_argument_error(
            "expression_engine_function_argument_float_conversion_failed",
            function_name,
        )
    })
}

/// Fetches the argument at `index`, without indexing directly into the
/// slice. Callers are expected to only be invoked after `ArgumentCount`
/// validation, so a missing argument here indicates an internal error
/// rather than a user-facing one.
#[hotpath::measure]
fn arg<'a>(
    args: &'a [ComputedItem],
    index: usize,
    function_name: &str,
) -> Result<&'a ComputedItem, Message> {
    args.get(index).ok_or_else(|| {
        function_argument_error(
            "expression_engine_function_missing_expected_argument",
            function_name,
        )
    })
}

/// Computes the sine of a float argument (radians).
#[hotpath::measure]
fn sin(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let arg = arg(args, 0, "sin")?;

    Ok(ComputedItem::Float(as_float(arg, "sin")?.sin()))
}

/// Computes the cosine of a float argument (radians).
#[hotpath::measure]
fn cos(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let arg = arg(args, 0, "cos")?;

    Ok(ComputedItem::Float(as_float(arg, "cos")?.cos()))
}

/// Computes the tangent of a float argument (radians).
#[hotpath::measure]
fn tan(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let arg = arg(args, 0, "tan")?;

    Ok(ComputedItem::Float(as_float(arg, "tan")?.tan()))
}

/// Computes the arcsine of a float argument, returning a value in radians.
#[hotpath::measure]
fn arcsin(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let arg = arg(args, 0, "arcsin")?;

    Ok(ComputedItem::Float(as_float(arg, "arcsin")?.asin()))
}

/// Computes the arccosine of a float argument, returning a value in radians.
#[hotpath::measure]
fn arccos(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let arg = arg(args, 0, "arccos")?;

    Ok(ComputedItem::Float(as_float(arg, "arccos")?.acos()))
}

/// Computes the arctangent of a float argument, returning a value in radians.
#[hotpath::measure]
fn arctan(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let arg = arg(args, 0, "arctan")?;

    Ok(ComputedItem::Float(as_float(arg, "arctan")?.atan()))
}

/// Extracts a floating-point value from a `ComputedItem`. Both unitless and
/// unit-bearing floats are accepted; integers are intentionally rejected so
/// that integer and floating-point values are never silently converted into
/// one another.
#[hotpath::measure]
fn as_float(item: &ComputedItem, function_name: &str) -> Result<f64, Message> {
    match item {
        ComputedItem::Float(value) | ComputedItem::FloatWithUnit { value, .. } => Ok(*value),
        ComputedItem::Boolean(_)
        | ComputedItem::Integer(_)
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => Err(function_argument_error(
            "expression_engine_function_argument_must_be_float",
            function_name,
        )),
    }
}

/// Builds the error returned when a function that requires all of its
/// numeric arguments to share the same type (`Integer` or `Float`) is called
/// with a mix of the two.
#[hotpath::measure]
fn mixed_numeric_types_error(function_name: &str) -> Message {
    function_argument_error(
        "expression_engine_function_arguments_mixed_numeric_types",
        function_name,
    )
}

/// Returns the absolute value of a numeric argument (float or integer).
#[hotpath::measure]
fn abs(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    match arg(args, 0, "abs")? {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.abs())),
        ComputedItem::Integer(value) => Ok(ComputedItem::Integer(value.abs())),
        ComputedItem::Boolean(_)
        | ComputedItem::FloatWithUnit { .. }
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => Err(function_argument_error(
            "expression_engine_function_argument_must_be_number",
            "abs",
        )),
    }
}

/// Computes the square root of a float argument.
#[hotpath::measure]
fn sqrt(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let value = as_float(arg(args, 0, "sqrt")?, "sqrt")?;
    Ok(ComputedItem::Float(value.sqrt()))
}

/// Returns the smallest integer greater than or equal to the argument.
#[hotpath::measure]
fn ceil(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    match arg(args, 0, "ceil")? {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.ceil())),
        ComputedItem::Integer(value) => Ok(ComputedItem::Integer(*value)),
        ComputedItem::Boolean(_)
        | ComputedItem::FloatWithUnit { .. }
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => Err(function_argument_error(
            "expression_engine_function_argument_must_be_number",
            "ceil",
        )),
    }
}

/// Returns the largest integer less than or equal to the argument.
#[hotpath::measure]
fn floor(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    match arg(args, 0, "floor")? {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.floor())),
        ComputedItem::Integer(value) => Ok(ComputedItem::Integer(*value)),
        ComputedItem::Boolean(_)
        | ComputedItem::FloatWithUnit { .. }
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => Err(function_argument_error(
            "expression_engine_function_argument_must_be_number",
            "floor",
        )),
    }
}

/// Rounds the argument to the nearest integer (ties round away from zero).
#[hotpath::measure]
fn round(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    match arg(args, 0, "round")? {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.round())),
        ComputedItem::Integer(value) => Ok(ComputedItem::Integer(*value)),
        ComputedItem::Boolean(_)
        | ComputedItem::FloatWithUnit { .. }
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => Err(function_argument_error(
            "expression_engine_function_argument_must_be_number",
            "round",
        )),
    }
}

/// Returns the minimum value among one or more numeric arguments of the same type.
#[hotpath::measure]
fn min(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    match arg(args, 0, "min")? {
        ComputedItem::Float(first) => {
            let mut result = *first;
            for arg in args.get(1..).unwrap_or_default() {
                match arg {
                    ComputedItem::Float(value) => result = result.min(*value),
                    ComputedItem::Boolean(_)
                    | ComputedItem::Integer(_)
                    | ComputedItem::FloatWithUnit { .. }
                    | ComputedItem::String(_)
                    | ComputedItem::Identifier(_)
                    | ComputedItem::Path(_)
                    | ComputedItem::Table(_)
                    | ComputedItem::TableWithUnits(_)
                    | ComputedItem::Unit(_) => return Err(mixed_numeric_types_error("min")),
                }
            }
            Ok(ComputedItem::Float(result))
        }
        ComputedItem::Integer(first) => {
            let mut result = *first;
            for arg in args.get(1..).unwrap_or_default() {
                match arg {
                    ComputedItem::Integer(value) => result = result.min(*value),
                    ComputedItem::Boolean(_)
                    | ComputedItem::Float(_)
                    | ComputedItem::FloatWithUnit { .. }
                    | ComputedItem::String(_)
                    | ComputedItem::Identifier(_)
                    | ComputedItem::Path(_)
                    | ComputedItem::Table(_)
                    | ComputedItem::TableWithUnits(_)
                    | ComputedItem::Unit(_) => return Err(mixed_numeric_types_error("min")),
                }
            }
            Ok(ComputedItem::Integer(result))
        }
        ComputedItem::Boolean(_)
        | ComputedItem::FloatWithUnit { .. }
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => Err(function_argument_error(
            "expression_engine_function_argument_must_be_number",
            "min",
        )),
    }
}

/// Returns the maximum value among one or more numeric arguments of the same type.
#[hotpath::measure]
fn max(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    match arg(args, 0, "max")? {
        ComputedItem::Float(first) => {
            let mut result = *first;
            for arg in args.get(1..).unwrap_or_default() {
                match arg {
                    ComputedItem::Float(value) => result = result.max(*value),
                    ComputedItem::Boolean(_)
                    | ComputedItem::Integer(_)
                    | ComputedItem::FloatWithUnit { .. }
                    | ComputedItem::String(_)
                    | ComputedItem::Identifier(_)
                    | ComputedItem::Path(_)
                    | ComputedItem::Table(_)
                    | ComputedItem::TableWithUnits(_)
                    | ComputedItem::Unit(_) => return Err(mixed_numeric_types_error("max")),
                }
            }
            Ok(ComputedItem::Float(result))
        }
        ComputedItem::Integer(first) => {
            let mut result = *first;
            for arg in args.get(1..).unwrap_or_default() {
                match arg {
                    ComputedItem::Integer(value) => result = result.max(*value),
                    ComputedItem::Boolean(_)
                    | ComputedItem::Float(_)
                    | ComputedItem::FloatWithUnit { .. }
                    | ComputedItem::String(_)
                    | ComputedItem::Identifier(_)
                    | ComputedItem::Path(_)
                    | ComputedItem::Table(_)
                    | ComputedItem::TableWithUnits(_)
                    | ComputedItem::Unit(_) => return Err(mixed_numeric_types_error("max")),
                }
            }
            Ok(ComputedItem::Integer(result))
        }
        ComputedItem::Boolean(_)
        | ComputedItem::FloatWithUnit { .. }
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => Err(function_argument_error(
            "expression_engine_function_argument_must_be_number",
            "max",
        )),
    }
}

/// Clamps the first argument to the inclusive range `[min, max]`.
#[hotpath::measure]
fn clamp(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    match (
        arg(args, 0, "clamp")?,
        arg(args, 1, "clamp")?,
        arg(args, 2, "clamp")?,
    ) {
        (
            ComputedItem::Float(value),
            ComputedItem::Float(min_value),
            ComputedItem::Float(max_value),
        ) => {
            if min_value > max_value {
                return Err(function_error(
                    "expression_engine_function_clamp_minimum_exceeds_maximum",
                    [],
                ));
            }
            Ok(ComputedItem::Float(value.clamp(*min_value, *max_value)))
        }
        (
            ComputedItem::Integer(value),
            ComputedItem::Integer(min_value),
            ComputedItem::Integer(max_value),
        ) => {
            if min_value > max_value {
                return Err(function_error(
                    "expression_engine_function_clamp_minimum_exceeds_maximum",
                    [],
                ));
            }
            Ok(ComputedItem::Integer(
                (*value).clamp(*min_value, *max_value),
            ))
        }
        _ => Err(mixed_numeric_types_error("clamp")),
    }
}

/// Computes the natural logarithm of a float argument.
#[hotpath::measure]
fn log(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let value = as_float(arg(args, 0, "log")?, "log")?;
    Ok(ComputedItem::Float(value.ln()))
}

/// Computes the base-2 logarithm of a float argument.
#[hotpath::measure]
fn log2(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let value = as_float(arg(args, 0, "log2")?, "log2")?;
    Ok(ComputedItem::Float(value.log2()))
}

/// Computes the base-10 logarithm of a float argument.
#[hotpath::measure]
fn log10(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let value = as_float(arg(args, 0, "log10")?, "log10")?;
    Ok(ComputedItem::Float(value.log10()))
}

/// Computes `e^x` for a float argument.
#[hotpath::measure]
fn exp(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let value = as_float(arg(args, 0, "exp")?, "exp")?;
    Ok(ComputedItem::Float(value.exp()))
}

/// Computes `atan2(y, x)` for two float arguments, returning the angle in radians.
#[hotpath::measure]
fn arctan2(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let y = as_float(arg(args, 0, "arctan2")?, "arctan2")?;
    let x = as_float(arg(args, 1, "arctan2")?, "arctan2")?;
    Ok(ComputedItem::Float(y.atan2(x)))
}

/// Computes the hyperbolic sine of a float argument.
#[hotpath::measure]
fn sinh(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let value = as_float(arg(args, 0, "sinh")?, "sinh")?;
    Ok(ComputedItem::Float(value.sinh()))
}

/// Computes the hyperbolic cosine of a float argument.
#[hotpath::measure]
fn cosh(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let value = as_float(arg(args, 0, "cosh")?, "cosh")?;
    Ok(ComputedItem::Float(value.cosh()))
}

/// Computes the hyperbolic tangent of a float argument.
#[hotpath::measure]
fn tanh(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let value = as_float(arg(args, 0, "tanh")?, "tanh")?;
    Ok(ComputedItem::Float(value.tanh()))
}

/// Converts a float argument from degrees to radians.
#[hotpath::measure]
fn to_radians(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let value = as_float(arg(args, 0, "to_radians")?, "to_radians")?;
    Ok(ComputedItem::Float(value.to_radians()))
}

/// Converts a float argument from radians to degrees.
#[hotpath::measure]
fn to_degrees(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let value = as_float(arg(args, 0, "to_degrees")?, "to_degrees")?;
    Ok(ComputedItem::Float(value.to_degrees()))
}

/// Returns the number of characters in a string argument as an integer.
#[hotpath::measure]
fn len(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let arg = arg(args, 0, "len")?;

    match arg {
        ComputedItem::String(value) => {
            let len = i64::try_from(value.as_str().len()).map_err(|_| {
                function_error("expression_engine_function_length_result_too_large", [])
            })?;
            Ok(ComputedItem::Integer(len))
        }
        ComputedItem::Boolean(_)
        | ComputedItem::Integer(_)
        | ComputedItem::Float(_)
        | ComputedItem::FloatWithUnit { .. }
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => Err(function_argument_error(
            "expression_engine_function_argument_must_be_string",
            "len",
        )),
    }
}

/// Converts a numeric argument to an integer, truncating towards zero for floats.
#[hotpath::measure]
fn to_int(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    match arg(args, 0, "to_int")? {
        ComputedItem::Integer(value) => Ok(ComputedItem::Integer(*value)),
        ComputedItem::Float(value) | ComputedItem::FloatWithUnit { value, .. } => {
            let int_value = truncated_f64_to_i64(*value, "to_int")?;
            Ok(ComputedItem::Integer(int_value))
        }
        ComputedItem::Boolean(_)
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => Err(function_argument_error(
            "expression_engine_function_argument_must_be_number",
            "to_int",
        )),
    }
}

/// Converts a numeric argument to a float, returning an error for integers outside the
/// exactly representable range.
#[hotpath::measure]
fn to_float(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    match arg(args, 0, "to_float")? {
        ComputedItem::Float(value) | ComputedItem::FloatWithUnit { value, .. } => {
            Ok(ComputedItem::Float(*value))
        }
        ComputedItem::Integer(value) => {
            let float_value = i64_to_f64(*value, "to_float")?;
            Ok(ComputedItem::Float(float_value))
        }
        ComputedItem::Boolean(_)
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => Err(function_argument_error(
            "expression_engine_function_argument_must_be_number",
            "to_float",
        )),
    }
}

/// Returns `true_value` if the boolean first argument is `true`, otherwise `false_value`.
#[hotpath::measure]
fn if_function(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let condition = arg(args, 0, "if")?;
    let true_value = arg(args, 1, "if")?;
    let false_value = arg(args, 2, "if")?;

    match condition {
        ComputedItem::Boolean(cond) => {
            if *cond {
                Ok(true_value.clone())
            } else {
                Ok(false_value.clone())
            }
        }
        ComputedItem::Integer(_)
        | ComputedItem::Float(_)
        | ComputedItem::FloatWithUnit { .. }
        | ComputedItem::String(_)
        | ComputedItem::Identifier(_)
        | ComputedItem::Path(_)
        | ComputedItem::Table(_)
        | ComputedItem::TableWithUnits(_)
        | ComputedItem::Unit(_) => Err(function_error(
            "expression_engine_function_if_condition_must_be_boolean",
            [],
        )),
    }
}

/// Returns a `FunctionDefinitions` containing the default mathematical functions.
#[hotpath::measure]
pub(crate) fn default_function_definitions() -> FunctionDefinitions {
    FunctionDefinitions::new()
        .with(FunctionDefinition::new(
            store_key!("sin"),
            "sine function",
            ArgumentCount::Exact { count: 1 },
            sin,
        ))
        .with(FunctionDefinition::new(
            store_key!("cos"),
            "cosine function",
            ArgumentCount::Exact { count: 1 },
            cos,
        ))
        .with(FunctionDefinition::new(
            store_key!("tan"),
            "tangent function",
            ArgumentCount::Exact { count: 1 },
            tan,
        ))
        .with(FunctionDefinition::new(
            store_key!("arcsin"),
            "inverse sine function",
            ArgumentCount::Exact { count: 1 },
            arcsin,
        ))
        .with(FunctionDefinition::new(
            store_key!("arccos"),
            "inverse cosine function",
            ArgumentCount::Exact { count: 1 },
            arccos,
        ))
        .with(FunctionDefinition::new(
            store_key!("arctan"),
            "inverse tangent function",
            ArgumentCount::Exact { count: 1 },
            arctan,
        ))
        .with(FunctionDefinition::new(
            store_key!("abs"),
            "absolute value function",
            ArgumentCount::Exact { count: 1 },
            abs,
        ))
        .with(FunctionDefinition::new(
            store_key!("sqrt"),
            "square root function",
            ArgumentCount::Exact { count: 1 },
            sqrt,
        ))
        .with(FunctionDefinition::new(
            store_key!("ceil"),
            "rounds a number up to the nearest integer",
            ArgumentCount::Exact { count: 1 },
            ceil,
        ))
        .with(FunctionDefinition::new(
            store_key!("floor"),
            "rounds a number down to the nearest integer",
            ArgumentCount::Exact { count: 1 },
            floor,
        ))
        .with(FunctionDefinition::new(
            store_key!("round"),
            "rounds a number to the nearest integer",
            ArgumentCount::Exact { count: 1 },
            round,
        ))
        .with(FunctionDefinition::new(
            store_key!("min"),
            "returns the smallest of its arguments",
            ArgumentCount::Min { min: 1 },
            min,
        ))
        .with(FunctionDefinition::new(
            store_key!("max"),
            "returns the largest of its arguments",
            ArgumentCount::Min { min: 1 },
            max,
        ))
        .with(FunctionDefinition::new(
            store_key!("clamp"),
            "clamps a value between a minimum and maximum",
            ArgumentCount::Exact { count: 3 },
            clamp,
        ))
        .with(FunctionDefinition::new(
            store_key!("log"),
            "natural logarithm function",
            ArgumentCount::Exact { count: 1 },
            log,
        ))
        .with(FunctionDefinition::new(
            store_key!("log2"),
            "base-2 logarithm function",
            ArgumentCount::Exact { count: 1 },
            log2,
        ))
        .with(FunctionDefinition::new(
            store_key!("log10"),
            "base-10 logarithm function",
            ArgumentCount::Exact { count: 1 },
            log10,
        ))
        .with(FunctionDefinition::new(
            store_key!("exp"),
            "e raised to the power of the argument",
            ArgumentCount::Exact { count: 1 },
            exp,
        ))
        .with(FunctionDefinition::new(
            store_key!("arctan2"),
            "two-argument inverse tangent function",
            ArgumentCount::Exact { count: 2 },
            arctan2,
        ))
        .with(FunctionDefinition::new(
            store_key!("sinh"),
            "hyperbolic sine function",
            ArgumentCount::Exact { count: 1 },
            sinh,
        ))
        .with(FunctionDefinition::new(
            store_key!("cosh"),
            "hyperbolic cosine function",
            ArgumentCount::Exact { count: 1 },
            cosh,
        ))
        .with(FunctionDefinition::new(
            store_key!("tanh"),
            "hyperbolic tangent function",
            ArgumentCount::Exact { count: 1 },
            tanh,
        ))
        .with(FunctionDefinition::new(
            store_key!("to_radians"),
            "converts an angle from degrees to radians",
            ArgumentCount::Exact { count: 1 },
            to_radians,
        ))
        .with(FunctionDefinition::new(
            store_key!("to_degrees"),
            "converts an angle from radians to degrees",
            ArgumentCount::Exact { count: 1 },
            to_degrees,
        ))
        .with(FunctionDefinition::new(
            store_key!("len"),
            "returns the length of a string",
            ArgumentCount::Exact { count: 1 },
            len,
        ))
        .with(FunctionDefinition::new(
            store_key!("to_int"),
            "converts a number to an integer",
            ArgumentCount::Exact { count: 1 },
            to_int,
        ))
        .with(FunctionDefinition::new(
            store_key!("to_float"),
            "converts a number to a float",
            ArgumentCount::Exact { count: 1 },
            to_float,
        ))
        .with(FunctionDefinition::new(
            store_key!("if"),
            "conditional function",
            ArgumentCount::Exact { count: 3 },
            if_function,
        ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::Sub;

    fn call(name: &str, args: &[ComputedItem]) -> ComputedItem {
        let definitions = default_function_definitions();
        let definition = definitions
            .get(name)
            .expect("function should be registered");
        definition.call(args).expect("function call should succeed")
    }

    fn assert_float_eq(name: &str, args: &[ComputedItem], expected: f64) {
        let result = call(name, args);
        assert!(
            matches!(result, ComputedItem::Float(value) if value.sub(expected).abs() < 1e-9),
            "{name} returned {result:?}, expected a float close to {expected}"
        );
    }

    fn assert_integer_eq(name: &str, args: &[ComputedItem], expected: i64) {
        let result = call(name, args);
        assert!(
            matches!(result, ComputedItem::Integer(value) if value == expected),
            "{name} returned {result:?}, expected integer {expected}"
        );
    }

    fn assert_error(
        name: &str,
        args: &[ComputedItem],
        expected_key: &str,
        expected_function: Option<&str>,
    ) {
        let definitions = default_function_definitions();
        let definition = definitions
            .get(name)
            .expect("function should be registered");
        let error = definition
            .call(args)
            .expect_err("{name} should have errored");

        assert_eq!(error.category(), MessageCategory::ExpressionEvaluation);
        assert_eq!(
            error.translate_data().message_key().as_str(),
            expected_key,
            "{name} returned an unexpected error key"
        );

        let params = error.translate_data().message_params();
        match expected_function {
            Some(function) => assert_eq!(
                params.get("function").map(ShareableString::as_str),
                Some(function)
            ),
            None => assert!(params.is_empty(), "{name} error should have no parameters"),
        }
    }

    #[test]
    fn abs_returns_absolute_value() {
        assert_float_eq("abs", &[ComputedItem::Float(-3.5)], 3.5);
        assert_integer_eq("abs", &[ComputedItem::Integer(-4)], 4);
    }

    #[test]
    fn sqrt_returns_square_root() {
        assert_float_eq("sqrt", &[ComputedItem::Float(9.0)], 3.0);
    }

    #[test]
    fn sqrt_errors_for_integer_argument() {
        assert_error(
            "sqrt",
            &[ComputedItem::Integer(9)],
            "expression_engine_function_argument_must_be_float",
            Some("sqrt"),
        );
    }

    #[test]
    fn ceil_floor_round_work_as_expected() {
        assert_float_eq("ceil", &[ComputedItem::Float(1.2)], 2.0);
        assert_float_eq("floor", &[ComputedItem::Float(1.8)], 1.0);
        assert_float_eq("round", &[ComputedItem::Float(1.5)], 2.0);
    }

    #[test]
    fn ceil_floor_round_preserve_integer_argument() {
        assert_integer_eq("ceil", &[ComputedItem::Integer(3)], 3);
        assert_integer_eq("floor", &[ComputedItem::Integer(3)], 3);
        assert_integer_eq("round", &[ComputedItem::Integer(3)], 3);
    }

    #[test]
    fn min_and_max_work_over_multiple_arguments_of_the_same_type() {
        assert_float_eq(
            "min",
            &[
                ComputedItem::Float(3.0),
                ComputedItem::Float(1.0),
                ComputedItem::Float(2.0),
            ],
            1.0,
        );
        assert_float_eq(
            "max",
            &[
                ComputedItem::Float(3.0),
                ComputedItem::Float(1.0),
                ComputedItem::Float(2.0),
            ],
            3.0,
        );
        assert_integer_eq(
            "min",
            &[
                ComputedItem::Integer(3),
                ComputedItem::Integer(1),
                ComputedItem::Integer(2),
            ],
            1,
        );
        assert_integer_eq(
            "max",
            &[
                ComputedItem::Integer(3),
                ComputedItem::Integer(1),
                ComputedItem::Integer(2),
            ],
            3,
        );
    }

    #[test]
    fn min_and_max_error_on_mixed_argument_types() {
        assert_error(
            "min",
            &[ComputedItem::Float(3.0), ComputedItem::Integer(1)],
            "expression_engine_function_arguments_mixed_numeric_types",
            Some("min"),
        );
        assert_error(
            "max",
            &[ComputedItem::Float(3.0), ComputedItem::Integer(1)],
            "expression_engine_function_arguments_mixed_numeric_types",
            Some("max"),
        );
    }

    #[test]
    fn numeric_functions_report_structured_non_numeric_argument_errors() {
        let args = [ComputedItem::Boolean(false)];
        for name in ["abs", "ceil", "floor", "round", "min", "max"] {
            assert_error(
                name,
                &args,
                "expression_engine_function_argument_must_be_number",
                Some(name),
            );
        }
    }

    #[test]
    fn clamp_restricts_value_to_range() {
        assert_float_eq(
            "clamp",
            &[
                ComputedItem::Float(5.0),
                ComputedItem::Float(0.0),
                ComputedItem::Float(3.0),
            ],
            3.0,
        );
        assert_float_eq(
            "clamp",
            &[
                ComputedItem::Float(-5.0),
                ComputedItem::Float(0.0),
                ComputedItem::Float(3.0),
            ],
            0.0,
        );
        assert_float_eq(
            "clamp",
            &[
                ComputedItem::Float(1.0),
                ComputedItem::Float(0.0),
                ComputedItem::Float(3.0),
            ],
            1.0,
        );
        assert_integer_eq(
            "clamp",
            &[
                ComputedItem::Integer(5),
                ComputedItem::Integer(0),
                ComputedItem::Integer(3),
            ],
            3,
        );
    }

    #[test]
    fn clamp_errors_when_min_greater_than_max() {
        assert_error(
            "clamp",
            &[
                ComputedItem::Float(1.0),
                ComputedItem::Float(3.0),
                ComputedItem::Float(0.0),
            ],
            "expression_engine_function_clamp_minimum_exceeds_maximum",
            None,
        );
    }

    #[test]
    fn clamp_errors_on_mixed_argument_types() {
        assert_error(
            "clamp",
            &[
                ComputedItem::Float(1.0),
                ComputedItem::Integer(0),
                ComputedItem::Float(3.0),
            ],
            "expression_engine_function_arguments_mixed_numeric_types",
            Some("clamp"),
        );
    }

    #[test]
    fn log_functions_compute_expected_values() {
        assert_float_eq("log", &[ComputedItem::Float(std::f64::consts::E)], 1.0);
        assert_float_eq("log2", &[ComputedItem::Float(8.0)], 3.0);
        assert_float_eq("log10", &[ComputedItem::Float(1000.0)], 3.0);
    }

    #[test]
    fn log_functions_error_for_integer_argument() {
        assert_error(
            "log",
            &[ComputedItem::Integer(1)],
            "expression_engine_function_argument_must_be_float",
            Some("log"),
        );
        assert_error(
            "log2",
            &[ComputedItem::Integer(8)],
            "expression_engine_function_argument_must_be_float",
            Some("log2"),
        );
        assert_error(
            "log10",
            &[ComputedItem::Integer(1000)],
            "expression_engine_function_argument_must_be_float",
            Some("log10"),
        );
    }

    #[test]
    fn exp_computes_e_to_the_power_of_argument() {
        assert_float_eq("exp", &[ComputedItem::Float(1.0)], std::f64::consts::E);
    }

    #[test]
    fn exp_errors_for_integer_argument() {
        assert_error(
            "exp",
            &[ComputedItem::Integer(1)],
            "expression_engine_function_argument_must_be_float",
            Some("exp"),
        );
    }

    #[test]
    fn arctan2_computes_two_argument_arctangent() {
        assert_float_eq(
            "arctan2",
            &[ComputedItem::Float(1.0), ComputedItem::Float(1.0)],
            std::f64::consts::FRAC_PI_4,
        );
    }

    #[test]
    fn arctan2_errors_for_integer_argument() {
        assert_error(
            "arctan2",
            &[ComputedItem::Integer(1), ComputedItem::Float(1.0)],
            "expression_engine_function_argument_must_be_float",
            Some("arctan2"),
        );
        assert_error(
            "arctan2",
            &[ComputedItem::Float(1.0), ComputedItem::Integer(1)],
            "expression_engine_function_argument_must_be_float",
            Some("arctan2"),
        );
    }

    #[test]
    fn hyperbolic_functions_compute_expected_values() {
        assert_float_eq("sinh", &[ComputedItem::Float(0.0)], 0.0);
        assert_float_eq("cosh", &[ComputedItem::Float(0.0)], 1.0);
        assert_float_eq("tanh", &[ComputedItem::Float(0.0)], 0.0);
    }

    #[test]
    fn hyperbolic_functions_error_for_integer_argument() {
        assert_error(
            "sinh",
            &[ComputedItem::Integer(0)],
            "expression_engine_function_argument_must_be_float",
            Some("sinh"),
        );
        assert_error(
            "cosh",
            &[ComputedItem::Integer(0)],
            "expression_engine_function_argument_must_be_float",
            Some("cosh"),
        );
        assert_error(
            "tanh",
            &[ComputedItem::Integer(0)],
            "expression_engine_function_argument_must_be_float",
            Some("tanh"),
        );
    }

    #[test]
    fn angle_conversion_functions_work_as_expected() {
        assert_float_eq(
            "to_radians",
            &[ComputedItem::Float(180.0)],
            std::f64::consts::PI,
        );
        assert_float_eq(
            "to_degrees",
            &[ComputedItem::Float(std::f64::consts::PI)],
            180.0,
        );
    }

    #[test]
    fn angle_conversion_functions_error_for_integer_argument() {
        assert_error(
            "to_radians",
            &[ComputedItem::Integer(180)],
            "expression_engine_function_argument_must_be_float",
            Some("to_radians"),
        );
        assert_error(
            "to_degrees",
            &[ComputedItem::Integer(180)],
            "expression_engine_function_argument_must_be_float",
            Some("to_degrees"),
        );
    }

    #[test]
    fn len_returns_string_length() {
        let result = call(
            "len",
            &[ComputedItem::String(ShareableString::new("hello"))],
        );
        assert!(
            matches!(result, ComputedItem::Integer(5)),
            "expected an integer result of 5, got {result:?}"
        );
    }

    #[test]
    fn len_errors_for_non_string_argument() {
        assert_error(
            "len",
            &[ComputedItem::Float(1.0)],
            "expression_engine_function_argument_must_be_string",
            Some("len"),
        );
    }

    #[test]
    fn to_int_converts_float_to_integer_and_preserves_integer() {
        assert_integer_eq("to_int", &[ComputedItem::Float(3.7)], 3);
        assert_integer_eq("to_int", &[ComputedItem::Float(-3.7)], -3);
        assert_integer_eq("to_int", &[ComputedItem::Integer(5)], 5);
    }

    #[test]
    fn to_int_errors_for_non_numeric_argument() {
        assert_error(
            "to_int",
            &[ComputedItem::String(ShareableString::new("abc"))],
            "expression_engine_function_argument_must_be_number",
            Some("to_int"),
        );
    }

    #[test]
    fn to_int_reports_structured_conversion_errors() {
        assert_error(
            "to_int",
            &[ComputedItem::Float(f64::INFINITY)],
            "expression_engine_function_argument_must_be_finite",
            Some("to_int"),
        );
        assert_error(
            "to_int",
            &[ComputedItem::Float(I64_MAX_F64 * 2.0)],
            "expression_engine_function_argument_out_of_integer_range",
            Some("to_int"),
        );
        assert_error(
            "to_int",
            &[ComputedItem::Float(I64_MAX_F64)],
            "expression_engine_function_argument_integer_conversion_failed",
            Some("to_int"),
        );
    }

    #[test]
    fn to_float_converts_integer_to_float_and_preserves_float() {
        assert_float_eq("to_float", &[ComputedItem::Integer(5)], 5.0);
        assert_float_eq("to_float", &[ComputedItem::Float(3.5)], 3.5);
    }

    #[test]
    fn to_float_errors_for_non_numeric_argument() {
        assert_error(
            "to_float",
            &[ComputedItem::String(ShareableString::new("abc"))],
            "expression_engine_function_argument_must_be_number",
            Some("to_float"),
        );
    }

    #[test]
    fn to_float_reports_precision_loss_with_function_parameter() {
        assert_error(
            "to_float",
            &[ComputedItem::Integer(i64::MAX)],
            "expression_engine_function_argument_float_precision_loss",
            Some("to_float"),
        );
    }

    #[test]
    fn missing_arguments_report_the_function_name() {
        assert_error(
            "sqrt",
            &[],
            "expression_engine_function_missing_expected_argument",
            Some("sqrt"),
        );
    }

    #[test]
    fn if_requires_a_boolean_condition() {
        assert_error(
            "if",
            &[
                ComputedItem::Integer(1),
                ComputedItem::Integer(2),
                ComputedItem::Integer(3),
            ],
            "expression_engine_function_if_condition_must_be_boolean",
            None,
        );
    }

    #[test]
    fn arccos_and_arctan_are_registered_once() {
        let definitions = default_function_definitions();
        assert!(definitions.get("arccos").is_some());
        assert!(definitions.get("arctan").is_some());
    }
}
