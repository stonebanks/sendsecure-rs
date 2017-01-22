#[derive(Debug, RustcDecodable)]
pub struct SafeboxResponse {
    pub guid: String,
    pub preview_url: String,
    pub encryption_key: String,
}
