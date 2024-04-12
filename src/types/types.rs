use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;

/// An enum that holds the allowed types.
#[derive(Clone)]
pub enum Types {
  String(String),
  List(Vec<String>),
  Set(HashSet<String>),
  Hash(HashMap<String, String>),
  SortedSet(BTreeSet<String>),
  // Json will come soon.
}

/// A struct for a key value of a String and a Type.
#[derive(Clone)]
pub struct KeyValue {
  pub key: String,
  pub value: Option<Types>,
}

/// Prints the given type.
pub fn print_data(data: Types) {
  match data {
    Types::String(out) => {
      print!("\"{}\"", out);
    }
    Types::List(out) => {
      print!("[ ");

      for i in 0..out.len() {
        print!("{}", out[i].as_str());

        if i != out.len() - 1 {
          print!(", ");
        }
      }
      print!(" ]");
    }
    Types::Set(out) => {}
    Types::Hash(out) => {}
    Types::SortedSet(out) => {}
  }
}
