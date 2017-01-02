extern crate hyper;
extern crate rustc_serialize;

mod error;
mod client;

use client::SendSecure;

fn main() {
    match SendSecure::Client::get_user_token("acme", "username", "password", "device_id", "device_name", "application_type", "endpoint", false) {
        Ok(res) => println!("{}", res),
        Err(e) => println!("{:?}", e),
    };

}
