use std::collections::HashMap;

enum Types {
    Text(String),
    List(Vec<Option<Types>>)
}



fn main() {
    let mut data_store: HashMap<String, Types> = HashMap::new();

    data_store.insert(String::from("Test"), Types::Text(String::from("This is some data.")));

    let vec = vec![Option::Some(Types::Text(String::from("This"))), Option::Some(Types::Text(String::from("Is"))), Option::Some(Types::Text(String::from("Data")))];

    data_store.insert(String::from("ListTest"), Types::List(vec));

    print!("Test: ");
    print_data(data_store.get(&String::from("Test")));
    println!();
    print!("ListTest: ");
    print_data(data_store.get(&String::from("ListTest")));
    println!();
    
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