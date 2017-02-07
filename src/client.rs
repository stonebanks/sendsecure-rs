use rustc_serialize::json::Json;
use std::option::Option;
use std::collections::HashMap;
use hyper::method;
use error::{SendSecureError, SendSecureResult};
use utils::requester::make_request;
use rustc_serialize::json;
use jsonclient::{JsonClient, UploadFileWithPath};
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
                          endpoint: Option<&str>,
                          one_time_password: Option<bool>)
                          -> SendSecureResult<String> {
        let formatted_url = format!("{0}/services/{1}/portal/host",
                                    endpoint.unwrap_or("https://portal.xmedius.com"),
                                    enterprise_account);
        let mut url = make_request(method::Method::Get, formatted_url.as_str(), None, None)?;
        url = format!("{0}api/user_token", url);
        let mut params = HashMap::new();
        params.insert("permalink", enterprise_account);
        params.insert("username", username);
        params.insert("password", password);
        params.insert("application_type", application_type);
        params.insert("device_id", device_id);
        params.insert("device_name", device_name);
        if one_time_password.unwrap_or(false) {
            params.insert("otp", "true");
        }
        let test = json::encode(&params)?;
        let body = make_request(method::Method::Post,
                                url.as_str(),
                                Some(test.as_str().as_bytes()),
                                None)?;
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
            endpoint: endpoint.unwrap_or("https://portal.xmedius.com").to_string(),
            locale: locale.unwrap_or("en").to_string(),
            jsonclient: JsonClient::new(api_token, enterprise_account, endpoint, locale),
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

        {
            let upload_url = safebox_output.upload_url.as_ref().map(String::as_str).unwrap_or("");
            let mut attachments_out: Vec<attachment::Attachment> = vec![];
            for elem in safebox.attachments.iter_mut() {
                let attachment = self.upload_attachement(upload_url, elem)?;
                attachments_out.push(attachment);
            }
            safebox_output.attachments = attachments_out;
        }

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
                                      attachment: &mut attachment::Attachment<'b>)
                                      -> SendSecureResult<attachment::Attachment<'b>> {
        let upload_url = Url::parse(upload_url)?;
        let response = self.jsonclient
            .upload_file(upload_url, attachment.file_path)?;
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
