use error::{SendSecureResult, SendSecureError};
use utils::requester::make_request;
use hyper::{header, method};
use url::Url;
use std::path::Path;
use std::io::Read;
use hyper::header::{Headers, ContentDisposition, DispositionParam, ContentType, DispositionType,
                    ContentLength, Accept, qitem};
use mime::{Mime, TopLevel, SubLevel, Attr, Value};
use mime_multipart;
use mime_guess;

#[derive(Debug)]
pub struct JsonClient {
    pub locale: String,
    pub enterprise_account: String,
    pub endpoint: String,
    pub api_token: String,
    pub sendsecure_url: Option<Url>,
}


pub trait UploadFileWithPath {
    fn upload_file(&mut self, upload_url: Url, file_path: &Path) -> SendSecureResult<String>;
}

pub trait UploadFileWithStream {
    fn upload_file(&mut self,
                   upload_url: Url,
                   stream: Vec<u8>,
                   file_name: &str,
                   file_size: u64)
                   -> SendSecureResult<String>;
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
            endpoint: endpoint.unwrap_or("https://portal.xmedius.com".to_string()),
            locale: locale.unwrap_or("en".to_string()),
            sendsecure_url: None,
        }
    }

    fn get_sendsecure_endpoint(&mut self) -> SendSecureResult<Option<Url>> {
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

    pub fn new_safebox(&mut self, user_email: &str) -> SendSecureResult<String> {
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
            Err(SendSecureError::UnexpectedError)
        }
    }

    pub fn security_profiles(&mut self, user_email: &str) -> SendSecureResult<String> {
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
            Err(SendSecureError::UnexpectedError)
        }
    }

    pub fn enterprise_settings(&mut self) -> SendSecureResult<String> {
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
            Err(SendSecureError::UnexpectedError)
        }
    }

    pub fn commit_safebox(&mut self, safebox_json: String) -> SendSecureResult<String> {
        let suffix = format!("api/v2/safeboxes?locale={}", self.locale);
        if let Some(sendsecure_endpoint) = self.get_sendsecure_endpoint()? {
            let mut headers = header::Headers::new();
            headers.set_raw("Authorization-Token",
                            vec![self.api_token.clone().into_bytes()]);
            let result = make_request(method::Method::Post,
                                      sendsecure_endpoint.join(suffix.as_str())?.as_str(),
                                      Some(safebox_json.as_str().as_bytes()),
                                      Some(headers))?;
            Ok(result)
        } else {
            Err(SendSecureError::UnexpectedError)
        }
    }
}

impl UploadFileWithPath for JsonClient {
    fn upload_file(&mut self, upload_url: Url, file_path: &Path) -> SendSecureResult<String> {
        let mut stream: Vec<u8> = vec![];
        let mut file = ::std::fs::File::open(file_path.clone())?;
        let size: u64 = ::std::io::copy(&mut file, &mut stream)?;

        UploadFileWithStream::upload_file(self,
                                          upload_url,
                                          stream,
                                          file_path.file_name()
                                              .and_then(|u| u.to_str())
                                              .unwrap(),
                                          size)
    }
}

impl UploadFileWithStream for JsonClient {
    fn upload_file(&mut self,
                   upload_url: Url,
                   stream: Vec<u8>,
                   file_name: &str,
                   file_size: u64)
                   -> SendSecureResult<String> {
        let mut output: Vec<u8> = Vec::new();
        let boundary = mime_multipart::generate_boundary();

        let part = mime_multipart::Part {
            headers: {
                let mut h = Headers::new();
                h.set(ContentType(mime_guess::guess_mime_type(file_name)));
                h.set(ContentDisposition {
                    disposition: DispositionType::Ext("form-data".to_owned()),
                    parameters: vec![DispositionParam::Ext("name".to_owned(), "file".to_owned()),
                                     DispositionParam::Ext("filename".to_owned(),
                                                           file_name.to_owned())],
                });
                h.set(ContentLength(file_size));
                h
            },
            body: stream,
        };
        let mut nodes: Vec<mime_multipart::Node> = Vec::new();
        nodes.push(mime_multipart::Node::Part(part));
        let msize = mime_multipart::write_multipart(&mut output, &boundary, &nodes)?;
        let mut headers = Headers::new();
        headers.set(Accept(vec![qitem(Mime(TopLevel::Application,
                                           SubLevel::Json,
                                           vec![(Attr::Charset, Value::Utf8)]))]));

        headers.set(ContentType(Mime(TopLevel::Multipart,
                                     SubLevel::FormData,
                                     vec![(Attr::Boundary,
                                           Value::Ext(String::from_utf8_lossy(&boundary)
                                               .into_owned()))])));
        headers.set(ContentLength(msize as u64));
        //let string = String::from_utf8_lossy(&output);
        let result = make_request(method::Method::Post,
                                  upload_url.as_str(),
                                  Some(&output[..]),
                                  Some(headers))?;
        Ok(result)
    }
}
