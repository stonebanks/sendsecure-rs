extern crate hyper;
extern crate rustc_serialize;
extern crate url;
extern crate multipart;
extern crate mime;


use rustc_serialize::json;

mod utils;
mod error;
mod client;
mod helpers;
mod jsonclient;
mod response;

use response::success::security_profiles::SecurityProfiles;
use error::SendSecureResult;
// use client::SendSecure;
// use url::Url;
// use std::path::Path;
// use jsonclient::{UploadFileWithPath, UploadFileWithStream};
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

    let secu = r#"{"security_profiles":[{
			"id": "39",
			"name": "All Contact Method Allowed!",
			"description": "All Contact Method Allowed!",
			"created_at": "2016-01-27T18:53:00.631Z",
			"updated_at": "2016-09-14T18:41:23.043Z",
			"allowed_login_attempts": {
				"value": null,
				"modifiable": false
			}

		}]}"#;
    //,"updated_at":"2016-09-14T18:41:23.043Z","allowed_login_attempts":{"value":10,"modifiable":false},"allow_remember_me":{"value":true,"modifiable":false},"allow_sms":{"value":true,"modifiable":false},"allow_voice":{"value":true,"modifiable":false},"allow_email":{"value":true,"modifiable":false},"code_time_limit":{"value":5,"modifiable":false},"code_length":{"value":6,"modifiable":false},"auto_extend_value":{"value":6,"modifiable":false},"auto_extend_unit":{"value":"hours","modifiable":false},"two_factor_required":{"value":true,"modifiable":false},"encrypt_attachments":{"value":true,"modifiable":false},"encrypt_message":{"value":true,"modifiable":false},"expiration_value":{"value":7,"modifiable":false},"expiration_unit":{"value":"days","modifiable":false},"reply_enabled":{"value":true,"modifiable":true},"group_replies":{"value":true,"modifiable":false},"retention_period_type":{"value":"discard_at_expiration","modifiable":false},"retention_period_value":{"value":null,"modifiable":false},"retention_period_unit":{"value":null,"modifiable":false}
    let toto: response::success::security_profiles::SecurityProfiles = json::decode(secu).unwrap();

    println!("{:?}", toto);
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
    // let response = jsonclient.new_safebox("toto@toto.com").unwrap();
    // println!("{}", response);
    //
    // let response2 = jsonclient.enterprise_settings().unwrap();
    // println!("{}", response2);
    //
    // let response3 = jsonclient.security_profiles("toto@toto.com").unwrap();
    // let toto: SecurityProfiles = json::decode(response3.as_str()).unwrap();
    // println!("{:?}", toto);

    let mut client = client::Client::new("USER|ac1cf0f7-ca4e-4d4b-a3e4-9164608800c1",
                                         "acme",
                                         Some("https://portal.integration.xmedius.com"),
                                         None);

    println!("{:?}", client.security_profiles("toto@toto.com").unwrap());
    println!("{:?}", client.enterprise_settings().unwrap());

    //
    // let url = Url::parse("https://httpbin.org/post").unwrap();
    // let path = Path::new(r#"e:\compile\auto\temp\toto.txt"#);
    // let tutu = UploadFileWithPath::upload_file(&mut jsonclient, url, path);//jsonclient.upload_file(url, path).unwrap();
    // println!("{:?}", tutu);
    //
    // let url2 = Url::parse("https://httpbin.org/post").unwrap();
    // let mut file = File::open(r#"e:\compile\auto\temp\toto.txt"#).unwrap();
    // let toutou = UploadFileWithStream::upload_file(&mut jsonclient,
    //                                                url2,
    //                                                &mut file,
    //                                                None,
    //                                                "toto".to_string());
    // println!("{:?}", toutou);

    // jsonclient.upload_file( url2,
    //                          &mut file,
    //                          content_type: Option<Mime>,
    //                          file_name: String)
    // let test2 = jsonclient.new_safebox("tss@test.com").unwrap();
    // println!("{:?}", test2);

}
