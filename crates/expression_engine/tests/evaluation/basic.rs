use datastore::definition::{
    GlobalObjectDefinition, IntegerDefinition, NumberDefinition, ParameterObjectDefinition,
};
use datastore::frozen::{GlobalObjectFrozen, ParameterObjectFrozen};
use datastore::{global_key, parameter_key};
use expression_engine::engine::ExpressionEngine;
use expression_engine::{ComputedItem, GlobalObjectInputData, ParameterObjectInputData};

#[test]
fn test_basic_data_integer() {
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_number"),
                IntegerDefinition::new_with_default("A number parameter", "42"),
            )
            .finish(),
    );

    let data = ParameterObjectInputData::new(frozen);

    let output = ExpressionEngine::new()
        .evaluate_parameters(data)
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
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_number"),
                IntegerDefinition::new_with_default("A number parameter", "42 + 55"),
            )
            .finish(),
    );

    let data = ParameterObjectInputData::new(frozen);

    let output = ExpressionEngine::new()
        .evaluate_parameters(data)
        .expect("evaluation should succeed");

    let number = output.get("p_number").unwrap();
    if let ComputedItem::Integer(number) = number {
        assert_eq!(*number, 97);
    } else {
        panic!("expected integer data");
    }
}

#[test]
fn test_basic_data_scientific_notation_expression() {
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Object")
            .with(
                parameter_key!("p_number"),
                NumberDefinition::new_with_default("A number parameter", "1.5e2 + 2.5e1"),
            )
            .finish(),
    );

    let data = ParameterObjectInputData::new(frozen);

    let output = ExpressionEngine::new()
        .evaluate_parameters(data)
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

    let global_data = GlobalObjectInputData::new(global_frozen);
    let data = ParameterObjectInputData::new(frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .evaluate_globals(global_data)
        .expect("evaluation should succeed");

    let output = engine
        .evaluate_parameters(data)
        .expect("evaluation should succeed");

    let number = output.get("p_number").unwrap();
    if let ComputedItem::Integer(number) = number {
        assert_eq!(*number, 149);
    } else {
        panic!("expected integer data");
    }
}
