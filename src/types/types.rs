#[derive(Clone)]
pub enum Types {
    Text(String),
    List(Vec<Option<Types>>),
}
