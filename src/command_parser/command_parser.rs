use crate::datastore::DataStore;
use crate::types;
use crate::types::Types;


/// Parses the command that is given and applies it to the given DataStore.
pub fn parse(mut raw: &str, store: &mut DataStore){

    raw = raw.trim();

    // Removes trailing ';' if present since its not needed.
    if raw.ends_with(';') {
        raw = &raw[0..raw.len()-1];
    }

    for line in raw.split(';'){
        run_command(line, store);
    }
}

/// Runs a single command on the DataStore.
fn run_command(command: &str, store: &mut DataStore){

    let tokens: Vec<&str> = command.split(' ').collect::<Vec<&str>>();

    match tokens[0].to_uppercase().as_str() {
        "GET" => {
            if tokens.len() == 2 {
                let out: Option<&Types> = store.get(tokens[1]);

                print!("{}: ", tokens[1]);

                types::print_data(out);
                
            }  else {
                println!("Invalid arguments!")
            }
        },
        "SET" => {
            if tokens.len() >= 3 {
                store.set(tokens[1], Types::Text(tokens[2..tokens.len()].join(" ")));

            } else {
                println!("Not enough arguments!");
            }
        },
        _ =>{
            println!("Could not parse token \"{}\"", tokens[0]);
        }
    }
}