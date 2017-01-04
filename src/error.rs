pub mod SendSecure {
    use std::io;
    use reqwest;
    // use hyper::{client, error, status};
    use rustc_serialize::json;
    // We derive `Debug` because all types should probably derive `Debug`.
    // This gives us a reasonable human readable description of `SendSecureError` value
    #[derive(Debug)]
    pub enum SendSecureError {
        ClientInitializationError(reqwest::Error),
        RequestBuilderError(reqwest::Error),
        ResponseError(reqwest::StatusCode),
        JSONSerializeError(json::BuilderError),
        JSONDecoderError(json::DecoderError),
        UnexpectedResponseError(String),
        IoError(io::Error),
    }
    pub type SendSecureResult<T> = Result<T, SendSecureError>;


    impl From<io::Error> for SendSecureError {
        fn from(err: io::Error) -> SendSecureError {
            SendSecureError::IoError(err)
        }
    }

    impl From<reqwest::Error> for SendSecureError {
        fn from(err: reqwest::Error) -> SendSecureError {
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
}
