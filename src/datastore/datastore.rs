use std::collections::HashMap;
use crate::types::Types;

/// A struct that holds a the hashmap that stores the data.
pub struct DataStore {
  store: HashMap<String, Types>,
}

impl DataStore {
  /// Creates a new DataStore.
  pub fn new() -> Self {
    let store = HashMap::new();
    Self { store }
  }

  /// Sets a value in the datastore with a given key.
  pub fn set(&mut self, key: &str, value: Types) {
    self.store.insert(key.to_string(), value);
  }

  /// Gets a value from the datastore with the given key.
  pub fn get(&self, value: &str) -> Option<&Types> {
    self.store.get(&String::from(value))
  }
}
