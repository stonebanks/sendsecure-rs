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
use std::fs::File;

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

    pub fn submit_safebox<'a, 'b>(&'a mut self,
                                  safebox: &'b mut safebox::Safebox<'b>)
                                  -> SendSecureResult<safeboxresponse::SafeboxResponse> {
        let mut safebox_output = self.initialize_safebox(safebox)?;
        if safebox_output.security_profile.is_none() {
            let result = self.default_security_profile(safebox_output.user_email.as_str())?;
            safebox_output.security_profile = result;
        }


        let upload_url = safebox_output.upload_url.as_ref().map(String::as_str).unwrap_or("");
        let mut temp: Vec<attachment::Attachment> = vec![];
        safebox.attachments.as_ref().map(|s| for elem in s.iter() {
            let attachment = self.upload_attachement(upload_url, elem).unwrap();
            temp.push(attachment);
        });
        safebox_output.attachments = Some(temp);


        return self.commit_safebox(safebox_output);
    }

    pub fn initialize_safebox<'a, 'b>(&'a mut self,
                                      safebox: &'a mut safebox::Safebox<'b>)
                                      -> SendSecureResult<safebox::Safebox<'b>> {
        let temp = self.jsonclient
            .new_safebox(safebox.user_email.as_str())?;
        let response: response::success::new_safebox::NewSafebox = json::decode(&temp)?;
        safebox.guid = Some(response.guid);
        safebox.public_encryption_key = Some(response.public_encryption_key);
        safebox.upload_url = Some(response.upload_url);
        Ok(safebox.clone())
    }

    pub fn upload_attachement<'a, 'b>(&'a mut self,
                                      upload_url: &str,
                                      attachment: &attachment::Attachment<'b>)
                                      -> SendSecureResult<attachment::Attachment<'b>> {
        let upload_url = Url::parse(upload_url)?;
        let mut file = File::open(attachment.file_path)?;
        let response = self.jsonclient
            .upload_file(upload_url,
                         &mut file,
                         attachment.content_type.to_owned(),
                         attachment.file_name
                             .and_then(|s| s.to_str())
                             .map(|s| s.to_string())
                             .unwrap())?;
        let response_object: response::success::upload_file::UploadFile =
            json::decode(&response.as_str())?;
        let mut attachment_output = attachment.clone();
        attachment_output.guid = Some(response_object.temporary_document.document_guid);
        Ok(attachment_output.clone())
    }

    pub fn commit_safebox(&mut self,
                          safebox: safebox::Safebox)
                          -> SendSecureResult<safeboxresponse::SafeboxResponse> {
        let test = safebox.clone();
        let commit_safebox = request::commit_safebox::CommitSafebox::new(test);
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
