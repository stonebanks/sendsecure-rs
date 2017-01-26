use std::cmp::Ordering;
use std::option::Option;


// use hyper::header::{Headers, Accept, qitem};
use hyper::{client, status, method, header, net};
use hyper::mime::{Mime, TopLevel, SubLevel, Attr, Value};
use std::io::Read;
use error::{SendSecureResult, SendSecureError};
use multipart::client::{Multipart, SizedRequest};
use url::Url;

pub fn make_request(method: method::Method,
                    url: &str,
                    mut params: Option<String>,
                    headers: Option<header::Headers>)
                    -> SendSecureResult<String> {
    let client = client::Client::new();
    let mut request = client.request(method, url);
    let hdrs = match headers {
        Some(mut hdrs) => {
            hdrs.set(header::ContentType::json());
            hdrs
        }
        None => {
            let mut hdrs = header::Headers::new();
            hdrs.set(header::ContentType::json());
            hdrs
        }
    };
    if let Some(ref mut body) = params {
        request = request.body(body.as_str());
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

pub fn post_file<F>(upload_url: Url, mut f: F) -> SendSecureResult<String>
    where F: FnMut(&mut Multipart<SizedRequest<client::Request<net::Fresh>>>)
                   -> SendSecureResult<()>
{
    let mut request = client::Request::new(method::Method::Post, upload_url)?;

    let mut headers = request.headers_mut();
    headers.set(header::Accept(vec![header::qitem(Mime(TopLevel::Application,
                                                       SubLevel::Json,
                                                       vec![(Attr::Charset, Value::Utf8)]))]));
    //let stream = request.start()?;
    let mut multipart = Multipart::from_request_sized(request)?;

    f(&mut multipart);
    let mut res = multipart.send()?;
    println!("{:?}", res);
    let status_code = res.status.class().default_code();
    res = try!(match status_code.cmp(&status::StatusCode::BadRequest) {
        Ordering::Less => Ok(res),
        Ordering::Greater | Ordering::Equal => Err(SendSecureError::ResponseError(status_code)),
    });
    let mut body = String::new();
    res.read_to_string(&mut body)?;
    Ok(body)
}
