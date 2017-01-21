use helpers::{safebox, recipient};

#[derive(RustcEncodable)]
pub struct Safebox {
    pub guid: Option<String>,
    pub recipients: Vec<recipient::Recipient>,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub document_ids: Option<Vec<Option<String>>>,
    pub security_profile_id: Option<i32>,
    pub reply_enabled: Option<bool>,
    pub group_replies: Option<bool>,
    pub expiration_value: i32,
    pub expiration_unit: String,
    pub retention_period_type: String,
    pub retention_period_value: Option<i32>,
    pub retention_period_unit: Option<String>,
    pub encrypt_message: bool,
    pub double_encryption: bool,
    pub public_encryption_key: String,
    pub notification_language: String,
}

#[derive(RustcEncodable)]
pub struct CommitSafebox {
    pub safebox: Safebox,
}

impl CommitSafebox {
    pub fn new(safebox: safebox::Safebox) -> CommitSafebox {
        CommitSafebox {
            safebox: Safebox {
                guid: safebox.guid,
                recipients: match safebox.recipients {
                    Some(recipients) => recipients,
                    None => vec![],
                },
                subject: safebox.subject,
                message: safebox.message,
                document_ids: match safebox.attachments {
                    Some(attachments) => {
                        attachments.iter()
                            .map(|&attachment| attachment.guid.unwrap_or(String::new()))
                            .collect()
                    }
                    None => None,
                },
                security_profile_id: match safebox.security_profile {
                    Some(security_profile) => security_profile.id,
                    None => None,
                },
                reply_enabled: match safebox.security_profile {
                    Some(security_profile) => {
                        if let Some(value) = security_profile.reply_enabled {
                            return value.value;
                        } else {
                            return None;
                        }
                    }
                    None => None,
                },
            },
        }
    }
}
