use error::SendSecure;
use utils::requester::{make_request, post_file};
use hyper::{header, method, client};
use url::Url;
use std::path::Path;
use std::io::Read;
use multipart::client::Multipart;
use mime::Mime;

#[derive(Debug)]
pub struct JsonClient {
    pub locale: String,
    pub enterprise_account: String,
    pub endpoint: String,
    pub api_token: String,
    pub sendsecure_url: Option<Url>,
}


pub trait UploadFileWithPath {
    fn upload_file(&mut self,
                   upload_url: Url,
                   file_path: &Path)
                   -> SendSecure::SendSecureResult<String>;
}

pub trait UploadFileWithStream {
    fn upload_file<St: Read>(&mut self,
                             upload_url: Url,
                             stream: &mut St,
                             content_type: Option<Mime>,
                             file_name: String)
                             -> SendSecure::SendSecureResult<String>;
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

    fn get_sendsecure_endpoint(&mut self) -> SendSecure::SendSecureResult<Option<Url>> {
        let formatted_url = format!("{}/services/{}/sendsecure/server/url",
                                    self.endpoint,
                                    self.enterprise_account);

        if self.sendsecure_url.is_none() {
            let default = make_request(method::Method::Get, formatted_url.as_str(), None, None)?;
            let default_url = Url::parse(default.as_str())?;
            self.sendsecure_url = Some(default_url);
        }
        Ok(self.sendsecure_url.to_owned())
    }

    pub fn new_safebox(&mut self, user_email: &str) -> SendSecure::SendSecureResult<String> {
        let suffix = format!("api/v2/safeboxes/new?user_email={}&locale={}",
                             user_email,
                             self.locale);
        if let Some(sendsecure_endpoint) = self.get_sendsecure_endpoint()? {
            let mut headers = header::Headers::new();
            headers.set_raw("Authorization-Token",
                            vec![self.api_token.clone().into_bytes()]);
            let result = make_request(method::Method::Get,
                                      sendsecure_endpoint.join(suffix.as_str())?.as_str(),
                                      None,
                                      Some(headers))?;
            Ok(result)
        } else {
            Err(SendSecure::SendSecureError::UnexpectedError)
        }
    }

    pub fn security_profiles(&mut self, user_email: &str) -> SendSecure::SendSecureResult<String> {
        let suffix = format!("api/v2/enterprises/{}/security_profiles?user_email={}&locale={}",
                             self.enterprise_account,
                             user_email,
                             self.locale);
        if let Some(sendsecure_endpoint) = self.get_sendsecure_endpoint()? {
            let mut headers = header::Headers::new();
            headers.set_raw("Authorization-Token",
                            vec![self.api_token.clone().into_bytes()]);
            let result = make_request(method::Method::Get,
                                      sendsecure_endpoint.join(suffix.as_str())?.as_str(),
                                      None,
                                      Some(headers))?;
            Ok(result)
        } else {
            Err(SendSecure::SendSecureError::UnexpectedError)
        }
    }

    pub fn enterprise_settings(&mut self) -> SendSecure::SendSecureResult<String> {
        let suffix = format!("api/v2/enterprises/{}/settings?locale={}",
                             self.enterprise_account,
                             self.locale);
        if let Some(sendsecure_endpoint) = self.get_sendsecure_endpoint()? {
            let mut headers = header::Headers::new();
            headers.set_raw("Authorization-Token",
                            vec![self.api_token.clone().into_bytes()]);
            let result = make_request(method::Method::Get,
                                      sendsecure_endpoint.join(suffix.as_str())?.as_str(),
                                      None,
                                      Some(headers))?;
            Ok(result)
        } else {
            Err(SendSecure::SendSecureError::UnexpectedError)
        }
    }
}

impl UploadFileWithPath for JsonClient {
    fn upload_file(&mut self,
                   upload_url: Url,
                   file_path: &Path)
                   -> SendSecure::SendSecureResult<String> {
        post_file(upload_url, |mut multipart| {
            try!(multipart.write_file("file", file_path));
            Ok(())
        })
    }
}

impl UploadFileWithStream for JsonClient {
    fn upload_file<St: Read>(&mut self,
                             upload_url: Url,
                             stream: &mut St,
                             content_type: Option<Mime>,
                             file_name: String)
                             -> SendSecure::SendSecureResult<String> {
        post_file(upload_url, |mut multipart| {
            try!(multipart.write_stream("file",
                                        stream,
                                        Some(file_name.as_str()),
                                        content_type.to_owned()));
            Ok(())
        })
    }
}
