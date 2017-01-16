pub mod SendSecure {

    use std::cmp::Ordering;
    use rustc_serialize::json::Json;
    use std::option::Option;
    use std::collections::HashMap;
    use hyper::{method, client};
    use std::io::Read;
    use error::SendSecure;
    use utils::requester::make_request;
    use rustc_serialize::json;
    use jsonclient::JsonClient;
    use helpers::safebox;
    use response::success::new_safebox;

    pub struct Client {
        jsonclient: JsonClient,
        pub api_token: String,
        pub enterprise_account: String,
        pub endpoint: String,
        pub locale: String,
    }

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
            let uurl = make_request(method::Method::Get, formatted_url.as_str(), None, None)?;
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
            let test = json::encode(&params)?;
            let body = make_request(method::Method::Post, url, Some(test), None)?;
            println!("{}", body);
            let json_body = Json::from_str(body.as_str())?;
            if let Some(obj) = json_body.as_object() {
                if let Some(origin) = obj.get("origin") {
                    return Ok(origin.as_string().unwrap_or_default().to_string());
                }
            }
            Err(SendSecure::SendSecureError::UnexpectedResponseError(body))
        }

        pub fn new(api_token: &str,
                   enterprise_account: &str,
                   endpoint: Option<&str>,
                   locale: Option<&str>)
                   -> Client {
            Client {
                api_token: api_token.to_string(),
                enterprise_account: enterprise_account.to_string(),
                endpoint: match endpoint {
                    Some(x) => x.to_string(),
                    None => "https://portal.xmedius.com".to_string(),
                },
                locale: match locale {
                    Some(x) => x.to_string(),
                    None => "en".to_string(),
                },
                jsonclient: JsonClient::new(api_token.to_string(),
                                            enterprise_account.to_string(),
                                            endpoint.map(|s| s.to_string()),
                                            locale.map(|s| s.to_string())),
            }
        }

        pub fn initialize_safebox<'b, 'a>
            (&'b mut self,
             safebox: &'b mut safebox::Safebox<'a>)
             -> SendSecure::SendSecureResult<&mut safebox::Safebox<'a>> {
            let temp = self.jsonclient
                .new_safebox(safebox.user_email.as_str())?;
            let response: new_safebox::newSafebox = json::decode(&temp)?;
            safebox.guid = Some(response.guid);
            safebox.public_encryption_key = Some(response.public_encryption_key);
            safebox.upload_url = Some(response.upload_url);
            Ok(safebox)
        }
    }
}
