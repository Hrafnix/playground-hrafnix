//! Evaluate a simple expression stored in parameter data.

#![allow(clippy::print_stdout)]

use datastore::prelude::*;
use expression_engine::prelude::*;
use std::process::ExitCode;

fn main() -> ExitCode {
    let store = SharedStringStore::new();
    let translations = translation::generate_translation_map(&store);
    let definition = ParameterObjectDefinition::builder("Example Parameters")
        .with(
            parameter_key!("p_answer"),
            IntegerDefinition::new_with_default("The expression to evaluate", "6 * 7"),
        )
        .finish();
    let frozen = ParameterObjectFrozen::new(definition);
    let input = ParameterObjectInputData::new(&frozen);

    match ExpressionEngine::new().evaluate_parameters(&input) {
        Ok(output) => {
            if let Some(value) = output.get("p_answer") {
                println!("6 * 7 = {value}");
                ExitCode::SUCCESS
            } else {
                eprintln!("The evaluated output did not contain `p_answer`.");
                ExitCode::FAILURE
            }
        }
        Err(errors) => {
            for error in errors {
                let rendered = error
                    .translated_message(&translations, "en")
                    .unwrap_or_else(|| error.translate_data().message_key().clone());
                eprintln!("{rendered}");
            }
            ExitCode::FAILURE
        }
    }
}
