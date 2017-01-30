extern crate hyper;
extern crate rustc_serialize;
extern crate url;
extern crate mime;
extern crate mime_multipart;
extern crate mime_guess;


use rustc_serialize::json;

mod utils;
mod error;
mod client;
mod helpers;
mod jsonclient;
mod json_objects;

use json_objects::response::success::security_profiles::SecurityProfiles;
use error::SendSecureResult;
use std::path::Path;
use std::fs::File;
use url::Url;
// use client::SendSecure;
// use url::Url;
// use std::path::Path;
use jsonclient::UploadFileWithPath;
// use std::fs::File;


fn main() {
    // match SendSecure::Client::get_user_token("acme",
    //                                          "bonjour",
    //                                          "Qwerty123",
    //                                          "device_id",
    //                                          "device_name",
    //                                          "application_type",
    //                                          "https://portal.integration.xmedius.com",
    //                                          false) {
    //     Ok(res) => println!("{}", res),
    //     Err(e) => println!("{:?}", e),
    // };

    // let secu = r#"{"security_profiles":[{
    // 		"id": "39",
    // 		"name": "All Contact Method Allowed!",
    // 		"description": "All Contact Method Allowed!",
    // 		"created_at": "2016-01-27T18:53:00.631Z",
    // 		"updated_at": "2016-09-14T18:41:23.043Z",
    // 		"allowed_login_attempts": {
    // 			"value": null,
    // 			"modifiable": false
    // 		}
    //
    // 	}]}"#;
    // //,"updated_at":"2016-09-14T18:41:23.043Z","allowed_login_attempts":{"value":10,"modifiable":false},"allow_remember_me":{"value":true,"modifiable":false},"allow_sms":{"value":true,"modifiable":false},"allow_voice":{"value":true,"modifiable":false},"allow_email":{"value":true,"modifiable":false},"code_time_limit":{"value":5,"modifiable":false},"code_length":{"value":6,"modifiable":false},"auto_extend_value":{"value":6,"modifiable":false},"auto_extend_unit":{"value":"hours","modifiable":false},"two_factor_required":{"value":true,"modifiable":false},"encrypt_attachments":{"value":true,"modifiable":false},"encrypt_message":{"value":true,"modifiable":false},"expiration_value":{"value":7,"modifiable":false},"expiration_unit":{"value":"days","modifiable":false},"reply_enabled":{"value":true,"modifiable":true},"group_replies":{"value":true,"modifiable":false},"retention_period_type":{"value":"discard_at_expiration","modifiable":false},"retention_period_value":{"value":null,"modifiable":false},"retention_period_unit":{"value":null,"modifiable":false}
    // let toto: json_objects::response::success::security_profiles::SecurityProfiles =
    //     json::decode(secu).unwrap();
    //
    // println!("{:?}", toto);
    //
    // let test = r#"{"mode": "allow", "list": ["exe", "zip", "bak"]}"#;
    //
    // println!("{:?}",
    //          json::decode::<helpers::enterprisesettings::ExtensionFilter>(test).unwrap());
    //
    // let ent = r#"{}"#;
    // println!("{:?}",
    //          json::decode::<helpers::enterprisesettings::EnterpriseSettings>(ent).unwrap());



    // let mut jsonclient =
    //     jsonclient::JsonClient::new("USER|ac1cf0f7-ca4e-4d4b-a3e4-9164608800c1".to_string(),
    //                                 "acme".to_string(),
    //                                 Some("https://portal.integration.xmedius.com".to_string()),
    //                                 None);
    //
    // let _ = jsonclient.upload_file(Url::parse("http://httpbin.org/post").unwrap(),
    //                                &Path::new(r#"C:\Users\allan.seymour\Pictures\30c744a23fc46a203003a6e2e8990465.jpg"#));


    // //
    // let response = jsonclient.new_safebox("toto@toto.com").unwrap();
    // println!("{}", response);
    //
    // let response2 = jsonclient.enterprise_settings().unwrap();
    // println!("{}", response2);
    //
    // let response3 = jsonclient.security_profiles("toto@toto.com").unwrap();
    // let toto: SecurityProfiles = json::decode(response3.as_str()).unwrap();
    // println!("{:?}", toto);

    // let mut client = client::Client::new("USER|ac1cf0f7-ca4e-4d4b-a3e4-9164608800c1",
    //                                      "acme",
    //                                      Some("https://portal.integration.xmedius.com"),
    //                                      None);

    // println!("{:?}", client.security_profiles("toto@toto.com").unwrap());
    // println!("{:?}", client.enterprise_settings().unwrap());
    // println!("{:?}",
    //          client.default_security_profile("toto@toto.com").unwrap());
    // let url = Url::parse("https://httpbin.org/post").unwrap();
    // let path = Path::new(r#"e:\compile\auto\temp\toto.txt"#);
    // let tutu = UploadFileWithPath::upload_file(&mut jsonclient, url, path);//jsonclient.upload_file(url, path).unwrap();
    // println!("{:?}", tutu);
    //
    // let url2 = Url::parse("https://httpbin.org/post").unwrap();
    // let mut file = File::open(r#"e:\compile\auto\temp\toto.txt"#).unwrap();
    // let toutou = jsonclient.upload_file(url2, &mut file, "text/plain".parse().unwrap(), "toto");
    // println!("{:?}", toutou);

    let user_email = "toto@toto.com";
    let token = "USER|ac1cf0f7-ca4e-4d4b-a3e4-9164608800c1";
    let enterprises_account = "acme";
    let endpoint = "https://portal.integration.xmedius.com";
    let mut safebox = helpers::safebox::Safebox::new(user_email);
    safebox.subject = Some("Hello World".to_string());
    safebox.message = Some("Son, you will find attached the evidence.".to_string());

    let mut recipient = helpers::recipient::Recipient::new("allan.seymour@xmedius.com");
    let contact_method = helpers::contactmethod::ContactMethod {
        destination_type: helpers::contactmethod::DestinationType::CellPhone,
        destination: "+15146384760".to_string(),
    };
    recipient.contact_methods.push(contact_method);
    recipient.first_name = Some("Allan".to_string());
    recipient.last_name = Some("Seymour".to_string());
    recipient.company_name = Some("XMedius".to_string());
    safebox.recipients.push(recipient);

    safebox.attachments
        .push(helpers::attachment::Attachment::new(Path::new(r#"E:\compile\auto\temp\toto.txt"#))
            .unwrap());
    safebox.attachments.push(helpers::attachment::Attachment::new(Path::new(r#"C:\Users\allan.seymour\Pictures\30c744a23fc46a203003a6e2e8990465.jpg"#)).unwrap());
    let mut client = client::Client::new(token, enterprises_account, Some(endpoint), None);
    println!("{:?}", client.submit_safebox(&mut safebox));



    // jsonclient.upload_file( url2,
    //                          &mut file,
    //                          content_type: Option<Mime>,
    //                          file_name: String)
    // let test2 = jsonclient.new_safebox("tss@test.com").unwrap();
    // println!("{:?}", test2);

}
