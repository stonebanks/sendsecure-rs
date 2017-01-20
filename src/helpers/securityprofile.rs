use helpers::value::Value;

#[derive(Debug, RustcDecodable)]
pub struct SecurityProfile {
    pub id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub created_at: Option<String>,
    pub update_at: Option<String>,
    pub allowed_login_attempts: Option<Value<i32>>,
    pub allow_remember_me: Option<Value<bool>>,
    pub allow_sms: Option<Value<bool>>,
    pub allow_voice: Option<Value<bool>>,
    pub allow_email: Option<Value<bool>>,
    pub code_time_limit: Option<Value<i32>>,
    pub code_length: Option<Value<i32>>,
    pub auto_extend_value: Option<Value<i32>>,
    pub auto_extend_unit: Option<Value<String>>,
    pub two_factor_required: Option<Value<bool>>,
    pub encrypt_attachments: Option<Value<bool>>,
    pub encrypt_message: Option<Value<bool>>,
    pub expiration_value: Option<Value<i32>>,
    pub expiration_unit: Option<Value<String>>,
    pub reply_enabled: Option<Value<bool>>,
    pub group_replies: Option<Value<bool>>,
    pub double_encryption: Option<Value<bool>>,
    pub retention_period_value: Option<Value<i32>>,
    pub retention_period_unit: Option<Value<String>>,
}
