use datastore::prelude::*;

const CHOICES: &[ChoiceItemCompileTime] = &[
    const_choice_item!("small", "Small"),
    const_choice_item!("large", "Large"),
];
const CHOICE: ChoiceCompileTime = const_choice!("Size", CHOICES, default = "large");

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
