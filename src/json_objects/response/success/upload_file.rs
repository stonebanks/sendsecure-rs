#[derive(Debug, RustcDecodable)]
pub struct TemporaryDocument {
    pub document_guid: String,
}

#[derive(Debug, RustcDecodable)]
pub struct UploadFile {
    pub temporary_document: TemporaryDocument,
}
