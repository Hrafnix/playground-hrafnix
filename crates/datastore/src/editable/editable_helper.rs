use crate::StoreError;
use crate::editable::ItemEditable;
use crate::traits::ObjectEditable;
use shareable_string::ShareableString;

/// Helper function to set the value of an editable object by key.
///
/// # Errors
///
/// Returns a `StoreError` if the key does not exist or if the value type does not match the expected type.
pub fn editable_set_value<
    T: ObjectEditable,
    S1: Into<ShareableString>,
    S2: Into<ShareableString>,
>(
    obj: &mut T,
    key: S1,
    value: S2,
) -> Result<(), StoreError> {
    let key = key.into();
    let value = value.into();

    let item = obj.get_mut(&key).ok_or(StoreError::KeyNotFound)?;

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
        ItemEditable::Integer(integer) => {
            integer.set(value);
        }
        ItemEditable::Map(_) => {
            return Err(StoreError::InvalidType("Cannot set a value for a Map item directly. Use the appropriate methods to modify the map.".to_string()));
        }
        ItemEditable::Number(number) => {
            number.set(value);
        }
        ItemEditable::String(string) => {
            string.set(value);
        }
        ItemEditable::Table(table) => {
            table.set_parameter(value);
        }
    }

    Ok(())
}
