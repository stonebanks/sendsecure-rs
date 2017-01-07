extern crate reqwest;
extern crate rustc_serialize;
use rustc_serialize::json;

mod utils;
mod error;
mod client;
mod helpers;
mod jsonclient;

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

    let test = r#"{"mode": "allow", "list": ["exe", "zip", "bak"]}"#;

    println!("{:?}",
             json::decode::<helpers::enterprisesettings::ExtensionFilter>(test).unwrap());

    let ent = r#"{}"#;
    println!("{:?}",
             json::decode::<helpers::enterprisesettings::EnterpriseSettings>(ent).unwrap());

}
