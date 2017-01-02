pub mod SendSecure {

    use std::cmp::Ordering;
    use rustc_serialize::json::Json;
    // use hyper::{client, error, status};
    use reqwest;
    use std::io::Read;
    use std::result::Result;
    use error::SendSecure;

    pub struct Client;

    impl Client {
        fn make_request(url: &str) -> SendSecure::SendSecureResult<String> {
            let client = reqwest::Client::new().unwrap();
            let mut res = try!(client.get(url).send());
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

        fn get_portal_url_for_permalink(endpoint: &str,
                                        enterprise_account: &str)
                                        -> SendSecure::SendSecureResult<String> {
            let formatted_url =
                format!("{0}/services/{1}/portal/host", endpoint, enterprise_account);
            Client::make_request(formatted_url.as_str())
        }

        pub fn get_user_token(enterprise_account: &str,
                              username: &str,
                              password: &str,
                              device_id: &str,
                              device_name: &str,
                              application_type: &str,
                              endpoint: &str,
                              one_time_password: bool)
                              -> SendSecure::SendSecureResult<String> {
            let url: &str = "https://httpbin.org/get";//"https://secure.bixi.com/data/stations.json";
            let body = try!(Client::make_request(url));
            let json_body = try!(Json::from_str(body.as_str()));
            if let Some(obj) = json_body.as_object() {
                if let Some(origin) = obj.get("origin") {
                    return Ok(origin.as_string().unwrap_or_default().to_string());
                }
            }
            Err(SendSecure::SendSecureError::UnexpectedResponseError(body))
        }
    }
}
