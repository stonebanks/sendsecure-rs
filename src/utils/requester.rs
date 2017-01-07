use std::cmp::Ordering;
use rustc_serialize::json::Json;
use std::option::Option;
use std::collections::HashMap;
use reqwest;
use std::io::Read;
use error::SendSecure;

pub fn make_request(method: reqwest::Method,
                    url: &str,
                    mut params: Option<HashMap<&str, &str>>)
                    -> SendSecure::SendSecureResult<String> {
    let mut res = try!(reqwest::Client::new()
        .map_err(SendSecure::SendSecureError::ClientInitializationError)
        .and_then(|client| {
            let mut request = client.request(method, url);
            if let Some(ref mut body) = params {
                request = request.form(&body);
            }
            request.send()
                .map_err(SendSecure::SendSecureError::RequestBuilderError)
        }));
    let status_code = res.status().class().default_code();
    res = try!(match status_code.cmp(&reqwest::StatusCode::BadRequest) {
        Ordering::Less => Ok(res),
        Ordering::Greater | Ordering::Equal => {
            Err(SendSecure::SendSecureError::ResponseError(status_code))
        }
    });
    let mut body = String::new();
    try!(res.read_to_string(&mut body));
    Ok(body)
}
