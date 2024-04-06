mod types;

use types::Types;
use types::KeyValue;
use types::print_data;

mod datastore;
use datastore::DataStore;




fn main() {
    let mut data_store: DataStore = DataStore::new();

    data_store.set("Test", Types::Text("This is some data.".to_string()));

    let list = create_list_strings(vec!["This", "Is", "Data"]);

    data_store.set("ListTest", list);


    data_store.set("Joe", Types::Json(vec![Option::Some(KeyValue { key: "Name".to_string(), value: Option::Some(Types::Text("Joe".to_string()))})]));



    print!("Test: ");
    print_data(data_store.get("Test"));
    println!();
    print!("ListTest: ");
    print_data(data_store.get("ListTest"));
    println!();
    print!("Joe: ");
    print_data(data_store.get("Joe"));
    
}

fn create_list_strings(data: Vec<&str>) -> Types{
    let mut vec = vec![Option::Some(Types::Text("".to_string()));data.len()];

    for i in 0..data.len(){
        vec[i] = Option::Some(Types::Text(data[i].to_string()));
    }

    let out = Types::List(vec);

    return out;
}

