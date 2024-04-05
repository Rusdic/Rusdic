mod types;
use types::Types;

mod datastore;
use datastore::DataStore;


fn main() {
    let mut data_store: DataStore = DataStore::new();

    data_store.set("Test", Types::Text("This is some data.".to_string()));

    let vec = create_list_strings(vec!["This", "Is", "Data"]);

    data_store.set("ListTest", vec);

    print!("Test: ");
    print_data(data_store.get("Test"));
    println!();
    print!("ListTest: ");
    print_data(data_store.get("ListTest"));
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