use std::collections::HashMap;

mod types;
use crate::types::Types;

fn main() {
    let mut data_store: HashMap<String, Types> = HashMap::new();

    data_store.insert(String::from("Test"), Types::Text("This is some data.".to_string()));

    let vec = create_list_strings(vec!["This", "Is", "Data"]);

    data_store.insert("ListTest".to_string(), vec);

    print!("Test: ");
    print_data(data_store.get(&String::from("Test")));
    println!();
    print!("ListTest: ");
    print_data(data_store.get(&String::from("ListTest")));
    println!();
    
}

fn create_list_strings(data: Vec<&str>) -> Types{
    let mut vec = vec![Option::Some(Types::Text("".to_string()));data.len()];

    for i in 0..data.len(){
        vec[i] = Option::Some(Types::Text(data[i].to_string()));
    }

    let out = Types::List(vec);

    return out;
}

fn print_data(data: Option<&Types>){
    match data {
        Some(i) => {
            match i {
                Types::Text(out) => {
                    print!("\"{}\"", out);
                },
                Types::List(out) => {
                    print!("{{ ");
                    for i in 0..out.len(){
                        print_data(out[i].as_ref());
                        if i != out.len() - 1 {
                            print!(", ");
                        }
                    }
                    print!(" }}");
                }
            }
        },
        None =>{
            println!("NULL");
        }
    }
}