use crate::datastore::DataStore;
use crate::types;
use crate::types::Types;

/// Parses the command that is given and applies it to the given DataStore.
pub fn parse(mut raw: &str, store: &mut DataStore) {
  // Removes trailing ';' if present since its not needed.
  if raw.ends_with(';') {
    raw = &raw[0..raw.len() - 1];
  }

  for line in raw.split(';') {
    let temp = line.trim();
    run_command(temp, store);
  }
}

/// Runs a single command on the DataStore.
fn run_command(command: &str, store: &mut DataStore) {
  let tokens: Vec<&str> = command.split(' ').collect::<Vec<&str>>();

  match tokens[0].to_uppercase().as_str() {
    // Strings
    "SET" => {
      if tokens.len() >= 3 {
        store.set(tokens[1], Types::String(tokens[2..tokens.len()].join(" ")));
      } else {
        println!("Not enough arguments!");
      }
    }
    "GET" => {
      if tokens.len() == 2 {
        match store.get(tokens[1]) {
          Some(data) => {
            print!("{}: ", tokens[1]);

            types::print_data(data.clone());
          }
          None => {
            print!("null");
          }
        }
      } else {
        println!("Invalid arguments!")
      }
    }
    "MGET" => {}
    // Lists
    "LPUSH" => {}
    "LPOP" => {}
    "RPUSH" => {}
    "RPOP" => {}
    "LLEN" => {}
    "LMOVE" => {}
    "LTRIM" => {}
    // HashSets
    "SADD" => {}
    "SREM" => {}
    "SISMEMBER" => {}
    "SINTER" => {}
    "SCARD" => {}
    // HashMaps
    "HSET" => {}
    "HGET" => {}
    "HMGET" => {}
    "HINCRBY" => {}
    // SortedSets
    "ZADD" => {}
    "ZRANGE" => {}
    "ZRANK" => {}
    "ZREVRANK" => {}
    _ => {
      println!("Could not parse token \"{}\"", tokens[0]);
    }
  }
}
