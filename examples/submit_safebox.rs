#![deny(warnings)]
extern crate sendsecure;

use std::path::Path;
use sendsecure::{helpers, client};

fn main() {

    let user_email = "darthvader@empire.com";
    let token = "USER|1d495165-4953-4457-8b5b-4fcf801e621a";
    let enterprises_account = "deathstar";
    let endpoint = "https://portal.xmedius.com";
    let mut safebox = helpers::safebox::Safebox::new(user_email);
    safebox.subject = Some("Hello World".to_string());
    safebox.message = Some("Son, you will find attached the evidence.".to_string());

    let mut recipient = helpers::recipient::Recipient::new("lukeskywalker@rebels.com");
    let contact_method = helpers::contactmethod::ContactMethod {
        destination_type: helpers::contactmethod::DestinationType::CellPhone,
        destination: "+15145550000".to_string(),
    };
    recipient.contact_methods.push(contact_method);
    recipient.first_name = Some("Allan".to_string());
    recipient.last_name = Some("Seymour".to_string());
    recipient.company_name = Some("XMedius".to_string());
    safebox.recipients.push(recipient);

    safebox.attachments
        .push(helpers::attachment::Attachment::new(Path::new(r#"Birth_Certificate.pdf"#)).unwrap());
    let mut client = client::Client::new(token, enterprises_account, Some(endpoint), None);
    println!("{:?}", client.submit_safebox(&mut safebox));

}
