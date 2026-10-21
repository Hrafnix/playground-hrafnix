use hrafnix_datastore::prelude::*;
use hrafnix_expression_engine::prelude::*;
use hrafnix_message::message::Message;
use hrafnix_shareable_string::ShareableString;
use std::collections::BTreeMap;

fn map_item() -> ItemFrozen {
    let entry = MapEntryFrozen::new_from_items(BTreeMap::from([(
        store_key!("field").into(),
        MapItemFrozen::Number(NumberFrozen::new(NumberDefinition::new("A map field"))),
    )]));
    let map = MapFrozen::new_from_items(
        "A map",
        BTreeMap::from([(store_key!("entry").into(), entry)]),
    )
    .expect("valid map schema");
    ItemFrozen::Map(map)
}

fn map_parameters() -> ParameterObjectInputData {
    let frozen = ParameterObjectFrozen::new_from_items(
        "Test Parameters",
        BTreeMap::from([(parameter_key!("p_map").into(), map_item())]),
    );
    ParameterObjectInputData::new(&frozen)
}

fn assert_message_parameter(message: &Message, key: &str, parameter: &str, value: &str) {
    assert_eq!(message.translate_data().message_key().as_str(), key);
    assert_eq!(
        message
            .translate_data()
            .message_params()
            .get(parameter)
            .map(ShareableString::as_str),
        Some(value)
    );
}

fn has_message_parameter(messages: &[Message], key: &str, parameter: &str, value: &str) -> bool {
    messages.iter().any(|message| {
        message.translate_data().message_key() == key
            && message
                .translate_data()
                .message_params()
                .get(parameter)
                .is_some_and(|actual| actual == value)
    })
}

#[test]
fn no_missing_requirements_when_everything_is_present() {
    // Why: Test that no missing requirements are reported when all referenced globals, parameters, and functions are provided.
    let global_frozen = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Test Globals")
            .with(
                global_key!("g_a"),
                IntegerDefinition::new_with_default("a global operand", "10"),
            )
            .finish(),
    );
    let global_data = GlobalObjectInputData::new(&global_frozen);

    let parameter_frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Parameters")
            .with(
                parameter_key!("p_b"),
                IntegerDefinition::new_with_default("a parameter operand", "5"),
            )
            .finish(),
    );
    let parameter_data = ParameterObjectInputData::new(&parameter_frozen);

    let mut engine = ExpressionEngine::new();
    engine
        .evaluate_globals(&global_data)
        .expect("globals should evaluate");

    let expression = ShareableString::from("sqrt(g_a) + p_b");

    let result =
        engine.check_missing_requirements(&Some(parameter_data), &None, &None, &expression);

    assert!(result.is_ok());
}

#[test]
fn reports_missing_global() {
    // Why: Test that a global item referenced by an expression but not supplied is reported as missing.
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("g_missing + 1");

    let errors = engine
        .check_missing_requirements(&None, &None, &None, &expression)
        .expect_err("missing global should be reported");

    assert_eq!(errors.len(), 1);
    assert_message_parameter(
        &errors[0],
        "expression_engine_evaluation_missing_required_global",
        "global",
        "g_missing",
    );
}

#[test]
fn reports_missing_parameter_when_none_supplied() {
    // Why: Test that a parameter referenced by an expression is reported as missing when no parameters are supplied.
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("p_missing * 2");

    let errors = engine
        .check_missing_requirements(&None, &None, &None, &expression)
        .expect_err("missing parameter should be reported");

    assert_eq!(errors.len(), 1);
    assert_message_parameter(
        &errors[0],
        "expression_engine_evaluation_missing_required_parameter",
        "parameter",
        "p_missing",
    );
}

#[test]
fn reports_missing_variable_when_none_supplied() {
    // Why: Test that a variable referenced by an expression is reported as missing when no variables are supplied.
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("v_missing - 1");

    let errors = engine
        .check_missing_requirements(&None, &None, &None, &expression)
        .expect_err("missing variable should be reported");

    assert_eq!(errors.len(), 1);
    assert_message_parameter(
        &errors[0],
        "expression_engine_evaluation_missing_required_variable",
        "variable",
        "v_missing",
    );
}

#[test]
fn reports_missing_function() {
    // Why: Test that a function call to an unregistered function is reported as a missing requirement.
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("not_a_real_function(1, 2)");

    let errors = engine
        .check_missing_requirements(&None, &None, &None, &expression)
        .expect_err("missing function should be reported");

    assert_eq!(errors.len(), 1);
    assert_message_parameter(
        &errors[0],
        "expression_engine_evaluation_missing_required_function",
        "function",
        "not_a_real_function",
    );
}

#[test]
fn reports_all_missing_requirement_kinds_at_once() {
    // Why: Test that all four kinds of missing requirements (global, parameter, variable, function) are reported together in a single expression.
    let engine = ExpressionEngine::new();
    let expression =
        ShareableString::from("g_missing + p_missing + v_missing + not_a_real_function(1)");

    let errors = engine
        .check_missing_requirements(&None, &None, &None, &expression)
        .expect_err("all missing requirement kinds should be reported");

    assert!(has_message_parameter(
        &errors,
        "expression_engine_evaluation_missing_required_global",
        "global",
        "g_missing"
    ));
    assert!(has_message_parameter(
        &errors,
        "expression_engine_evaluation_missing_required_parameter",
        "parameter",
        "p_missing"
    ));
    assert!(has_message_parameter(
        &errors,
        "expression_engine_evaluation_missing_required_variable",
        "variable",
        "v_missing"
    ));
    assert!(has_message_parameter(
        &errors,
        "expression_engine_evaluation_missing_required_function",
        "function",
        "not_a_real_function"
    ));
}

#[test]
fn provided_parameters_variables_and_globals_satisfy_requirements() {
    // Why: Test that supplying globals, parameters, and variables together satisfies all requirements of an expression referencing them.
    let engine = ExpressionEngine::new();

    let global_frozen = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Test Globals")
            .with(
                global_key!("g_a"),
                IntegerDefinition::new_with_default("a global operand", "1"),
            )
            .finish(),
    );
    let parameter_frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Parameters")
            .with(
                parameter_key!("p_b"),
                IntegerDefinition::new_with_default("a parameter operand", "2"),
            )
            .finish(),
    );
    let variable_frozen = VariableObjectFrozen::new(
        VariableObjectDefinition::builder("Test Variables")
            .with(
                variable_key!("v_c"),
                IntegerDefinition::new_with_default("a variable operand", "3"),
            )
            .finish(),
    );

    let globals = Some(GlobalObjectInputData::new(&global_frozen));
    let parameters = Some(ParameterObjectInputData::new(&parameter_frozen));
    let variables = Some(VariableObjectInputData::new(&variable_frozen));

    let expression = ShareableString::from("g_a + p_b + v_c");

    let result = engine.check_missing_requirements(&parameters, &variables, &globals, &expression);

    assert!(result.is_ok());
}

#[test]
fn invalid_expression_syntax_returns_an_error() {
    // Why: Test that a syntactically invalid expression returns an error rather than reporting missing requirements.
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("1 +");

    let result = engine.check_missing_requirements(&None, &None, &None, &expression);

    assert!(result.is_err());
}

#[test]
fn indexed_map_reference_is_satisfied_by_flattened_entries() {
    // Why: Map items are flattened to `name[entry][field]` keys, so `name` itself never exists.
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("p_map[entry][field] + 1.0");

    let result =
        engine.check_missing_requirements(&Some(map_parameters()), &None, &None, &expression);

    assert!(result.is_ok());
}

#[test]
fn bare_map_reference_is_satisfied_by_the_parent_name() {
    let engine = ExpressionEngine::new();
    let expression = ShareableString::from("p_map + 1.0");

    assert!(
        engine
            .check_missing_requirements(&Some(map_parameters()), &None, &None, &expression,)
            .is_ok()
    );
}

#[test]
fn missing_indexed_reference_is_reported() {
    let engine = ExpressionEngine::new();
    let frozen = ParameterObjectFrozen::new(
        ParameterObjectDefinition::builder("Test Parameters")
            .with(
                parameter_key!("p_other"),
                IntegerDefinition::new_with_default("Another parameter", "1"),
            )
            .finish(),
    );
    let parameters = ParameterObjectInputData::new(&frozen);
    let expression = ShareableString::from("p_table[0][a]");

    let errors = engine
        .check_missing_requirements(&Some(parameters), &None, &None, &expression)
        .expect_err("missing indexed parameter should be reported");

    assert_eq!(errors.len(), 1);
    assert_message_parameter(
        &errors[0],
        "expression_engine_evaluation_missing_required_parameter",
        "parameter",
        "p_table",
    );
}

#[test]
fn map_requirements_check_only_the_parent_name() {
    let engine = ExpressionEngine::new();
    for expression in [
        "p_map[other][field]",
        "p_map[entry][other]",
        "p_map[entry][field] + p_map[other][field]",
        "p_map[entry][field] + p_map",
        "p_map + p_map[entry][field]",
        "p_map[1 / 0][field]",
    ] {
        assert!(
            engine
                .check_missing_requirements(
                    &Some(map_parameters()),
                    &None,
                    &None,
                    &ShareableString::from(expression),
                )
                .is_ok(),
            "{expression}"
        );
    }
}

#[test]
fn requirements_inside_index_expressions_are_still_checked() {
    let engine = ExpressionEngine::new();
    let errors = engine
        .check_missing_requirements(
            &Some(map_parameters()),
            &None,
            &None,
            &ShareableString::from(
                "p_map[unknown_function(g_missing + p_missing + v_missing)][field]",
            ),
        )
        .expect_err("index expression dependencies are missing");
    assert_eq!(errors.len(), 4);
    for (kind, name) in [
        ("global", "g_missing"),
        ("parameter", "p_missing"),
        ("variable", "v_missing"),
        ("function", "unknown_function"),
    ] {
        assert!(has_message_parameter(
            &errors,
            &format!("expression_engine_evaluation_missing_required_{kind}"),
            kind,
            name,
        ));
    }
}

#[test]
fn bare_index_identifiers_are_checked_and_deduplicated() {
    let engine = ExpressionEngine::new();
    let errors = engine
        .check_missing_requirements(
            &Some(map_parameters()),
            &None,
            &None,
            &ShareableString::from("p_map[g_index][p_index] + p_map[v_index][g_index]"),
        )
        .expect_err("bare index dependencies are missing");

    assert_eq!(errors.len(), 3);
    for (kind, name) in [
        ("global", "g_index"),
        ("parameter", "p_index"),
        ("variable", "v_index"),
    ] {
        assert!(has_message_parameter(
            &errors,
            &format!("expression_engine_evaluation_missing_required_{kind}"),
            kind,
            name,
        ));
    }
}

#[test]
fn supplied_bare_index_dependencies_are_not_evaluated() {
    let engine = ExpressionEngine::new();
    let globals = GlobalObjectFrozen::new(
        GlobalObjectDefinition::builder("Index dependencies")
            .with(
                global_key!("g_index"),
                IntegerDefinition::new_with_default("Index", "1 / 0"),
            )
            .finish(),
    );
    let parameters = ParameterObjectFrozen::new_from_items(
        "Index dependencies",
        BTreeMap::from([
            (parameter_key!("p_map").into(), map_item()),
            (
                parameter_key!("p_index").into(),
                ItemFrozen::Integer(IntegerFrozen::new(IntegerDefinition::new_with_default(
                    "Index",
                    "unknown_function()",
                ))),
            ),
        ]),
    );
    let variables = VariableObjectFrozen::new(
        VariableObjectDefinition::builder("Index dependencies")
            .with(
                variable_key!("v_index"),
                IntegerDefinition::new_with_default("Index", "g_missing"),
            )
            .finish(),
    );

    assert!(
        engine
            .check_missing_requirements(
                &Some(ParameterObjectInputData::new(&parameters)),
                &Some(VariableObjectInputData::new(&variables)),
                &Some(GlobalObjectInputData::new(&globals)),
                &ShareableString::from("p_map[g_index][p_index] + p_map[v_index][field]"),
            )
            .is_ok()
    );
}

#[test]
fn unprefixed_and_quoted_index_names_are_literals() {
    let engine = ExpressionEngine::new();
    for expression in [
        "p_map[entry][field]",
        "p_map[\"g_index\"][\"p_index\"]",
        "p_map[\"v_index\"][field]",
    ] {
        assert!(
            engine
                .check_missing_requirements(
                    &Some(map_parameters()),
                    &None,
                    &None,
                    &ShareableString::from(expression),
                )
                .is_ok(),
            "{expression}"
        );
    }
}

#[test]
fn parent_name_checks_apply_to_globals_and_variables() {
    let engine = ExpressionEngine::new();
    let globals = GlobalObjectFrozen::new_from_items(
        "Maps",
        BTreeMap::from([(global_key!("g_map").into(), map_item())]),
    );
    let variables = VariableObjectFrozen::new_from_items(
        "Maps",
        BTreeMap::from([(variable_key!("v_map").into(), map_item())]),
    );
    let globals = Some(GlobalObjectInputData::new(&globals));
    let variables = Some(VariableObjectInputData::new(&variables));
    assert!(
        engine
            .check_missing_requirements(
                &None,
                &variables,
                &globals,
                &ShareableString::from("g_map[other][field] + v_map[entry][other] + g_map + v_map"),
            )
            .is_ok()
    );

    let errors = engine
        .check_missing_requirements(
            &None,
            &variables,
            &globals,
            &ShareableString::from("g_missing[other][field] + v_missing[entry][other]"),
        )
        .expect_err("both parent names are missing");
    assert_eq!(errors.len(), 2);
    assert!(has_message_parameter(
        &errors,
        "expression_engine_evaluation_missing_required_global",
        "global",
        "g_missing",
    ));
    assert!(has_message_parameter(
        &errors,
        "expression_engine_evaluation_missing_required_variable",
        "variable",
        "v_missing",
    ));
}

#[test]
fn nested_index_operands_and_base_tables_are_supported() {
    let engine = ExpressionEngine::new();
    let selector = MapFrozen::new_from_items(
        "Selector",
        BTreeMap::from([(
            store_key!("entry").into(),
            MapEntryFrozen::new_from_items(BTreeMap::from([(
                store_key!("field").into(),
                MapItemFrozen::String(StringFrozen::new(StringDefinition::new_with_default(
                    "Entry",
                    "\"entry\"",
                ))),
            )])),
        )]),
    )
    .unwrap();
    let globals = GlobalObjectFrozen::new_from_items(
        "Index operands",
        BTreeMap::from([(global_key!("g_selector").into(), ItemFrozen::Map(selector))]),
    );
    let parameters = ParameterObjectFrozen::new_from_items(
        "Targets",
        BTreeMap::from([
            (parameter_key!("p_map").into(), map_item()),
            (
                parameter_key!("p_table").into(),
                ItemFrozen::Table(TableFrozen::new_from_rows(
                    TableDefinition::new(
                        "Table",
                        vec![(store_key!("field"), NumberDefinition::new("Field"))],
                    ),
                    vec![vec!["1.0".into()]],
                )),
            ),
        ]),
    );
    for expression in [
        "p_map[g_selector[entry][field]][field]",
        "p_table[0][field]",
        "p_table[0 + 0][field]",
    ] {
        assert!(
            engine
                .check_missing_requirements(
                    &Some(ParameterObjectInputData::new(&parameters)),
                    &None,
                    &Some(GlobalObjectInputData::new(&globals)),
                    &ShareableString::from(expression),
                )
                .is_ok(),
            "{expression}"
        );
    }
}

#[test]
fn checking_requirements_does_not_invoke_index_functions() {
    use std::sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    };
    let called = Arc::new(AtomicBool::new(false));
    let callback_called = Arc::clone(&called);
    let mut engine = ExpressionEngine::new();
    engine
        .register_function(FunctionDefinition::new(
            store_key!("entry_for"),
            "Selects a map entry",
            ArgumentCount::Exact { count: 1 },
            move |_| {
                callback_called.store(true, Ordering::Relaxed);
                Ok(ComputedItem::String("entry".into()))
            },
        ))
        .unwrap();
    let parameters = ParameterObjectFrozen::new_from_items(
        "Index operands",
        BTreeMap::from([
            (parameter_key!("p_map").into(), map_item()),
            (
                parameter_key!("p_choice").into(),
                ItemFrozen::Integer(IntegerFrozen::new(IntegerDefinition::new_with_default(
                    "Index", "0",
                ))),
            ),
        ]),
    );
    let parameters = Some(ParameterObjectInputData::new(&parameters));
    assert!(
        engine
            .check_missing_requirements(
                &parameters,
                &None,
                &None,
                &ShareableString::from("p_map[entry_for(p_choice + 1)][field]"),
            )
            .is_ok()
    );
    assert!(
        engine
            .check_missing_requirements(
                &parameters,
                &None,
                &None,
                &ShareableString::from("p_map[entry_for(p_choice + 2)][field]"),
            )
            .is_ok()
    );
    assert!(!called.load(Ordering::Relaxed));
}
