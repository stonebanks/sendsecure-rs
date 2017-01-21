use helpers::securityprofile::SecurityProfile;
use helpers::attachment::Attachment;
use helpers::recipient::Recipient;

// struct CommitSafeboxRequest {
//     pub safebox: CommitSafebox,
// }

#[derive(Debug)]
pub struct Safebox<'a> {
    pub guid: Option<String>,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub security_profile: Option<SecurityProfile>,
    pub upload_url: Option<String>,
    pub user_email: String,
    pub public_encryption_key: Option<String>,
    pub attachments: Option<Vec<Attachment<'a>>>,
    pub recipients: Option<Vec<Recipient>>,
}


impl<'a> Safebox<'a> {
    pub fn new(user_email: String) -> Safebox<'a> {
        Safebox {
            user_email: user_email,
            guid: None,
            subject: None,
            message: None,
            attachments: None,
            recipients: None,
            public_encryption_key: None,
            upload_url: None,
            security_profile: None,
        }
    }

    //pub fn to_json() -> String {}
}
