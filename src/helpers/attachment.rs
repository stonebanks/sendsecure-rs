use std::fs::File;
use std::path::Path;
use error::{SendSecureResult, SendSecureError};
use std::ffi::OsStr;
use std::fs::metadata;
use mime::Mime;

#[derive(Debug)]
pub struct Attachment<'a> {
    pub guid: Option<String>,
    pub file_name: Option<&'a OsStr>,
    pub content_type: Mime,
    pub size: u64,
    pub file: File,
}


impl<'a> Attachment<'a> {
    pub fn new(path: &Path, content_type: Option<Mime>) -> SendSecureResult<Attachment> {
        let file = File::open(path)?;
        let metadata = metadata(path)?;
        Ok(Attachment {
            file_name: path.file_name(),
            guid: None,
            size: metadata.len(),
            file: file,
            content_type: match content_type {
                Some(x) => x,
                None => "application/octet-stream".parse().unwrap(),
            },
        })
    }

    pub fn into_guid(self) -> String {
        self.guid.unwrap_or_else(String::new)
    }
}
