mod command_parser;
mod datastore;
mod types;

use datastore::DataStore;

fn main() {
  let mut data_store: DataStore = DataStore::new();

  command_parser::parse("SET joe:1 this is a test; GET joe:1;", &mut data_store);
}
