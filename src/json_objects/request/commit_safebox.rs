use helpers::{safebox, recipient, attachment};
use helpers::securityprofile::{RetentionPeriodType, TimeUnit};

#[derive(RustcEncodable)]
pub struct Safebox {
    pub guid: Option<String>,
    pub recipients: Option<Vec<recipient::Recipient>>,
    pub subject: Option<String>,
    pub message: Option<String>,
    pub document_ids: Option<Vec<String>>,
    pub security_profile_id: Option<i32>,
    pub reply_enabled: Option<bool>,
    pub group_replies: Option<bool>,
    pub expiration_value: Option<i32>,
    pub expiration_unit: Option<TimeUnit>,
    pub retention_period_type: Option<RetentionPeriodType>,
    pub retention_period_value: Option<i32>,
    pub retention_period_unit: Option<TimeUnit>,
    pub encrypt_message: Option<bool>,
    pub double_encryption: Option<bool>,
    pub public_encryption_key: Option<String>,
    pub notification_language: String,
}

#[derive(RustcEncodable)]
pub struct CommitSafebox {
    pub safebox: Safebox,
}

impl CommitSafebox {
    pub fn new(safebox: safebox::Safebox) -> CommitSafebox {
        let security_profile = safebox.security_profile.as_ref();
        CommitSafebox {
            safebox: Safebox {
                guid: safebox.guid,
                recipients: safebox.recipients,
                subject: safebox.subject,
                message: safebox.message,
                document_ids: safebox.attachments.map(|attachments| {
                    attachments.into_iter()
                        .map(attachment::Attachment::into_guid)
                        .collect()
                }),
                security_profile_id: security_profile
                    .map(|security_profile| security_profile.id),
                reply_enabled: security_profile
                    .and_then(|sp| sp.reply_enabled.as_ref().and_then(|re| re.value)),
                group_replies: safebox.security_profile
                    .as_ref()
                    .and_then(|sp| sp.group_replies.as_ref().and_then(|re| re.value)),
                expiration_value: security_profile.and_then(|sp| sp.expiration_value.as_ref())
                    .and_then(|re| re.value),
                expiration_unit: security_profile.and_then(|sp| sp.expiration_unit.as_ref()).and_then(|re| re.value.to_owned()),
                retention_period_type: security_profile.and_then(|sp| sp.retention_period_type.as_ref()).and_then(|re| re.value.to_owned()),
                retention_period_value: security_profile.and_then(|sp| sp.retention_period_value.as_ref()).and_then(|re| re.value),
                retention_period_unit: security_profile.and_then(|sp| sp.retention_period_unit.as_ref()).and_then(|re| re.value.to_owned()),
                encrypt_message: security_profile.and_then(|sp| sp.encrypt_message.as_ref()).and_then(|re| re.value),
                double_encryption: security_profile.and_then(|sp| sp.double_encryption.as_ref()).and_then(|re| re.value),
                public_encryption_key: safebox.public_encryption_key,
                notification_language: safebox.notification_language,
            },
        }
    }
}
