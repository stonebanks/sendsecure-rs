#[derive(Debug, RustcDecodable, Clone)]
pub struct Value<T> {
    pub value: Option<T>,
    pub modifiable: Option<bool>,
}
