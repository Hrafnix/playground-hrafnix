use datastore::definition::TabDefinition;

#[test]
fn test_definition_tab() {
    let def = TabDefinition::new("A tab");

    assert_eq!(def.description(), "A tab");
    assert_eq!(def.description_ref(), "A tab");
}

#[test]
fn test_definition_tab_equality() {
    let def_1 = TabDefinition::new("A tab");
    let def_2 = TabDefinition::new("A tab");
    let def_3 = TabDefinition::new("A new tab");

    assert_eq!(def_1, def_2);
    assert_ne!(def_1, def_3);
    assert_eq!(def_1, &def_2);
    assert_ne!(&def_1, def_3);
    assert_ne!(def_1, &def_3);
    assert_ne!(&def_1, &def_3);
}
