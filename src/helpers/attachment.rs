use std::path::Path;
use error::SendSecureResult;
use std::ffi::OsStr;
use std::fs::metadata;

#[derive(Debug, Clone)]
pub struct Attachment<'a> {
    pub guid: Option<String>,
    pub file_name: Option<&'a OsStr>,
    pub size: u64,
    pub file_path: &'a Path,
}


impl<'a> Attachment<'a> {
    pub fn new(path: &Path) -> SendSecureResult<Attachment> {
        let metadata = metadata(path)?;
        Ok(Attachment {
            file_name: path.file_name(),
            guid: None,
            size: metadata.len(),
            file_path: path,
        })
    }

    pub fn into_guid(self) -> String {
        self.guid.unwrap_or_else(String::new)
    }
}
