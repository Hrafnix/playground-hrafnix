//! Demonstrates basic usage of the datastore crate.
//!
//! Creates a [`Store`], defines an [`ObjectDefinition`] with string and number
//! properties, adds an object instance, and reads/writes values through
//! proxy-based access using the [`store_key!`] macro.
use datastore::prelude::*;

fn main() {
    // 1. Create a Store
    let store = Store::new(Default::default());

    // 2. Define an Object Structure
    // An Object is a collection of named properties.
    let mut user_def = ObjectDefinition::builder("User Profile");
    user_def.insert(
        store_key!("username"),
        PropertyDefinition::new("The user's unique name", BasicDefinition::new_string("")),
    );
    user_def.insert(
        store_key!("age"),
        PropertyDefinition::new("The user's age", BasicDefinition::new_number("0")),
    );

    let def = user_def.finish();

    // 3. Add an Object to the Store
    // Objects are added at the top level with a unique key.
    store.create_object(store_key!("user_123"), &def).unwrap();

    // 4. Access Data via Proxies
    // The proxy provides a way to interact with data in the store.
    let mut user_proxy = store.object("user_123").unwrap();
    let mut username_proxy = user_proxy.basic(store_key!("username")).unwrap();
    let mut age_proxy = user_proxy.basic(store_key!("age")).unwrap();

    // 5. Update Data
    // Changes are made to the proxy first, then pushed to the store.
    username_proxy.set_value("john doe");
    username_proxy.push().unwrap();

    age_proxy.set_value("30");
    age_proxy.push().unwrap();

    // 6. Observe Changes
    // If another handle to the same data exists, it can observe changes.
    let mut observer_proxy = store.basic(&path!("user_123/username")).unwrap();

    // We update the data via another proxy.
    username_proxy.set_value("john doe updated");
    username_proxy.push().unwrap();

    // The store has changed, so has_changed() will return true.
    assert!(observer_proxy.has_changed());

    // Pull the latest data from the store.
    observer_proxy.pull().unwrap();
    assert_eq!(observer_proxy.value().as_str(), "john doe updated");

    println!("Username: {}", observer_proxy.value());
    println!("Age: {}", age_proxy.value());
}
