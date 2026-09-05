use datastore::definition::SeparatorDefinition;

#[test]
fn test_definition_separator() {
    let def = SeparatorDefinition::new("A separator");

    assert_eq!(def.description(), "A separator");
    assert_eq!(def.description_ref(), "A separator");
}

#[test]
fn test_definition_separator_equality() {
    let def_1 = SeparatorDefinition::new("A separator");
    let def_2 = SeparatorDefinition::new("A separator");
    let def_3 = SeparatorDefinition::new("A new separator");

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(def_1, &def_2);
    assert_ne!(&def_1, def_3);
    assert_ne!(def_1, &def_3);
    assert_ne!(&def_1, &def_3);
}
