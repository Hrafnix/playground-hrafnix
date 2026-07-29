use crate::evaluation::expression::function_definition::{FunctionDefinition, FunctionDefinitions};
use crate::expression::function_definition::ArgumentCount;
use crate::{ComputedItem, ExpressionError};
use datastore::store_key;

fn sin(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.sin())),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "sin function argument must be a number".to_string(),
        )),
    }
}

fn cos(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.cos())),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "cos function argument must be a number".to_string(),
        )),
    }
}

fn tan(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.tan())),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "tan function argument must be a number".to_string(),
        )),
    }
}

fn arcsin(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.asin())),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "arcsin function argument must be a number".to_string(),
        )),
    }
}

fn arccos(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.acos())),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "arccos function argument must be a number".to_string(),
        )),
    }
}

fn arctan(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let arg = &args[0];

    match arg {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.atan())),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "arctan function argument must be a number".to_string(),
        )),
    }
}

/// Extracts a floating-point value from a `ComputedItem`. Only the `Float`
/// variant is accepted; `Integer` is intentionally rejected so that integer
/// and floating-point values are never silently converted into one another.
fn as_float(item: &ComputedItem, function_name: &str) -> Result<f64, ExpressionError> {
    match item {
        ComputedItem::Float(value) => Ok(*value),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            format!("{function_name} function argument must be a float"),
        )),
    }
}

/// Builds the error returned when a function that requires all of its
/// numeric arguments to share the same type (`Integer` or `Float`) is called
/// with a mix of the two.
fn mixed_numeric_types_error(function_name: &str) -> ExpressionError {
    ExpressionError::new(
        crate::ExpressionCategory::Evaluation,
        format!("{function_name} function arguments must all be the same numeric type"),
    )
}

fn abs(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    match &args[0] {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.abs())),
        ComputedItem::Integer(value) => Ok(ComputedItem::Integer(value.abs())),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "abs function argument must be a number".to_string(),
        )),
    }
}

fn sqrt(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let value = as_float(&args[0], "sqrt")?;
    Ok(ComputedItem::Float(value.sqrt()))
}

fn ceil(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    match &args[0] {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.ceil())),
        ComputedItem::Integer(value) => Ok(ComputedItem::Integer(*value)),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "ceil function argument must be a number".to_string(),
        )),
    }
}

fn floor(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    match &args[0] {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.floor())),
        ComputedItem::Integer(value) => Ok(ComputedItem::Integer(*value)),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "floor function argument must be a number".to_string(),
        )),
    }
}

fn round(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    match &args[0] {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(value.round())),
        ComputedItem::Integer(value) => Ok(ComputedItem::Integer(*value)),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "round function argument must be a number".to_string(),
        )),
    }
}

fn min(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    match &args[0] {
        ComputedItem::Float(first) => {
            let mut result = *first;
            for arg in &args[1..] {
                match arg {
                    ComputedItem::Float(value) => result = result.min(*value),
                    _ => return Err(mixed_numeric_types_error("min")),
                }
            }
            Ok(ComputedItem::Float(result))
        }
        ComputedItem::Integer(first) => {
            let mut result = *first;
            for arg in &args[1..] {
                match arg {
                    ComputedItem::Integer(value) => result = result.min(*value),
                    _ => return Err(mixed_numeric_types_error("min")),
                }
            }
            Ok(ComputedItem::Integer(result))
        }
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "min function argument must be a number".to_string(),
        )),
    }
}

fn max(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    match &args[0] {
        ComputedItem::Float(first) => {
            let mut result = *first;
            for arg in &args[1..] {
                match arg {
                    ComputedItem::Float(value) => result = result.max(*value),
                    _ => return Err(mixed_numeric_types_error("max")),
                }
            }
            Ok(ComputedItem::Float(result))
        }
        ComputedItem::Integer(first) => {
            let mut result = *first;
            for arg in &args[1..] {
                match arg {
                    ComputedItem::Integer(value) => result = result.max(*value),
                    _ => return Err(mixed_numeric_types_error("max")),
                }
            }
            Ok(ComputedItem::Integer(result))
        }
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "max function argument must be a number".to_string(),
        )),
    }
}

fn clamp(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    match (&args[0], &args[1], &args[2]) {
        (
            ComputedItem::Float(value),
            ComputedItem::Float(min_value),
            ComputedItem::Float(max_value),
        ) => {
            if min_value > max_value {
                return Err(ExpressionError::new(
                    crate::ExpressionCategory::Evaluation,
                    "clamp function min argument must not be greater than max argument".to_string(),
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
                return Err(ExpressionError::new(
                    crate::ExpressionCategory::Evaluation,
                    "clamp function min argument must not be greater than max argument".to_string(),
                ));
            }
            Ok(ComputedItem::Integer(
                (*value).clamp(*min_value, *max_value),
            ))
        }
        _ => Err(mixed_numeric_types_error("clamp")),
    }
}

fn log(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let value = as_float(&args[0], "log")?;
    Ok(ComputedItem::Float(value.ln()))
}

fn log2(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let value = as_float(&args[0], "log2")?;
    Ok(ComputedItem::Float(value.log2()))
}

fn log10(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let value = as_float(&args[0], "log10")?;
    Ok(ComputedItem::Float(value.log10()))
}

fn exp(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let value = as_float(&args[0], "exp")?;
    Ok(ComputedItem::Float(value.exp()))
}

fn arctan2(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let y = as_float(&args[0], "arctan2")?;
    let x = as_float(&args[1], "arctan2")?;
    Ok(ComputedItem::Float(y.atan2(x)))
}

fn sinh(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let value = as_float(&args[0], "sinh")?;
    Ok(ComputedItem::Float(value.sinh()))
}

fn cosh(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let value = as_float(&args[0], "cosh")?;
    Ok(ComputedItem::Float(value.cosh()))
}

fn tanh(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let value = as_float(&args[0], "tanh")?;
    Ok(ComputedItem::Float(value.tanh()))
}

fn to_radians(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let value = as_float(&args[0], "to_radians")?;
    Ok(ComputedItem::Float(value.to_radians()))
}

fn to_degrees(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let value = as_float(&args[0], "to_degrees")?;
    Ok(ComputedItem::Float(value.to_degrees()))
}

fn len(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let arg = &args[0];

    match arg {
        ComputedItem::String(value) => Ok(ComputedItem::Integer(value.as_str().len() as i64)),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "len function argument must be a string".to_string(),
        )),
    }
}

fn to_int(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    match &args[0] {
        ComputedItem::Integer(value) => Ok(ComputedItem::Integer(*value)),
        ComputedItem::Float(value) => Ok(ComputedItem::Integer(*value as i64)),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "to_int function argument must be a number".to_string(),
        )),
    }
}

fn to_float(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    match &args[0] {
        ComputedItem::Float(value) => Ok(ComputedItem::Float(*value)),
        ComputedItem::Integer(value) => Ok(ComputedItem::Float(*value as f64)),
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "to_float function argument must be a number".to_string(),
        )),
    }
}

fn if_function(args: &[ComputedItem]) -> Result<ComputedItem, ExpressionError> {
    let condition = &args[0];
    let true_value = &args[1];
    let false_value = &args[2];

    match condition {
        ComputedItem::Boolean(cond) => {
            if *cond {
                Ok(true_value.clone())
            } else {
                Ok(false_value.clone())
            }
        }
        _ => Err(ExpressionError::new(
            crate::ExpressionCategory::Evaluation,
            "if function first argument must be a boolean".to_string(),
        )),
    }
}

/// Returns a `FunctionDefinitions` containing the default mathematical functions.
pub(crate) fn get_default_function_definitions() -> FunctionDefinitions {
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
    use shareable_string::ShareableString;

    fn call(name: &str, args: &[ComputedItem]) -> ComputedItem {
        let definitions = get_default_function_definitions();
        let definition = definitions
            .get(name)
            .expect("function should be registered");
        definition.call(args).expect("function call should succeed")
    }

    fn assert_float_eq(name: &str, args: &[ComputedItem], expected: f64) {
        match call(name, args) {
            ComputedItem::Float(value) => assert!(
                (value - expected).abs() < 1e-9,
                "{name} returned {value}, expected {expected}"
            ),
            other => panic!("expected a float result for {name}, got {other:?}"),
        }
    }

    fn assert_integer_eq(name: &str, args: &[ComputedItem], expected: i64) {
        match call(name, args) {
            ComputedItem::Integer(value) => assert_eq!(
                value, expected,
                "{name} returned {value}, expected {expected}"
            ),
            other => panic!("expected an integer result for {name}, got {other:?}"),
        }
    }

    fn assert_errors(name: &str, args: &[ComputedItem]) {
        let definitions = get_default_function_definitions();
        let definition = definitions.get(name).unwrap();
        assert!(definition.call(args).is_err(), "{name} should have errored");
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
        assert_errors("sqrt", &[ComputedItem::Integer(9)]);
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
        assert_errors("min", &[ComputedItem::Float(3.0), ComputedItem::Integer(1)]);
        assert_errors("max", &[ComputedItem::Float(3.0), ComputedItem::Integer(1)]);
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
        assert_errors(
            "clamp",
            &[
                ComputedItem::Float(1.0),
                ComputedItem::Float(3.0),
                ComputedItem::Float(0.0),
            ],
        );
    }

    #[test]
    fn clamp_errors_on_mixed_argument_types() {
        assert_errors(
            "clamp",
            &[
                ComputedItem::Float(1.0),
                ComputedItem::Integer(0),
                ComputedItem::Float(3.0),
            ],
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
        assert_errors("log", &[ComputedItem::Integer(1)]);
        assert_errors("log2", &[ComputedItem::Integer(8)]);
        assert_errors("log10", &[ComputedItem::Integer(1000)]);
    }

    #[test]
    fn exp_computes_e_to_the_power_of_argument() {
        assert_float_eq("exp", &[ComputedItem::Float(1.0)], std::f64::consts::E);
    }

    #[test]
    fn exp_errors_for_integer_argument() {
        assert_errors("exp", &[ComputedItem::Integer(1)]);
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
        assert_errors(
            "arctan2",
            &[ComputedItem::Integer(1), ComputedItem::Float(1.0)],
        );
        assert_errors(
            "arctan2",
            &[ComputedItem::Float(1.0), ComputedItem::Integer(1)],
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
        assert_errors("sinh", &[ComputedItem::Integer(0)]);
        assert_errors("cosh", &[ComputedItem::Integer(0)]);
        assert_errors("tanh", &[ComputedItem::Integer(0)]);
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
        assert_errors("to_radians", &[ComputedItem::Integer(180)]);
        assert_errors("to_degrees", &[ComputedItem::Integer(180)]);
    }

    #[test]
    fn len_returns_string_length() {
        let result = call(
            "len",
            &[ComputedItem::String(ShareableString::new("hello"))],
        );
        match result {
            ComputedItem::Integer(value) => assert_eq!(value, 5),
            other => panic!("expected an integer result, got {other:?}"),
        }
    }

    #[test]
    fn len_errors_for_non_string_argument() {
        let definitions = get_default_function_definitions();
        let definition = definitions.get("len").unwrap();
        let result = definition.call(&[ComputedItem::Float(1.0)]);
        assert!(result.is_err());
    }

    #[test]
    fn to_int_converts_float_to_integer_and_preserves_integer() {
        assert_integer_eq("to_int", &[ComputedItem::Float(3.7)], 3);
        assert_integer_eq("to_int", &[ComputedItem::Float(-3.7)], -3);
        assert_integer_eq("to_int", &[ComputedItem::Integer(5)], 5);
    }

    #[test]
    fn to_int_errors_for_non_numeric_argument() {
        assert_errors(
            "to_int",
            &[ComputedItem::String(ShareableString::new("abc"))],
        );
    }

    #[test]
    fn to_float_converts_integer_to_float_and_preserves_float() {
        assert_float_eq("to_float", &[ComputedItem::Integer(5)], 5.0);
        assert_float_eq("to_float", &[ComputedItem::Float(3.5)], 3.5);
    }

    #[test]
    fn to_float_errors_for_non_numeric_argument() {
        assert_errors(
            "to_float",
            &[ComputedItem::String(ShareableString::new("abc"))],
        );
    }

    #[test]
    fn arccos_and_arctan_are_registered_once() {
        let definitions = get_default_function_definitions();
        assert!(definitions.get("arccos").is_some());
        assert!(definitions.get("arctan").is_some());
    }
}
