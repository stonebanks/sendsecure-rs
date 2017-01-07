#[derive(Debug)]
pub struct Recipient {
    pub email: String,
    pub contact_methods: Vec<String>,
    pub firstname: Option<String>,
    pub lastname: Option<String>,
    pub company_name: Option<String>,
}

impl Recipient {
    pub fn new(email: String) -> Recipient {
        Recipient {
            email: email,
            contact_methods: vec![],
            firstname: None,
            lastname: None,
            company_name: None,
        }
    }
}
