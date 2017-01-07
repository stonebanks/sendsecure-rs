use rustc_serialize::{Decodable, Decoder};
use std::result::Result;
use helpers::value::Value;

#[derive(Debug)]
pub enum Mode {
    Allow,
    Forbid,
}

#[derive(Debug)]
pub struct ExtensionFilter {
    pub mode: Option<Mode>,
    pub list: Vec<String>,
}

#[derive(Debug)]
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
            decoder.read_enum_variant(&["allow", "forbid"], |decoder, x| {
                match x {
                    0 => Result::Ok(Mode::Allow),
                    _ => Result::Ok(Mode::Forbid),
                }
            })
        })
    }
}

impl Decodable for ExtensionFilter {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<ExtensionFilter, D::Error> {
        decoder.read_struct("root", 0, |decoder| {
            Ok(ExtensionFilter {
                mode: decoder.read_struct_field("mode", 0, |decoder| Decodable::decode(decoder))
                    .unwrap_or(None),
                list: decoder.read_struct_field("list", 0, |decoder| Decodable::decode(decoder))
                    .unwrap_or(vec![]),
            })
        })
    }
}

impl Decodable for EnterpriseSettings {
    fn decode<D: Decoder>(decoder: &mut D) -> Result<EnterpriseSettings, D::Error> {
        decoder.read_struct("root", 0, |decoder| {
            Ok(EnterpriseSettings {
                created_at:
                    decoder.read_struct_field("created_at", 0, |decoder| Decodable::decode(decoder))
                    .unwrap_or(None),
                updated_at:
                    decoder.read_struct_field("update_at", 0, |decoder| Decodable::decode(decoder))
                    .unwrap_or(None),
                default_security_profile_id: decoder.read_struct_field("default_security_profile_id", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                pdf_language: decoder.read_struct_field("pdf_language", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                use_pdfa_audit_records: decoder.read_struct_field("use_pdfa_audit_records", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                international_dialing_plan: decoder.read_struct_field("international_dialing_plan", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                extension_filter: decoder.read_struct_field("extension_filter", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                include_users_in_autocomplete: decoder.read_struct_field("include_users_in_autocomplete", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
                include_favorites_in_autocomplete: decoder.read_struct_field("include_favorites_in_autocomplete", 0, |decoder| Decodable::decode(decoder)).unwrap_or(None),
            })
        })
    }
}
