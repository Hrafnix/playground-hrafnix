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
}
