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

use client::SendSecure;
use url::Url;
use std::path::Path;
use jsonclient::{UploadFileWithPath, UploadFileWithStream};
use std::fs::File;


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
    let mut jsonclient =
        jsonclient::JsonClient::new("api_token".to_string(), "xmdev".to_string(), None, None);

    let url = Url::parse("https://httpbin.org/post").unwrap();
    let path = Path::new("/tmp/toto.txt");
    let tutu = UploadFileWithPath::upload_file(&mut jsonclient, url, path);//jsonclient.upload_file(url, path).unwrap();
    println!("{:?}", tutu);

    let url2 = Url::parse("https://httpbin.org/post").unwrap();
    let mut file = File::open("/tmp/toto.txt").unwrap();
    let toutou = UploadFileWithStream::upload_file(&mut jsonclient,
                                                   url2,
                                                   &mut file,
                                                   None,
                                                   "toto".to_string());
    println!("{:?}", toutou);

    // jsonclient.upload_file( url2,
    //                          &mut file,
    //                          content_type: Option<Mime>,
    //                          file_name: String)
    // let test2 = jsonclient.new_safebox("tss@test.com").unwrap();
    // println!("{:?}", test2);

}
