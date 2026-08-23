use crate::editable::{ItemEditable, MapEntryEditable, MapItemEditable};
use crate::traits::ObjectEditable;
use message::message::{Message, MessageCategory};
use shareable_string::ShareableString;

/// Helper function to set the value of an editable object by key.
///
/// # Errors
///
/// Returns a `Message` if the key does not exist or if the value type does not match the expected type.
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn editable_set_value<
    T: ObjectEditable,
    S1: Into<ShareableString>,
    S2: Into<ShareableString>,
>(
    obj: &mut T,
    key: S1,
    value: S2,
) -> Result<(), Message> {
    let key = key.into();
    let value = value.into();

    let item = obj
        .get_mut(&key)
        .ok_or_else(|| Message::error(MessageCategory::Datastore, "datastore_key_not_found"))?;

    match item {
        ItemEditable::Boolean(boolean) => {
            boolean.set(value);
        }
        ItemEditable::Choice(choice) => {
            choice.set(value);
        }
        ItemEditable::File(file) => {
            file.set(value);
        }
        ItemEditable::Folder(folder) => {
            folder.set(value);
        }
        ItemEditable::Integer(integer) => {
            integer.set(value);
        }
        ItemEditable::Map(_) => {
            return Err(Message::error(
                MessageCategory::Datastore,
                "datastore_map_value_set_not_supported",
            ));
        }
        ItemEditable::Number(number) => {
            number.set(value);
        }
        ItemEditable::NumberWithUnits(number_with_units) => {
            number_with_units.set(value);
        }
        ItemEditable::String(string) => {
            string.set(value);
        }
        ItemEditable::Table(table) => {
            table.set_parameter(value);
        }
        ItemEditable::TableWithUnits(table_with_units) => {
            table_with_units.set_parameter(value);
        }
        ItemEditable::Unit(unit) => {
            unit.set(value);
        }
        ItemEditable::Tab(_) | ItemEditable::Separator(_) => {
            return Err(Message::error(
                MessageCategory::Datastore,
                "datastore_tab_or_separator_value_set_not_supported",
            ));
        }
    }

    Ok(())
}

/// Helper function to set the value of a map item in an editable map by key and item key.
///
/// # Errors
///
/// Returns a `Message` if the key or item key does not exist.
#[cfg_attr(feature = "hotpath", hotpath::measure)]
pub fn editable_set_map_value<S1: Into<ShareableString>, S2: Into<ShareableString>>(
    entry: &mut MapEntryEditable,
    key: S1,
    value: S2,
) -> Result<(), Message> {
    let key = key.into();

    let item = entry
        .get_mut(&key)
        .ok_or_else(|| Message::error(MessageCategory::Datastore, "datastore_key_not_found"))?;

    match item {
        MapItemEditable::Boolean(boolean) => {
            boolean.set(value);
        }
        MapItemEditable::Choice(choice) => {
            choice.set(value);
        }
        MapItemEditable::File(file) => {
            file.set(value);
        }
        MapItemEditable::Integer(integer) => {
            integer.set(value);
        }
        MapItemEditable::Number(number) => {
            number.set(value);
        }
        MapItemEditable::NumberWithUnits(number_with_units) => {
            number_with_units.set(value);
        }
        MapItemEditable::String(string) => {
            string.set(value);
        }
        MapItemEditable::Table(table) => {
            table.set_parameter(value);
        }
        MapItemEditable::TableWithUnits(table_with_units) => {
            table_with_units.set_parameter(value);
        }
        MapItemEditable::Unit(unit) => {
            unit.set(value);
        }
    }

    Ok(())
}
