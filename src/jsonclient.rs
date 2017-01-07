use error::SendSecure;
use utils::requester::make_request;
use reqwest;

pub struct JsonClient {
    pub locale: String,
    pub enterprise_account: String,
    pub endpoint: String,
    pub api_token: String,
    pub sendsecure_url: Option<String>,
}


impl JsonClient {
    pub fn new(api_token: String,
               enterprise_account: String,
               endpoint: Option<String>,
               locale: Option<String>)
               -> JsonClient {
        JsonClient {
            api_token: api_token,
            enterprise_account: enterprise_account,
            endpoint: match endpoint {
                Some(x) => x,
                None => "https://portal.xmedius.com".to_string(),
            },
            locale: match locale {
                Some(x) => x,
                None => "en".to_string(),
            },
            sendsecure_url: None,
        }
    }

    fn get_sendsecure_endpoint(&mut self) -> SendSecure::SendSecureResult<Option<&str>> {
        let formatted_url = format!("{}/services/{}/sendsecure/server/url",
                                    self.endpoint,
                                    self.enterprise_account);

        if self.sendsecure_url.is_none() {
            let default = make_request(reqwest::Method::Get, formatted_url.as_str(), None)?;
            self.sendsecure_url = Some(default);
        }
        Ok(self.sendsecure_url.as_ref().map(String::as_str))
    }

    pub fn new_safebox(&self, user_email: String) {
        let suffix = format!("api/v2/safeboxes/new?user_email={}&locale={}",
                             user_email,
                             self.locale);

    }
}
