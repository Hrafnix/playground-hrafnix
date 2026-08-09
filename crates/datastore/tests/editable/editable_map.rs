use datastore::prelude::*;
use std::collections::BTreeMap;
use units::{UnitFamilyId, UnitId};

#[test]
fn test_editable_map_entry_round_trip() {
    // Why: Editable map entries should thaw from frozen, allow edits to their items, and freeze
    // back to an equivalent frozen entry reflecting those edits.
    let item_type: BTreeMap<StoreKey, MapItemDefinition> = vec![
        (
            store_key!("field1").into(),
            MapItemDefinition::String(StringDefinition::new("Field 1")),
        ),
        (
            store_key!("field2").into(),
            MapItemDefinition::Number(NumberDefinition::new_with_default("Field 2", "1")),
        ),
    ]
    .into_iter()
    .collect();
    let frozen_entry = MapEntryFrozen::new(&item_type);

    let mut editable_entry = frozen_entry.thaw();
    assert_eq!(editable_entry.get_string("field1").unwrap().value(), "");

    if let MapItemEditable::String(string_editable) =
        editable_entry.get_mut("field1").expect("field1 item")
    {
        string_editable.set("hello");
    }

    assert_eq!(
        editable_entry.get_string("field1").unwrap().value(),
        "hello"
    );

    let frozen_entry_2 = editable_entry.freeze();
    assert_eq!(frozen_entry_2.definition(), frozen_entry.definition());
    assert_eq!(
        frozen_entry_2.get_string("field1").unwrap().value(),
        "hello"
    );
    assert_ne!(frozen_entry_2.hash(), frozen_entry.hash());
}

#[test]
fn test_editable_map_round_trip() {
    // Why: Editable maps should thaw from frozen, expose their entries, and freeze back to an
    // equivalent frozen map.
    let entry_1 = MapEntryFrozen::new_from_items(
        vec![(
            store_key!("field1").into(),
            MapItemFrozen::String(StringFrozen::new(StringDefinition::new("Field 1"))),
        )]
        .into_iter()
        .collect(),
    );
    let entry_2 = MapEntryFrozen::new_from_items(
        vec![(
            store_key!("field1").into(),
            MapItemFrozen::String(StringFrozen::new(StringDefinition::new("Field 1"))),
        )]
        .into_iter()
        .collect(),
    );

    let items: BTreeMap<StoreKey, MapEntryFrozen> = vec![
        (store_key!("row1").into(), entry_1),
        (store_key!("row2").into(), entry_2),
    ]
    .into_iter()
    .collect();

    let frozen_map = MapFrozen::new_from_items("A map", items).expect("valid schema");
    let editable_map = frozen_map.thaw();

    assert_eq!(editable_map.count(), 2);
    assert_eq!(editable_map.definition().description().as_ref(), "A map");
    assert!(editable_map.get("row1").is_some());
    assert!(editable_map.get("row2").is_some());
    assert!(editable_map.get("row3").is_none());

    let frozen_map_2 = editable_map.freeze();
    assert_eq!(frozen_map_2.count(), 2);
    assert_eq!(frozen_map_2.hash(), frozen_map.hash());
}

#[test]
fn test_editable_map_get_mut() {
    // Why: Editable map should allow mutable access to its entries for in-place edits.
    let item_type: BTreeMap<StoreKey, MapItemDefinition> = vec![(
        store_key!("field1").into(),
        MapItemDefinition::String(StringDefinition::new("Field 1")),
    )]
    .into_iter()
    .collect();
    let entry = MapEntryFrozen::new(&item_type);
    let items: BTreeMap<StoreKey, MapEntryFrozen> = vec![(store_key!("row1").into(), entry)]
        .into_iter()
        .collect();
    let frozen_map = MapFrozen::new_from_items("A map", items).expect("valid schema");

    let mut editable_map = frozen_map.thaw();
    let entry_mut = editable_map.get_mut("row1").expect("row1 entry");
    assert_eq!(entry_mut.get_string("field1").unwrap().value(), "");

    if let MapItemEditable::String(string_editable) =
        entry_mut.get_mut("field1").expect("field1 item")
    {
        string_editable.set("edited");
    }

    let frozen_map_2 = editable_map.freeze();
    assert_eq!(
        frozen_map_2
            .get("row1")
            .unwrap()
            .get_string("field1")
            .unwrap()
            .value(),
        "edited"
    );
    assert_ne!(frozen_map_2.hash(), frozen_map.hash());
}

#[test]
fn test_editable_map_unit_round_trip() {
    let item_type: BTreeMap<StoreKey, MapItemDefinition> = vec![(
        store_key!("unit").into(),
        MapItemDefinition::Unit(UnitDefinition::new_with_default(
            "Length unit",
            UnitFamilyId::Length,
            UnitId::Length_Meter.string_id().as_str(),
        )),
    )]
    .into_iter()
    .collect();
    let frozen = MapEntryFrozen::new(&item_type);
    let mut editable = frozen.thaw();

    editable
        .get_mut("unit")
        .expect("unit item")
        .get_mut_unit()
        .expect("unit value")
        .set(UnitId::Length_Foot.string_id().as_str());

    let refrozen = editable.freeze();
    assert_eq!(
        refrozen.get_unit("unit").map(UnitFrozen::value),
        Some(UnitId::Length_Foot.string_id().into())
    );
    assert_ne!(refrozen.hash(), frozen.hash());
}
