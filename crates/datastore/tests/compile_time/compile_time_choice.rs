use datastore::prelude::*;

const CHOICES: &[ChoiceItemCompileTime] = &[
    choice_item_compile_time!("small", "Small"),
    choice_item_compile_time!("large", "Large"),
];
const CHOICE: ChoiceCompileTime = choice_compile_time!("Size", CHOICES, default = "large");

#[test]
fn choice_compile_time_preserves_order_and_default() {
    assert!(CHOICE.contains("small"));
    assert!(!CHOICE.contains("medium"));
    assert_eq!(CHOICE.default_value(), "large");
    assert_eq!(
        CHOICE.ids().map(|key| key.to_string()).collect::<Vec<_>>(),
        ["small", "large"]
    );

    let definition = CHOICE.into_definition();
    assert_eq!(definition.default_value(), "large");
    assert_eq!(
        definition
            .choices()
            .iter()
            .map(|choice| choice.id().to_string())
            .collect::<Vec<_>>(),
        ["small", "large"]
    );
}

#[test]
#[should_panic(expected = "ChoiceCompileTime choice ids must be unique")]
fn choice_compile_time_rejects_duplicate_ids() {
    const DUPLICATES: &[ChoiceItemCompileTime] = &[
        choice_item_compile_time!("duplicate", "First"),
        choice_item_compile_time!("duplicate", "Second"),
    ];
    let _ = ChoiceCompileTime::__new(std::hint::black_box("Duplicates"), DUPLICATES);
}
