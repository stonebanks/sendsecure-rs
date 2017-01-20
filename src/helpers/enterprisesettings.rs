use rustc_serialize::{Decodable, Decoder};
use std::result::Result;
use helpers::value::Value;

#[derive(Debug)]
pub enum Mode {
    Allow,
    Forbid,
}

#[derive(Debug, RustcDecodable)]
pub struct ExtensionFilter {
    pub mode: Option<Mode>,
    pub list: Vec<String>,
}

#[derive(Debug, RustcDecodable)]
pub struct EnterpriseSettings {
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub default_security_profile_id: Option<i32>,
    pub pdf_language: Option<String>,
    pub use_pdfa_audit_records: Option<bool>,
    pub international_dialing_plan: Option<String>,
    pub extension_filter: Option<ExtensionFilter>,
    pub include_users_in_autocomplete: Option<bool>,
    pub include_favorites_in_autocomplete: Option<bool>,
}

impl Decodable for Mode {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<Mode, D::Error> {
        decoder.read_enum("Mode", |decoder| {
            decoder.read_enum_variant(&["allow", "forbid"], |_, x| match x {
                0 => Result::Ok(Mode::Allow),
                _ => Result::Ok(Mode::Forbid),
            })
        })
    }
}
