use helpers::contactmethod;

#[derive(Debug, RustcDecodable, RustcEncodable, Clone)]
pub struct Recipient {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company_name: Option<String>,
    pub contact_methods: Vec<contactmethod::ContactMethod>,
}

impl Recipient {
    pub fn new(email: &str) -> Recipient {
        Recipient {
            email: email.to_string(),
            first_name: None,
            last_name: None,
            company_name: None,
            contact_methods: vec![],
        }
    }
}
