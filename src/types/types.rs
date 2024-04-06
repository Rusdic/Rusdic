#[derive(Clone)]
pub enum Types {
    Text(String),
    List(Vec<Option<Types>>),
    Json(Vec<Option<KeyValue>>)
}

#[derive(Clone)]
pub struct KeyValue {
    pub key: String,
    pub value: Option<Types>
}

pub fn print_data(data: Option<&Types>){
    match data {
        Some(i) => {
            match i {
                Types::Text(out) => {
                    print!("\"{}\"", out);
                },
                Types::List(out) => {
                    print!("[ ");

                    for i in 0..out.len(){

                        print_data(out[i].as_ref());

                        if i != out.len() - 1 {
                            print!(", ");
                        }
                    }
                    print!(" ]");
                },
                Types::Json(data) => {
                    print!("{{ ");

                    for i in 0..data.len(){
                        match &data[i] {
                            Some(j) => {
                                print!("{}: ", j.key);

                                print_data(j.value.as_ref())
                            },
                            None => {
                                print!("null");
                            }
                        }
                    }
                    print!(" }}");
                }
            }
        },
        None =>{
            println!("null");
        }
    }
}