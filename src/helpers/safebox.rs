use helpers::securityprofile::SecurityProfile;
use helpers::attachment::Attachment;
use helpers::recipient::Recipient;

// struct CommitSafeboxRequest {
//     pub safebox: CommitSafebox,
// }

#[derive(Debug, Clone)]
pub struct Safebox<'a> {
    pub guid: Option<String>,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub security_profile: Option<SecurityProfile>,
    pub upload_url: Option<String>,
    pub user_email: String,
    pub public_encryption_key: Option<String>,
    pub attachments: Vec<Attachment<'a>>,
    pub recipients: Vec<Recipient>,
    pub notification_language: String,
}


impl<'a> Safebox<'a> {
    pub fn new(user_email: &str) -> Safebox<'a> {
        Safebox {
            user_email: user_email.to_string(),
            guid: None,
            subject: None,
            message: None,
            attachments: vec![],
            recipients: vec![],
            public_encryption_key: None,
            upload_url: None,
            security_profile: None,
            notification_language: "en".to_string(),
        }
    }

    // pub fn to_json() -> String {}
}
