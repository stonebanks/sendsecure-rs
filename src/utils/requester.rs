use std::cmp::Ordering;
use std::option::Option;


// use hyper::header::{Headers, Accept, qitem};
use hyper::{client, status, method, header};
use std::io::Read;
use error::{SendSecureResult, SendSecureError};

pub fn make_request(method: method::Method,
                    url: &str,
                    mut params: Option<&[u8]>,
                    headers: Option<header::Headers>)
                    -> SendSecureResult<String> {
    let client = client::Client::new();
    let mut request = client.request(method, url);
    let mut hdrs = headers.unwrap_or(header::Headers::new());
    if hdrs.has::<header::ContentType>() == false {
        hdrs.set(header::ContentType::json());
    }
    if let Some(ref mut body) = params {
        request = request.body(body);
    }
    request = request.headers(hdrs);
    let mut res = request.send()?;
    let status_code = res.status.class().default_code();
    res = try!(match status_code.cmp(&status::StatusCode::BadRequest) {
        Ordering::Less => Ok(res),
        Ordering::Greater | Ordering::Equal => Err(SendSecureError::ResponseError(status_code)),
    });
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    Ok(body)
}
