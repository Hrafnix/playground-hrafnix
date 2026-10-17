use hrafnix_datastore::prelude::*;
use hrafnix_expression_engine::prelude::*;
use hrafnix_message::message::{Message, MessageCategory, MessageLevel};
use std::ops::{Mul, Sub};

fn evaluation_message(actual: impl Into<ShareableString>) -> Message {
    Message::new_with_params(
        MessageLevel::Error,
        MessageCategory::ExpressionEvaluation,
        "expression_engine_evaluation_custom_function_failed".into(),
        [("actual".into(), actual.into())].into_iter().collect(),
        None,
    )
}

/// A function that sums its integer arguments.
fn add_integers(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    let mut total: i64 = 0;
    for arg in args {
        match arg {
            ComputedItem::Integer(value) => total += value,
            other => {
                return Err(evaluation_message(format!(
                    "add() expects integer arguments, got {other:?}"
                )));
            }
        }
    }
    Ok(ComputedItem::Integer(total))
}

/// A function that multiplies two float arguments.
fn multiply_floats(args: &[ComputedItem]) -> Result<ComputedItem, Message> {
    match args {
        [ComputedItem::Float(a), ComputedItem::Float(b)] => Ok(ComputedItem::Float(a.mul(b))),
        _ => Err(evaluation_message(
            "multiply() expects exactly two float arguments",
        )),
    }
}

fn build_parameter(definition: ParameterObjectDefinition) -> ParameterObjectInputData {
    ParameterObjectInputData::new(&ParameterObjectFrozen::new(definition))
}

fn evaluate_number_expression(expression: &str) -> ComputedItem {
    let data = build_parameter(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_result"),
                NumberDefinition::new_with_default("A number parameter", expression),
            )
            .finish(),
    );

    ExpressionEngine::new()
        .evaluate_parameters(&data)
        .expect("evaluation should succeed")
        .get("p_result")
        .expect("p_result should be computed")
        .clone()
}

fn evaluate_integer_expression(expression: &str) -> ComputedItem {
    let data = build_parameter(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_result"),
                IntegerDefinition::new_with_default("An integer parameter", expression),
            )
            .finish(),
    );

    ExpressionEngine::new()
        .evaluate_parameters(&data)
        .expect("evaluation should succeed")
        .get("p_result")
        .expect("p_result should be computed")
        .clone()
}

fn assert_float_expression(expression: &str, expected: f64) {
    match evaluate_number_expression(expression) {
        ComputedItem::Float(actual) => assert!(
            actual.sub(expected).abs() <= f64::EPSILON,
            "expected expression `{expression}` to equal {expected}, got {actual}"
        ),
        other => panic!("expected expression `{expression}` to produce a float, got {other:?}"),
    }
}

fn assert_integer_expression(expression: &str, expected: i64) {
    match evaluate_integer_expression(expression) {
        ComputedItem::Integer(actual) => assert_eq!(
            actual, expected,
            "expression `{expression}` produced the wrong integer"
        ),
        other => panic!("expected expression `{expression}` to produce an integer, got {other:?}"),
    }
}

#[test]
fn default_trigonometric_functions_combine_with_arithmetic() {
    assert_float_expression("2.0 * sin(g_pi / 2.0) + cos(0.0) - tan(g_pi / 4.0)", 2.0);
}

#[test]
fn default_inverse_trigonometric_functions_can_be_nested() {
    assert_float_expression(
        "to_degrees(arcsin(1.0) + arccos(0.0) + arctan(1.0) + arctan2(1.0, 1.0))",
        270.0,
    );
}

#[test]
fn default_rounding_and_absolute_functions_preserve_float_math() {
    assert_float_expression(
        "ceil(2.1) + floor(3.9) + round(4.5) + abs(-3.0) / 2.0",
        12.5,
    );
}

#[test]
fn default_min_max_clamp_and_abs_functions_compose_with_integer_math() {
    assert_integer_expression("max(2, min(9, 4 + 3)) + clamp(-5, -2, 2) + abs(-3)", 8);
}

#[test]
fn default_root_logarithm_and_exponential_functions_can_be_chained() {
    assert_float_expression(
        "sqrt(81.0) + log(exp(2.0)) + log2(8.0) + log10(100.0)",
        16.0,
    );
}

#[test]
fn default_hyperbolic_functions_work_with_power_and_subtraction() {
    assert_float_expression(
        "cosh(1.0)^2.0 - sinh(1.0)^2.0 + tanh(0.0)",
        1.000_000_000_000_000_7,
    );
}

#[test]
fn default_angle_conversion_functions_work_inside_larger_expressions() {
    assert_float_expression("to_degrees(g_pi) / 2.0 + to_radians(180.0) / g_pi", 91.0);
}

#[test]
fn default_length_and_numeric_conversion_functions_can_be_nested() {
    assert_float_expression("to_float(to_int(5.9) + len(\"four\")) / 3.0", 3.0);
}

#[test]
fn default_conditional_function_accepts_computed_branches() {
    assert_integer_expression("if(2 + 2 == 4, max(3, 7) * 2, min(8, 9))", 14);
}

#[test]
fn default_trigonometric_functions_preserve_decimal_arguments() {
    assert_float_expression(
        "sin(0.125) + cos(0.375) - tan(0.25)",
        0.7998404340765056_f64,
    );
}

#[test]
fn default_range_functions_combine_decimal_places_without_losing_precision() {
    assert_float_expression(
        "clamp(12.345, 1.125, 9.875) - min(3.625, 4.875) + max(0.125, 0.25)",
        6.5,
    );
}

#[test]
fn default_logarithmic_functions_accept_fractional_decimal_values() {
    assert_float_expression("sqrt(2.25) + log10(0.01) + log2(0.125) + exp(0.0)", -2.5);
}

#[test]
fn default_rounding_functions_handle_values_near_decimal_boundaries() {
    assert_float_expression(
        "ceil(1.0001) + floor(2.9999) + round(3.4999) + round(3.5001)",
        11.0,
    );
}

#[test]
fn default_numeric_conversions_handle_decimal_places_inside_arithmetic() {
    assert_float_expression("to_float(to_int(123.987)) / 8.0", 15.375);
}

#[test]
fn decimal_trigonometric_expression_matches_golden_value() {
    assert_float_expression(
        "sin(0.123456789) + cos(0.987654321) - tan(0.314159265)",
        0.348_873_117_208_259_14,
    );
}

#[test]
fn decimal_inverse_trigonometric_expression_matches_golden_value() {
    assert_float_expression(
        "arcsin(0.333333333) + arccos(0.666666667) + arctan(0.777777777) \
         + arctan2(0.123456789, 0.987654321)",
        1.966_303_741_012_579_9,
    );
}

#[test]
fn decimal_hyperbolic_expression_matches_golden_value() {
    assert_float_expression(
        "sinh(0.125) + cosh(0.375) - tanh(0.625)",
        0.641_866_399_596_32,
    );
}

#[test]
fn decimal_logarithmic_expression_matches_golden_value() {
    assert_float_expression(
        "sqrt(12.3456789) + log(1.23456789) + log2(3.14159265) \
         + log10(98.7654321) + exp(0.123456789)",
        8.501_865_061_314_87,
    );
}

#[test]
fn decimal_angle_conversion_expression_matches_golden_value() {
    assert_float_expression(
        "to_degrees(0.123456789) + to_radians(12.3456789)",
        7.289_025_707_136_119,
    );
}

#[test]
fn registered_function_is_invoked_during_evaluation() {
    // Why: Test that a registered custom function is invoked and its result used during evaluation.
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            IntegerDefinition::new_with_default("A number parameter", "add(2, 3)"),
        )
        .finish();

    let data = build_parameter(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("add"),
            "sums integer arguments",
            ArgumentCount::Unbounded,
            add_integers,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    let number = output.get("p_result").expect("p_result should be computed");
    match number {
        ComputedItem::Integer(value) => assert_eq!(*value, 5),
        other => panic!("expected integer 5, got {other:?}"),
    }
}

#[test]
fn registered_function_can_reference_variables() {
    // Why: Test that a registered function can accept a global variable as one of its arguments.

    // Parameters are evaluated against the engine's globals, so set up a global
    // variable that the function can reference alongside a literal argument.
    let global_frozen = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Test Globals")
            .with(
                global_key!("g_a"),
                IntegerDefinition::new_with_default("a global operand", "10"),
            )
            .finish(),
    );

    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_sum"),
            IntegerDefinition::new_with_default("sum of operands", "add(g_a, 32)"),
        )
        .finish();

    let global_data = GlobalObjectInputData::new(&global_frozen);
    let data = build_parameter(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("add"),
            "sums integer arguments",
            ArgumentCount::Unbounded,
            add_integers,
        ))
        .expect("function should register");
    engine
        .evaluate_globals(&global_data)
        .expect("globals should evaluate");

    let output = engine
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    match output.get("p_sum").expect("p_sum should be computed") {
        ComputedItem::Integer(value) => assert_eq!(*value, 42),
        other => panic!("expected integer 42, got {other:?}"),
    }
}

#[test]
fn registered_function_combines_with_other_operators() {
    // Why: Test that a function call result can be combined with arithmetic operators in the same expression.
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            IntegerDefinition::new_with_default("a number parameter", "add(2, 3) * 4 - 1"),
        )
        .finish();

    let data = build_parameter(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("add"),
            "sums integer arguments",
            ArgumentCount::Unbounded,
            add_integers,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    match output.get("p_result").expect("p_result should be computed") {
        ComputedItem::Integer(value) => assert_eq!(*value, 19),
        other => panic!("expected integer 19, got {other:?}"),
    }
}

#[test]
fn nested_registered_function_calls_evaluate_correctly() {
    // Why: Test that nested calls to a registered function are evaluated correctly.
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            IntegerDefinition::new_with_default("a number parameter", "add(add(1, 2), add(3, 4))"),
        )
        .finish();

    let data = build_parameter(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("add"),
            "sums integer arguments",
            ArgumentCount::Unbounded,
            add_integers,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    match output.get("p_result").expect("p_result should be computed") {
        ComputedItem::Integer(value) => assert_eq!(*value, 10),
        other => panic!("expected integer 10, got {other:?}"),
    }
}

#[test]
fn float_returning_function_works_with_number_definition() {
    // Why: Test that a registered function returning a float value is compatible with a NumberDefinition parameter.
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            NumberDefinition::new_with_default("a number parameter", "multiply(2.5, 4.0)"),
        )
        .finish();

    let data = build_parameter(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("multiply"),
            "multiplies two floats",
            ArgumentCount::Exact { count: 2 },
            multiply_floats,
        ))
        .expect("function should register");

    let output = engine
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    match output.get("p_result").expect("p_result should be computed") {
        ComputedItem::Float(value) => {
            let expected: f64 = 10.0;
            assert!(
                value.sub(expected).abs() < f64::EPSILON,
                "expected {expected}, got {value}"
            );
        }
        other => panic!("expected float 10.0, got {other:?}"),
    }
}

#[test]
fn calling_an_unregistered_function_returns_an_error() {
    // Why: Test that calling a function that has not been registered produces a clear evaluation error.
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            IntegerDefinition::new_with_default("a number parameter", "missing(1, 2)"),
        )
        .finish();

    let data = build_parameter(frozen);

    let engine = ExpressionEngine::new();
    let error = engine
        .evaluate_parameters(&data)
        .expect_err("evaluation should fail for an undefined function");

    let message = error
        .first()
        .expect("at least one error should be reported");
    assert_eq!(
        message.translate_data().message_key().as_str(),
        "expression_engine_evaluation_function_not_defined"
    );
    assert_eq!(
        message
            .translate_data()
            .message_params()
            .get("function")
            .map(ShareableString::as_str),
        Some("missing")
    );
}

#[test]
fn calling_an_expression_as_a_function_returns_a_translatable_error() {
    let frozen = ParameterObjectDefinition::builder("Test Object")
        .with(
            parameter_key!("p_result"),
            IntegerDefinition::new_with_default("a number parameter", "(1 + 2)(3)"),
        )
        .finish();

    let data = build_parameter(frozen);
    let error = ExpressionEngine::new()
        .evaluate_parameters(&data)
        .expect_err("evaluation should fail when an expression is used as a function name");

    let message = error
        .first()
        .expect("at least one error should be reported");
    assert_eq!(
        message.translate_data().message_key().as_str(),
        "expression_engine_parser_function_name_required_expression"
    );
}

// Frozen with libm 0.2.16 on Rust 1.99.0 using the repository's baseline CPU flags.
// These are bit-pattern compatibility references, not approximate accuracy tests.
fn assert_expression_bits(expression: &str, expected: u64) {
    match evaluate_number_expression(expression) {
        ComputedItem::Float(actual) => assert_eq!(
            actual.to_bits(),
            expected,
            "expression `{expression}` changed its binary64 result: actual {:#018x}, expected {expected:#018x}",
            actual.to_bits(),
        ),
        other => panic!("expected expression `{expression}` to produce a float, got {other:?}"),
    }
}

#[test]
fn transcendental_reference_bits() {
    for (expression, expected) in [
        ("sin(0.123456789)", 0x3fbf_8653_acec_ff98),
        ("cos(0.987654321)", 0x3fe1_9eeb_7c6c_2316),
        ("tan(0.314159265)", 0x3fd4_cb7b_fadc_49e5),
        ("arcsin(0.333333333)", 0x3fd5_bfe3_4ea3_e1f7),
        ("arccos(0.666666667)", 0x3fea_ea08_d7fb_7a64),
        ("arctan(0.777777777)", 0x3fe5_2744_00ac_0c47),
        ("arctan2(0.123456789, -0.987654321)", 0x4008_234d_7f95_54c5),
        ("sinh(0.125)", 0x3fc0_0aac_cd00_d2f1),
        ("cosh(0.375)", 0x3ff1_2364_0f68_5b59),
        ("tanh(0.625)", 0x3fe1_bf47_eabb_8f96),
        ("sqrt(12.3456789)", 0x400c_1bf0_3f3f_15b6),
        ("log(1.23456789)", 0x3fca_f8e8_0d7f_7a34),
        ("log2(3.14159265)", 0x3ffa_6c87_3427_94e8),
        ("log10(98.7654321)", 0x3fff_e9e6_e628_2503),
        ("exp(0.123456789)", 0x3ff2_1a38_0e17_ced3),
        ("to_degrees(0.123456789)", 0x401c_4b51_77b8_2cf3),
        ("to_radians(12.3456789)", 0x3fcb_949c_64e7_8da9),
        ("sin(123456789.125)", 0x3fef_fefc_c2ee_7d37),
    ] {
        assert_expression_bits(expression, expected);
    }
}

#[test]
fn power_and_remainder_reference_bits() {
    for (expression, expected) in [
        ("1.23456789 ^ 0.333333333", 0x3ff1_2a0c_a8e7_f099),
        ("2.0 ^ -1074.0", 0x0000_0000_0000_0001),
        ("2.0 ^ -1075.0", 0x0000_0000_0000_0000),
        ("(-1.23456789) ^ 7.0", 0xc011_7c26_d5ec_7cfb),
        ("5.3 % 0.2", 0x3fb9_9999_9999_9978),
        ("-5.3 % 0.2", 0xbfb9_9999_9999_9978),
        ("5.3 % -0.2", 0x3fb9_9999_9999_9978),
        ("-4.0 % 2.0", 0x0000_0000_0000_0000),
        ("1e-323 % 5e-324", 0x0000_0000_0000_0000),
    ] {
        assert_expression_bits(expression, expected);
    }
}

#[test]
fn subnormal_and_underflow_reference_bits() {
    for (expression, expected) in [
        ("5e-324", 0x0000_0000_0000_0001),
        ("5e-324 + 5e-324", 0x0000_0000_0000_0002),
        // Half of the least subnormal ties to zero; 1.5 and 2.5 ULP both tie to 2.
        ("5e-324 * 0.5", 0x0000_0000_0000_0000),
        ("-5e-324 * 0.5", 0x0000_0000_0000_0000),
        ("1.5e-323 * 0.5", 0x0000_0000_0000_0002),
        ("2.5e-323 * 0.5", 0x0000_0000_0000_0002),
        ("2.2250738585072014e-308 / 2.0", 0x0008_0000_0000_0000),
        ("sin(5e-324)", 0x0000_0000_0000_0001),
        ("sqrt(5e-324)", 0x1e60_0000_0000_0000),
        ("exp(-745.0)", 0x0000_0000_0000_0001),
        ("exp(-746.0)", 0x0000_0000_0000_0000),
    ] {
        assert_expression_bits(expression, expected);
    }
}

#[test]
fn rounding_boundary_and_zero_reference_bits() {
    for (expression, expected) in [
        ("round(0.49999999999999994)", 0x0000_0000_0000_0000),
        ("round(0.5)", 0x3ff0_0000_0000_0000),
        ("round(0.5000000000000001)", 0x3ff0_0000_0000_0000),
        ("round(-0.49999999999999994)", 0x0000_0000_0000_0000),
        ("round(-0.5)", 0xbff0_0000_0000_0000),
        ("ceil(1.0000000000000002)", 0x4000_0000_0000_0000),
        ("floor(0.9999999999999999)", 0x0000_0000_0000_0000),
        // The engine canonicalizes every signed-zero result to positive zero.
        ("ceil(-0.25)", 0x0000_0000_0000_0000),
        ("-0.0", 0x0000_0000_0000_0000),
        ("sin(-0.0)", 0x0000_0000_0000_0000),
        ("-0.0 / 3.0", 0x0000_0000_0000_0000),
        ("9007199254740992.0 + 1.0", 0x4340_0000_0000_0000),
        ("9007199254740994.0 + 1.0", 0x4340_0000_0000_0002),
    ] {
        assert_expression_bits(expression, expected);
    }
}

#[test]
fn arithmetic_order_and_unfused_reference_bits() {
    for (expression, expected) in [
        (
            "(10000000000000000.0 + -10000000000000000.0) + 1.0",
            0x3ff0_0000_0000_0000,
        ),
        (
            "10000000000000000.0 + (-10000000000000000.0 + 1.0)",
            0x0000_0000_0000_0000,
        ),
        // A fused multiply-add would instead retain the exact product's -2^-104.
        (
            "1.0000000000000002 * 0.9999999999999998 - 1.0",
            0x0000_0000_0000_0000,
        ),
        (
            "1.0000000000000002 * (0.9999999999999998 - 1.0)",
            0xbcb0_0000_0000_0001,
        ),
        ("0.1 + 0.2 + 0.3", 0x3fe3_3333_3333_3334),
        ("0.1 + (0.2 + 0.3)", 0x3fe3_3333_3333_3333),
        ("2.0 ^ 3.0 ^ 2.0", 0x4080_0000_0000_0000),
        ("(2.0 ^ 3.0) ^ 2.0", 0x4050_0000_0000_0000),
        // Unit conversion intentionally fuses this formula; expression operators must not.
        (
            "-17.77777777777778 * (9.0 / 5.0) + 32.0",
            0x0000_0000_0000_0000,
        ),
    ] {
        assert_expression_bits(expression, expected);
    }
}
