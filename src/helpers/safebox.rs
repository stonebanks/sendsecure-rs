use helpers::securityprofile::SecurityProfile;
use helpers::attachment::Attachment;
use helpers::recipient::Recipient;

#[derive(Debug)]
pub struct Safebox<'a> {
    pub guid: Option<String>,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub security_profile: Option<SecurityProfile>,
    pub upload_url: Option<String>,
    pub user_email: String,
    pub public_encryption_key: Option<String>,
    pub attachments: Vec<Attachment<'a>>,
    pub recipient: Vec<Recipient>,
}


impl<'a> Safebox<'a> {
    fn new(user_email: String) -> Safebox<'a> {
        Safebox {
            user_email: user_email,
            guid: None,
            subject: None,
            message: None,
            attachments: vec![],
            recipient: vec![],
            public_encryption_key: None,
            upload_url: None,
            security_profile: None,
        }
    }
}
