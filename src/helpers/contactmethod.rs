use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};

#[derive(Debug, Clone)]
pub enum DestinationType {
    HomePhone,
    CellPhone,
    OfficePhone,
    OtherPhone,
}

#[derive(Debug, RustcDecodable, RustcEncodable, Clone)]
pub struct ContactMethod {
    pub destination_type: DestinationType,
    pub destination: String,
}


impl Decodable for DestinationType {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<DestinationType, D::Error> {
        decoder.read_enum("DestinationType", |decoder| {
            decoder.read_enum_variant(&["home_phone", "cell_phone", "office_phone", "other_phone"],
                                      |_, x| match x {
                                          0 => Result::Ok(DestinationType::HomePhone),
                                          1 => Result::Ok(DestinationType::CellPhone),
                                          2 => Result::Ok(DestinationType::OfficePhone),
                                          _ => Result::Ok(DestinationType::OtherPhone),
                                      })
        })
    }
}


impl Encodable for DestinationType {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_enum("DestinationType", |s| match *self {
            DestinationType::HomePhone => s.emit_enum_variant("home_phone", 0, 0, |_| Ok(())),
            DestinationType::CellPhone => s.emit_enum_variant("cell_phone", 1, 0, |_| Ok(())),
            DestinationType::OfficePhone => s.emit_enum_variant("office_phone", 2, 0, |_| Ok(())),
            DestinationType::OtherPhone => s.emit_enum_variant("other_phone", 3, 0, |_| Ok(())),
        })
    }
}
