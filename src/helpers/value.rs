use rustc_serialize::{Decodable, Decoder};
use std::result::Result;

#[derive(Debug)]
pub struct Value {
    pub value: Option<String>,
    pub modifiable: Option<bool>,
}


impl Decodable for Value {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Value, D::Error> {
        decoder.read_struct("root", 0, |decoder| {
            Ok(Value {
                value: decoder.read_struct_field("value", 0, |decoder| Decodable::decode(decoder))
                    .unwrap_or(None),
                modifiable:
                    decoder.read_struct_field("modifiable", 0, |decoder| Decodable::decode(decoder))
                    .unwrap_or(None),
            })
        })
    }
}
