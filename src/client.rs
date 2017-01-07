pub mod SendSecure {

    use std::cmp::Ordering;
    use rustc_serialize::json::Json;
    use std::option::Option;
    use std::collections::HashMap;
    use reqwest;
    use std::io::Read;
    use error::SendSecure;
    use utils::requester::make_request;

    pub struct Client;

    impl Client {
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
            let uurl = make_request(reqwest::Method::Get, formatted_url.as_str(), None)?;
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
            let body = try!(make_request(reqwest::Method::Post, url, Some(params)));
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
