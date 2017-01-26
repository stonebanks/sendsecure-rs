use std::path::Path;
use error::SendSecureResult;
use std::ffi::OsStr;
use std::fs::metadata;
use mime::Mime;

#[derive(Debug, Clone)]
pub struct Attachment<'a> {
    pub guid: Option<String>,
    pub file_name: Option<&'a OsStr>,
    pub content_type: Mime,
    pub size: u64,
    pub file_path: &'a Path,
}


impl<'a> Attachment<'a> {
    pub fn new(path: &Path, content_type: Option<Mime>) -> SendSecureResult<Attachment> {
        let metadata = metadata(path)?;
        Ok(Attachment {
            file_name: path.file_name(),
            guid: None,
            size: metadata.len(),
            file_path: path,
            content_type: content_type.unwrap_or("application/octet-stream".parse().unwrap()),
        })
    }

    pub fn into_guid(self) -> String {
        self.guid.unwrap_or_else(String::new)
    }
}
