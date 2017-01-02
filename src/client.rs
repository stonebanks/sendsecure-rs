pub mod SendSecure {

    use std::cmp::Ordering;
    use rustc_serialize::json::Json;
    use std::option::Option;
    use std::collections::HashMap;
    use reqwest;
    use std::io::Read;
    use error::SendSecure;

    pub struct Client;

    impl Client {
        fn make_request(method: reqwest::Method,
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

        pub fn get_user_token(enterprise_account: &str,
                              username: &str,
                              password: &str,
                              device_id: &str,
                              device_name: &str,
                              application_type: &str,
                              endpoint: &str,
                              one_time_password: bool)
                              -> SendSecure::SendSecureResult<String> {
            let formatted_url =
                format!("{0}/services/{1}/portal/host", endpoint, enterprise_account);
            let uurl =
                try!(Client::make_request(reqwest::Method::Get, formatted_url.as_str(), None));
            let url: &str = "http://httpbin.org/post";//"https://secure.bixi.com/data/stations.json";
            let mut params = HashMap::new();
            params.insert("permalink", enterprise_account);
            params.insert("username", username);
            params.insert("password", password);
            params.insert("application_type", application_type);
            params.insert("device_id", device_id);
            params.insert("device_name", device_name);
            if one_time_password {
                params.insert("otp", "true");
            }
            let body = try!(Client::make_request(reqwest::Method::Post, url, Some(params)));
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
