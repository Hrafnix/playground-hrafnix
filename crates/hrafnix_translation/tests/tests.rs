//! Integration tests for the built-in translation catalog.

use hrafnix_shareable_string::{ShareableString, SharedStringStore};
use hrafnix_translation::generate_translation_map;
use std::collections::HashMap;

const LANGUAGES: [&str; 7] = ["en", "zh", "de", "es", "fr", "ja", "ko"];

const TRANSLATION_KEYS: [&str; 182] = [
    "files_io_replaced_sync_failed",
    "files_io_error",
    "files_io_not_found",
    "files_io_permission_denied",
    "files_io_not_a_file",
    "files_io_connection_refused",
    "files_io_connection_reset",
    "files_io_host_unreachable",
    "files_io_network_unreachable",
    "files_io_connection_aborted",
    "files_io_not_connected",
    "files_io_address_in_use",
    "files_io_address_not_available",
    "files_io_network_down",
    "files_io_broken_pipe",
    "files_io_already_exists",
    "files_io_would_block",
    "files_io_not_a_directory",
    "files_io_directory_not_empty",
    "files_io_read_only_filesystem",
    "files_io_stale_network_file_handle",
    "files_io_invalid_input",
    "files_io_invalid_data",
    "files_io_timed_out",
    "files_io_write_zero",
    "files_io_storage_full",
    "files_io_not_seekable",
    "files_io_quota_exceeded",
    "files_io_file_too_large",
    "files_io_resource_busy",
    "files_io_executable_file_busy",
    "files_io_deadlock",
    "files_io_crosses_devices",
    "files_io_too_many_links",
    "files_io_invalid_filename",
    "files_io_argument_list_too_long",
    "files_io_interrupted",
    "files_io_unsupported",
    "files_io_unexpected_eof",
    "files_io_out_of_memory",
    "files_io_other",
    "files_json_syntax",
    "files_json_unexpected_eof",
    "files_json_data",
    "files_json_serialization",
    "files_invalid_utf8",
    "files_incomplete_utf8",
    "files_missing_file_version",
    "files_duplicate_file_version",
    "files_invalid_file_version",
    "files_unsupported_file_version",
    "files_expected_root",
    "files_duplicate_root",
    "files_unknown_top_level_key",
    "datastore_key_empty",
    "datastore_key_invalid_character",
    "datastore_key_invalid_prefix",
    "datastore_key_conflict",
    "datastore_key_reserved",
    "datastore_key_not_found",
    "datastore_index_not_found",
    "datastore_schema_mismatch",
    "datastore_missing_schema",
    "datastore_map_value_set_not_supported",
    "datastore_tab_or_separator_value_set_not_supported",
    "expression_engine_lexer_invalid_character",
    "expression_engine_lexer_invalid_number",
    "expression_engine_lexer_invalid_operator",
    "expression_engine_lexer_invalid_string",
    "expression_engine_lexer_unterminated_string_literal",
    "expression_engine_evaluation_function_name_empty",
    "expression_engine_evaluation_missing_required_global",
    "expression_engine_evaluation_missing_required_parameter",
    "expression_engine_evaluation_missing_required_variable",
    "expression_engine_evaluation_missing_required_function",
    "expression_engine_parser_expected_end_of_input",
    "expression_engine_parser_expected_expression",
    "expression_engine_parser_expected_operator",
    "expression_engine_parser_expected_specific_operator",
    "expression_engine_parser_function_name_required_number",
    "expression_engine_parser_function_name_required_text",
    "expression_engine_parser_function_name_required_expression",
    "expression_engine_parser_invalid_prefix_operator",
    "expression_engine_translator_binary_missing_left_operand",
    "expression_engine_translator_binary_missing_right_operand",
    "expression_engine_translator_unary_missing_operand",
    "expression_engine_translator_unary_plus_missing_operand",
    "expression_engine_translator_index_missing_target",
    "expression_engine_translator_index_missing_index",
    "expression_engine_translator_index_target_must_be_identifier",
    "expression_engine_translator_invalid_numeric_literal",
    "expression_engine_translator_unsupported_operator",
    "expression_engine_evaluation_floating_point_not_finite",
    "expression_engine_evaluation_variable_not_found",
    "expression_engine_evaluation_integer_overflow",
    "expression_engine_evaluation_invalid_unary_operation",
    "expression_engine_evaluation_unsupported_operator",
    "expression_engine_evaluation_division_by_zero",
    "expression_engine_evaluation_modulus_by_zero",
    "expression_engine_evaluation_invalid_integer_exponent",
    "expression_engine_evaluation_function_not_defined",
    "expression_engine_evaluation_function_wrong_argument_count_exact",
    "expression_engine_evaluation_function_wrong_argument_count_minimum",
    "expression_engine_evaluation_function_wrong_argument_count_maximum",
    "expression_engine_evaluation_function_wrong_argument_count_range",
    "expression_engine_evaluation_invalid_index_count",
    "expression_engine_evaluation_table_index_count",
    "expression_engine_evaluation_missing_first_index",
    "expression_engine_evaluation_missing_second_index",
    "expression_engine_evaluation_expected_table_for_indexing",
    "expression_engine_evaluation_table_row_index_out_of_bounds",
    "expression_engine_evaluation_expected_table_row_index",
    "expression_engine_evaluation_table_field_not_found",
    "expression_engine_evaluation_table_column_index_out_of_bounds",
    "expression_engine_evaluation_expected_table_field_index",
    "expression_engine_evaluation_invalid_choice",
    "expression_engine_evaluation_invalid_unit_id",
    "expression_engine_evaluation_invalid_unit_for_family",
    "expression_engine_evaluation_expected_unit_value",
    "expression_engine_evaluation_expected_definition_value",
    "expression_engine_evaluation_value_below_minimum",
    "expression_engine_evaluation_value_above_maximum",
    "expression_engine_evaluation_unit_conversion_failed",
    "expression_engine_evaluation_incompatible_units",
    "expression_engine_evaluation_offset_temperature_arithmetic",
    "expression_engine_evaluation_expected_number_with_units_definition",
    "expression_engine_evaluation_unknown_unit",
    "expression_engine_evaluation_table_column_count_mismatch",
    "expression_engine_evaluation_table_column_requirement_failed_increasing",
    "expression_engine_evaluation_table_column_requirement_failed_constant_or_increasing",
    "expression_engine_evaluation_table_column_requirement_failed_decreasing",
    "expression_engine_evaluation_table_column_requirement_failed_constant_or_decreasing",
    "expression_engine_evaluation_table_column_requirement_failed_constant",
    "expression_engine_evaluation_table_column_requirement_missing_cell",
    "expression_engine_evaluation_table_missing_column_definition",
    "expression_engine_evaluation_table_value_below_minimum",
    "expression_engine_evaluation_table_value_above_maximum",
    "expression_engine_evaluation_expected_table_parameter",
    "expression_engine_evaluation_table_cell_missing_column_definition",
    "expression_engine_evaluation_expected_table_cell_number",
    "expression_engine_evaluation_table_unit_count_mismatch",
    "expression_engine_evaluation_table_unit_conversion_failed",
    "expression_engine_evaluation_table_cell_missing_unit",
    "expression_engine_evaluation_sum_requires_numeric_arguments",
    "expression_engine_evaluation_add_requires_numeric_arguments",
    "expression_engine_function_argument_must_be_finite",
    "expression_engine_function_argument_out_of_integer_range",
    "expression_engine_function_argument_integer_conversion_failed",
    "expression_engine_function_argument_float_precision_loss",
    "expression_engine_function_argument_float_conversion_failed",
    "expression_engine_function_missing_expected_argument",
    "expression_engine_function_argument_must_be_float",
    "expression_engine_function_arguments_mixed_numeric_types",
    "expression_engine_function_argument_must_be_number",
    "expression_engine_function_clamp_minimum_exceeds_maximum",
    "expression_engine_function_incompatible_units",
    "expression_engine_function_length_result_too_large",
    "expression_engine_function_argument_must_be_string",
    "expression_engine_function_if_condition_must_be_boolean",
    "expression_engine_evaluation_custom_function_failed",
    "hrafnix_about",
    "hrafnix_language",
    "hrafnix_new",
    "hrafnix_format",
    "hrafnix_simulate",
    "hrafnix_usage",
    "hrafnix_commands",
    "hrafnix_options",
    "hrafnix_arguments",
    "hrafnix_help",
    "hrafnix_version",
    "hrafnix_supported_languages",
    "hrafnix_error",
    "hrafnix_error_invalid_input",
    "hrafnix_error_unrecognized_subcommand",
    "hrafnix_error_unexpected_argument",
    "hrafnix_error_value_required",
    "hrafnix_error_invalid_value",
    "hrafnix_tip",
    "hrafnix_error_similar_subcommand",
    "hrafnix_error_similar_argument",
    "hrafnix_more_information",
];

#[test]
fn generated_map_contains_every_translation_key() {
    let map = generate_translation_map(&SharedStringStore::new());
    assert_eq!(map.len(), TRANSLATION_KEYS.len());

    for key in TRANSLATION_KEYS {
        assert!(map.contains_key(key), "missing translation key {key}");

        for language in LANGUAGES {
            assert!(
                map.languages_for_key(key)
                    .is_some_and(|languages| { languages.iter().any(|value| value == language) }),
                "missing {language} translation for {key}"
            );
            assert!(
                map.get_translation(key, language, None)
                    .is_some_and(|value| !value.as_str().is_empty()),
                "empty {language} translation for {key}"
            );
        }
    }
}

#[test]
fn generated_map_selects_each_supported_language() {
    let map = generate_translation_map(&SharedStringStore::new());
    let expected = [
        ("en", "Invalid key: key cannot be empty"),
        ("zh", "无效键：键不能为空"),
        (
            "de",
            "Ungültiger Schlüssel: Der Schlüssel darf nicht leer sein",
        ),
        ("es", "Clave no válida: la clave no puede estar vacía"),
        ("fr", "Clé non valide : la clé ne peut pas être vide"),
        ("ja", "無効なキー: キーを空にすることはできません"),
        ("ko", "잘못된 키: 키는 비워둘 수 없습니다"),
    ];

    assert_eq!(
        expected.map(|(language, _)| language),
        LANGUAGES,
        "the locale fixture must cover every supported language"
    );
    for (language, expected_translation) in expected {
        let translation = map.get_translation("datastore_key_empty", language, None);
        assert_eq!(
            translation.as_ref().map(ShareableString::as_str),
            Some(expected_translation),
            "incorrect {language} translation"
        );
    }
}

#[test]
fn generated_map_substitutes_parameters_in_localized_messages() {
    let store = SharedStringStore::new();
    let map = generate_translation_map(&store);
    let params = HashMap::from([
        (store.launder("function"), store.launder("sum")),
        (store.launder("expected"), store.launder("2")),
        (store.launder("actual"), store.launder("3")),
    ]);

    let translation = map.get_translation(
        "expression_engine_evaluation_function_wrong_argument_count_exact",
        "de",
        Some(&params),
    );

    assert_eq!(
        translation.as_ref().map(ShareableString::as_str),
        Some("Die Funktion „sum“ erfordert genau 2 Argumente, erhalten wurden 3.")
    );
}

#[test]
fn generated_map_falls_back_to_english_for_unknown_language() {
    let map = generate_translation_map(&SharedStringStore::new());

    assert_eq!(map.get_fallback_language().as_ref(), "en");
    assert_eq!(
        map.get_translation("datastore_key_not_found", "unknown", None)
            .as_ref()
            .map(ShareableString::as_str),
        Some("Key not found")
    );
}

#[test]
fn generated_map_returns_none_for_unknown_key() {
    let map = generate_translation_map(&SharedStringStore::new());

    assert!(
        map.get_translation("translation_key_that_does_not_exist", "en", None)
            .is_none()
    );
}
