extern crate reqwest;
extern crate rustc_serialize;
use rustc_serialize::json;


mod error;
mod client;
mod helpers;

use client::SendSecure;

fn main() {
    match SendSecure::Client::get_user_token("xmdev",
                                             "username",
                                             "password",
                                             "device_id",
                                             "device_name",
                                             "application_type",
                                             "https://portal.xmedius.com",
                                             false) {
        Ok(res) => println!("{}", res),
        Err(e) => println!("{:?}", e),
    };

    let secu =
        r#"{"id": "Person", "name": "Titi", "allow_sms": {"value": "tutu", "modifiable": true}}"#;
    let toto: helpers::securityprofile::SecurityProfile = json::decode(secu).unwrap();

    println!("{:?}", toto);
}
