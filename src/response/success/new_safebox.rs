#[derive(RustcDecodable)]
pub struct newSafebox {
    pub guid: String,
    pub public_encryption_key: String,
    pub upload_url: String,
}
