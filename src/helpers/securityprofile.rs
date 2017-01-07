use rustc_serialize::{Decodable, Decoder};
use std::result::Result;
use helpers::value::Value;

#[derive(Debug)]
pub struct SecurityProfile {
    pub id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub update_at: Option<String>,
    pub allowed_login_attempts: Option<Value>,
    pub allow_remember_me: Option<Value>,
    pub allow_sms: Option<Value>,
    pub allow_voice: Option<Value>,
    pub allow_email: Option<Value>,
    pub code_time_limit: Option<Value>,
    pub code_length: Option<Value>,
    pub auto_extend_value: Option<Value>,
    pub auto_extend_unit: Option<Value>,
    pub two_factor_required: Option<Value>,
    pub encrypt_attachments: Option<Value>,
    pub encrypt_message: Option<Value>,
    pub expiration_value: Option<Value>,
    pub expiration_unit: Option<Value>,
    pub reply_enabled: Option<Value>,
    pub group_replies: Option<Value>,
    pub double_encryption: Option<Value>,
    pub retention_period_value: Option<Value>,
    pub retention_period_unit: Option<Value>,
}

impl Decodable for SecurityProfile {
    // 'static cf. http://stackoverflow.com/a/32644068
    fn decode<D: Decoder>(decoder: &mut D) -> Result<SecurityProfile, D::Error> {
        decoder.read_struct("root", 0, |decoder| {
            Ok(SecurityProfile {
                id: decoder.read_struct_field("id", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                name: decoder.read_struct_field("name", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                description: decoder.read_struct_field("description", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                created_at: decoder.read_struct_field("created_at", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                update_at: decoder.read_struct_field("update_at", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                allowed_login_attempts: decoder.read_struct_field("allowed_login_attempts",
                                       0,
                                       |decoder| Decodable::decode(decoder))
                    .unwrap_or(None),
                allow_remember_me: decoder.read_struct_field("allow_remember_me", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                allow_sms: decoder.read_struct_field("allow_sms", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                allow_voice: decoder.read_struct_field("allow_voice", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                allow_email: decoder.read_struct_field("allow_email", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                code_time_limit: decoder.read_struct_field("code_time_limit", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                code_length: decoder.read_struct_field("code_length", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                auto_extend_value: decoder.read_struct_field("auto_extend_value", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                auto_extend_unit: decoder.read_struct_field("auto_extend_unit", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                two_factor_required: decoder.read_struct_field("two_factor_required", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                encrypt_attachments: decoder.read_struct_field("encrypt_attachments", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                encrypt_message: decoder.read_struct_field("encrypt_message", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                expiration_value: decoder.read_struct_field("expiration_value", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                expiration_unit: decoder.read_struct_field("expiration_unit", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                reply_enabled: decoder.read_struct_field("reply_enabled", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                group_replies: decoder.read_struct_field("group_replies", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                double_encryption: decoder.read_struct_field("double_encryption", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                retention_period_value: decoder.read_struct_field("retention_period_value", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                retention_period_unit: decoder.read_struct_field("retention_period_unit", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
            })
        })
    }
}
