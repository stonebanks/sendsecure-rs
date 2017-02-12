#![deny(warnings)]
extern crate sendsecure;

use sendsecure::client;

fn main() {
    let user_token = client::Client::get_user_token("deathstar",
                                                    "darthvader",
                                                    "d@Rk$1De",
                                                    "DV-TIE/x1",
                                                    "TIE Advanced x1",
                                                    Some("The Force App"),
                                                    None,
                                                    None)
        .unwrap();

    println!("{}", user_token);
}
