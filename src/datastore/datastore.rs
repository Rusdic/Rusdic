use std::collections::HashMap;
use crate::types::types::Types;

pub struct DataStore {
    store: HashMap<String, Types>
}

impl DataStore {
    pub fn new() -> Self {
        let store = HashMap::new();
        Self { store }
    }

    pub fn set(&mut self, key: &str, value: Types){
        self.store.insert(key.to_string(), value);
    }

    pub fn get(&self, value: &str) -> Option<&Types> {
        self.store.get(&String::from(value))
    }
}