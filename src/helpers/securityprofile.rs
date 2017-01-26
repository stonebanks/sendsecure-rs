use rustc_serialize::{Decodable, Decoder, Encodable, Encoder};
use helpers::value::Value;

#[derive(Debug, Clone)]
pub enum TimeUnit {
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

#[derive(Debug, Clone)]
pub enum RetentionPeriodType {
    DiscardAtExpiration,
    RetainAtExpiration,
    DoNotDiscard,
}

#[derive(Debug, RustcDecodable, Clone)]
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
    pub expiration_unit: Option<Value<TimeUnit>>,
    pub reply_enabled: Option<Value<bool>>,
    pub group_replies: Option<Value<bool>>,
    pub double_encryption: Option<Value<bool>>,
    pub retention_period_type: Option<Value<RetentionPeriodType>>,
    pub retention_period_value: Option<Value<i32>>,
    pub retention_period_unit: Option<Value<TimeUnit>>,
}

impl Decodable for TimeUnit {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<TimeUnit, D::Error> {
        decoder.read_enum("TimeUnit", |decoder| {
            decoder.read_enum_variant(&["hours", "days", "weeks", "months", "years"],
                                      |_, x| match x {
                                          0 => Result::Ok(TimeUnit::Hours),
                                          1 => Result::Ok(TimeUnit::Days),
                                          2 => Result::Ok(TimeUnit::Weeks),
                                          3 => Result::Ok(TimeUnit::Months),
                                          _ => Result::Ok(TimeUnit::Years),
                                      })
        })
    }
}

impl Decodable for RetentionPeriodType {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<RetentionPeriodType, D::Error> {
        decoder.read_enum("RetentionPeriodType", |decoder| {
            decoder.read_enum_variant(&["discard_at_expiration",
                                        "retain_at_expiration",
                                        "do_not_discard"],
                                      |_, x| match x {
                                          0 => Result::Ok(RetentionPeriodType::DiscardAtExpiration),
                                          1 => Result::Ok(RetentionPeriodType::RetainAtExpiration),
                                          _ => Result::Ok(RetentionPeriodType::DoNotDiscard),
                                      })
        })
    }
}

impl Encodable for TimeUnit {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_enum("TimeUnit", |s| match *self {
            TimeUnit::Hours => s.emit_enum_variant("hours", 0, 0, |_| Ok(())),
            TimeUnit::Days => s.emit_enum_variant("days", 1, 0, |_| Ok(())),
            TimeUnit::Months => s.emit_enum_variant("months", 2, 0, |_| Ok(())),
            TimeUnit::Weeks => s.emit_enum_variant("weeks", 3, 0, |_| Ok(())),
            TimeUnit::Years => s.emit_enum_variant("years", 4, 0, |_| Ok(())),
        })
    }
}

impl Encodable for RetentionPeriodType {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_enum("RetentionPeriodType", |s| match *self {
            RetentionPeriodType::DiscardAtExpiration => {
                s.emit_enum_variant("discard_at_expiration", 0, 0, |_| Ok(()))
            }
            RetentionPeriodType::RetainAtExpiration => {
                s.emit_enum_variant("retain_at_expiration", 1, 0, |_| Ok(()))
            }
            RetentionPeriodType::DoNotDiscard => {
                s.emit_enum_variant("do_not_discard", 2, 0, |_| Ok(()))
            }
        })
    }
}
