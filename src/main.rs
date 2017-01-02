extern crate reqwest;
extern crate rustc_serialize;

mod error;
mod client;

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

}
