use std::io;
use url::ParseError;
use hyper::{error, status};
use rustc_serialize::json;
// We derive `Debug` because all types should probably derive `Debug`.
// This gives us a reasonable human readable description of `SendSecureError` value
#[derive(Debug)]
pub enum SendSecureError {
    RequestBuilderError(error::Error),
    ResponseError(status::StatusCode),
    JSONSerializeError(json::BuilderError),
    JSONDecoderError(json::DecoderError),
    JSONEncoderError(json::EncoderError),
    UnexpectedResponseError(String),
    IoError(io::Error),
    UrlError(ParseError),
    UnexpectedError,
}
pub type SendSecureResult<T> = Result<T, SendSecureError>;


impl From<io::Error> for SendSecureError {
    fn from(err: io::Error) -> SendSecureError {
        SendSecureError::IoError(err)
    }
}

impl From<error::Error> for SendSecureError {
    fn from(err: error::Error) -> SendSecureError {
        SendSecureError::RequestBuilderError(err)
    }
}

impl From<json::BuilderError> for SendSecureError {
    fn from(err: json::BuilderError) -> SendSecureError {
        SendSecureError::JSONSerializeError(err)
    }
}

impl From<json::DecoderError> for SendSecureError {
    fn from(err: json::DecoderError) -> SendSecureError {
        SendSecureError::JSONDecoderError(err)
    }
}

impl From<json::EncoderError> for SendSecureError {
    fn from(err: json::EncoderError) -> SendSecureError {
        SendSecureError::JSONEncoderError(err)
    }
}

impl From<ParseError> for SendSecureError {
    fn from(err: ParseError) -> SendSecureError {
        SendSecureError::UrlError(err)
    }
}
