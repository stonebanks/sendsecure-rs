pub enum DestinationType {
    HomePhone,
    CellPhone,
    OfficePhone,
    OtherPhone,
}

pub struct ContactMethod {
    pub destination_type: DestinationType,
    pub destination: String,
}
