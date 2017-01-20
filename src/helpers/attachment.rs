use std::fs::File;
use std::path::Path;
use error::{SendSecureResult, SendSecureError};
use std::ffi::OsStr;

#[derive(Debug)]
pub struct Attachment<'a> {
    pub guid: Option<String>,
    pub file_name: Option<&'a OsStr>,
    pub content_type: String,
    pub size: u32,
    pub file: File,
}


impl<'a> Attachment<'a> {
    pub fn new(path: &Path, content_type: Option<String>) -> SendSecureResult<Attachment> {
        let file = try!(File::open(path));
        Ok(Attachment {
            file_name: path.file_name(),
            guid: None,
            size: 0,
            file: file,
            content_type: match content_type {
                Some(x) => x,
                None => "application/octet-stream".to_string(),
            },
        })
    }
}
