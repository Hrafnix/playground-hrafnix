use datastore::prelude::*;
use expression_engine::prelude::*;

#[test]
fn test_basic_data_choice_bare_identifier() {
    // Why: A choice value written as a bare identifier (e.g. `option_1`) should be
    // treated as the literal choice value rather than looked up as a variable.
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_choice"),
                ChoiceDefinition::new_with_default(
                    "A choice parameter",
                    vec![
                        ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                        ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                    ],
                    "option_1",
                ),
            )
            .finish(),
    );

    let data = ParameterObjectInputData::new(&frozen);

    let output = ExpressionEngine::new()
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    let choice = output.get("p_choice").unwrap();
    if let ComputedItem::Identifier(value) = choice {
        assert_eq!(value.as_ref(), "option_1");
    } else {
        panic!("expected identifier data");
    }
}

#[test]
fn test_basic_data_choice_quoted_string() {
    // Why: A choice value can also be written as a quoted string literal.
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_choice"),
                ChoiceDefinition::new_with_default(
                    "A choice parameter",
                    vec![
                        ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                        ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                    ],
                    "\"option_2\"",
                ),
            )
            .finish(),
    );

    let data = ParameterObjectInputData::new(&frozen);

    let output = ExpressionEngine::new()
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    let choice = output.get("p_choice").unwrap();
    if let ComputedItem::String(value) = choice {
        assert_eq!(value.as_ref(), "option_2");
    } else {
        panic!("expected string data");
    }
}

#[test]
fn test_basic_data_choice_invalid_identifier_errors() {
    // Why: A bare identifier that doesn't match any defined choice should error.
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_choice"),
                ChoiceDefinition::new_with_default(
                    "A choice parameter",
                    vec![
                        ChoiceItemDefinition::new(store_key!("option_1"), "Option 1"),
                        ChoiceItemDefinition::new(store_key!("option_2"), "Option 2"),
                    ],
                    "not_a_choice",
                ),
            )
            .finish(),
    );

    let data = ParameterObjectInputData::new(&frozen);

    let result = ExpressionEngine::new().evaluate_parameters(&data);
    assert!(result.is_err());
}

#[test]
fn test_basic_data_integer() {
    // Why: Test that a plain integer literal is evaluated to its own value.
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_number"),
                IntegerDefinition::new_with_default("A number parameter", "42"),
            )
            .finish(),
    );

    let data = ParameterObjectInputData::new(&frozen);

    let output = ExpressionEngine::new()
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    let number = output.get("p_number").unwrap();
    if let ComputedItem::Integer(number) = number {
        assert_eq!(*number, 42);
    } else {
        panic!("expected integer data");
    }
}

#[test]
fn test_basic_data_integer_expression() {
    // Why: Test that a simple integer addition expression is evaluated correctly.
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_number"),
                IntegerDefinition::new_with_default("A number parameter", "42 + 55"),
            )
            .finish(),
    );

    let data = ParameterObjectInputData::new(&frozen);

    let output = ExpressionEngine::new()
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    let number = output.get("p_number").unwrap();
    if let ComputedItem::Integer(number) = number {
        assert_eq!(*number, 97);
    } else {
        panic!("expected integer data");
    }
}

#[test]
fn test_basic_data_implicit_multiplication_before_parenthesis() {
    // Why: Test that a number directly followed by parentheses is treated as implicit multiplication.
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_number"),
                IntegerDefinition::new_with_default("A number parameter", "5(3 + 2)"),
            )
            .finish(),
    );

    let data = ParameterObjectInputData::new(&frozen);

    let output = ExpressionEngine::new()
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    let number = output.get("p_number").unwrap();
    if let ComputedItem::Integer(number) = number {
        assert_eq!(*number, 25);
    } else {
        panic!("expected integer data");
    }
}

#[test]
fn test_basic_data_scientific_notation_expression() {
    // Why: Test that numbers written in scientific notation are parsed and summed correctly.
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_number"),
                NumberDefinition::new_with_default("A number parameter", "1.5e2 + 2.5e1"),
            )
            .finish(),
    );

    let data = ParameterObjectInputData::new(&frozen);

    let output = ExpressionEngine::new()
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    let number = output.get("p_number").unwrap();
    if let ComputedItem::Float(number) = number {
        assert!((*number - 175.0).abs() < f64::EPSILON);
    } else {
        panic!("expected float data");
    }
}

#[test]
fn test_basic_global_data_integer_expression() {
    // Why: Test that a parameter expression can reference a global value evaluated beforehand.
    let global_frozen = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Test Object")
            .with(
                global_key!("g_number"),
                IntegerDefinition::new_with_default("A number parameter", "42"),
            )
            .finish(),
    );

    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_number"),
                IntegerDefinition::new_with_default("A number parameter", "g_number + 107"),
            )
            .finish(),
    );

    let global_data = GlobalObjectInputData::new(&global_frozen);
    let data = ParameterObjectInputData::new(&frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .evaluate_globals(&global_data)
        .expect("evaluation should succeed");

    let output = engine
        .evaluate_parameters(&data)
        .expect("evaluation should succeed");

    let number = output.get("p_number").unwrap();
    if let ComputedItem::Integer(number) = number {
        assert_eq!(*number, 149);
    } else {
        panic!("expected integer data");
    }
}
