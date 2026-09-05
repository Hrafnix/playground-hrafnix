use datastore::prelude::*;

const CHOICES: &[ChoiceItemCompileTime] = &[
    const_choice_item!("small", "Small"),
    const_choice_item!("large", "Large"),
];
const CHOICE: ChoiceCompileTime = const_choice!("Size", CHOICES, default = "large");
const CHOICE_WITHOUT_DEFAULT: ChoiceCompileTime = const_choice!("Size", CHOICES);

#[test]
fn choice_compile_time_preserves_order_and_default() {
    assert_eq!(CHOICES[0].id(), store_key!("small"));
    assert_eq!(CHOICES[0].description(), "Small");
    assert_eq!(CHOICES[0].into_definition().description(), "Small");
    assert_eq!(CHOICE.choices(), CHOICES);
    assert_eq!(CHOICE.description(), "Size");
    assert!(CHOICE.contains("small"));
    assert!(!CHOICE.contains("medium"));
    assert_eq!(CHOICE.default_value(), "large");
    assert_eq!(
        CHOICE.ids().map(|key| key.to_string()).collect::<Vec<_>>(),
        ["small", "large"]
    );
    assert_eq!(
        CHOICE.descriptions().collect::<Vec<_>>(),
        ["Small", "Large"]
    );
    assert_eq!(CHOICE_WITHOUT_DEFAULT.default_value(), "");
    assert_eq!(CHOICE_WITHOUT_DEFAULT.into_definition().default_value(), "");

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
