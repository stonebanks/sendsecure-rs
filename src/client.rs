use rustc_serialize::json::Json;
use std::option::Option;
use std::collections::HashMap;
use hyper::{method, client};
use error::{SendSecureError, SendSecureResult};
use utils::requester::make_request;
use rustc_serialize::json;
use jsonclient::{JsonClient, UploadFileWithStream};
use helpers::{safebox, securityprofile, enterprisesettings, safeboxresponse, attachment};
use json_objects::{response, request};
use url::Url;

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
                          -> SendSecureResult<String> {
        let formatted_url = format!("{0}/services/{1}/portal/host", endpoint, enterprise_account);
        println!("{}", formatted_url);
        let mut url = make_request(method::Method::Get, formatted_url.as_str(), None, None)?;
        url = format!("{0}api/user_token", url);
        // println!("{}", url);
        // let url: &str = "http://httpbin.org/post";//"https://secure.bixi.com/data/stations.json";
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
        let body = make_request(method::Method::Post, url.as_str(), Some(test), None)?;
        println!("{}", body);
        let json_body = Json::from_str(body.as_str())?;
        if let Some(obj) = json_body.as_object() {
            if let Some(origin) = obj.get("token") {
                return Ok(origin.as_string().unwrap_or_default().to_string());
            }
        }
        Err(SendSecureError::UnexpectedResponseError(body))
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

    pub fn submit_safebox<'b, 'a>(&'b mut self,
                                  safebox: &'b mut safebox::Safebox<'a>)
                                  -> SendSecureResult<safeboxresponse::SafeboxResponse> {
        self.initialize_safebox(safebox)?;
        // if safebox.attachments.is_some() {
        //     for attachment in safebox.attachments.as_ref().unwrap().iter_mut() {
        //         self.upload_attachement(safebox, attachment);
        //     }
        // }
        safebox.attachments.iter_mut().next().map(|s| {
            let iterator = s.iter_mut();
            for elem in iterator {
                self.upload_attachement(safebox, elem);
                //(*elem).guid = Some("gg".to_string());
            }
        });
        // .map(|attachments| {
        //     for attachment in attachments.iter_mut() {
        //         self.upload_attachement(safebox, attachment);
        //     }
        //     // safebox.attachments
        //     Vec::<u32>::new()
        // });
        // match safebox.attachments {
        //     Some(ref mut attachments) => {
        //         for attachment in attachments.iter_mut() {
        //             self.upload_attachement(safebox, attachment);
        //         }
        //     }
        //     None => safebox.attachments = Some(vec![]),
        // }
        // for attachment in safebox.attachments


        unimplemented!()
    }

    pub fn initialize_safebox<'b, 'a>(&'b mut self,
                                      safebox: &'b mut safebox::Safebox<'a>)
                                      -> SendSecureResult<&'b mut safebox::Safebox<'a>> {
        let temp = self.jsonclient
            .new_safebox(safebox.user_email.as_str())?;
        let response: response::success::new_safebox::NewSafebox = json::decode(&temp)?;
        safebox.guid = Some(response.guid);
        safebox.public_encryption_key = Some(response.public_encryption_key);
        safebox.upload_url = Some(response.upload_url);
        Ok(safebox)
    }

    pub fn upload_attachement<'b, 'a>(&'b mut self,
                                      safebox: &mut safebox::Safebox,
                                      attachment: &'b mut attachment::Attachment<'a>)
                                      -> SendSecureResult<&'b mut attachment::Attachment<'a>> {
        let upload_url = Url::parse(safebox.upload_url.as_ref().map(String::as_str).unwrap_or(""))?;
        let response = self.jsonclient
            .upload_file(upload_url,
                         &mut attachment.file,
                         attachment.content_type.to_owned(),
                         attachment.file_name
                             .and_then(|s| s.to_str())
                             .map(|s| s.to_string())
                             .unwrap())?;
        let response_object: response::success::upload_file::UploadFile =
            json::decode(&response.as_str())?;
        attachment.guid = Some(response_object.temporary_document.document_guid);
        Ok(attachment)
    }

    pub fn commit_safebox(&mut self,
                          safebox: safebox::Safebox)
                          -> SendSecureResult<safeboxresponse::SafeboxResponse> {
        let commit_safebox = request::commit_safebox::CommitSafebox::new(safebox);
        let request = json::encode(&commit_safebox)?;
        let string = self.jsonclient.commit_safebox(request)?;
        let response: safeboxresponse::SafeboxResponse = json::decode(&string)?;
        Ok(response)
    }

    pub fn security_profiles(&mut self,
                             user_email: &str)
                             -> SendSecureResult<Vec<securityprofile::SecurityProfile>> {
        let string = self.jsonclient.security_profiles(user_email)?;
        let response: response::success::security_profiles::SecurityProfiles =
            json::decode(&string)?;
        Ok(response.security_profiles)
    }

    pub fn enterprise_settings(&mut self)
                               -> SendSecureResult<enterprisesettings::EnterpriseSettings> {
        let string = self.jsonclient.enterprise_settings()?;
        let response: enterprisesettings::EnterpriseSettings = json::decode(&string)?;
        Ok(response)
    }

    pub fn default_security_profile
        (&mut self,
         user_email: &str)
         -> SendSecureResult<Option<securityprofile::SecurityProfile>> {
        let securityprofiles = self.security_profiles(user_email)?;
        let enterprisesettings = self.enterprise_settings()?;
        let result = securityprofiles.iter()
            .find(|ref securityprofile| if let Some(x) =
                enterprisesettings.default_security_profile_id {
                return securityprofile.id == x;
            } else {
                return false;
            });
        Ok(result.cloned())
    }
}
