#[derive(RustcDecodable)]
pub struct NewSafebox {
    pub guid: String,
    pub public_encryption_key: String,
    pub upload_url: String,
}
