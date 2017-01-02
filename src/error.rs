pub mod SendSecure {
    use std::io;
    use hyper::{client,error,status};
    use rustc_serialize::json;
    // We derive `Debug` because all types should probably derive `Debug`.
    // This gives us a reasonable human readable description of `SendSecureError` value
    #[derive(Debug)]
    pub enum SendSecureError {
        UnexpectedServerError(error::Error),
        ResponseError(status::StatusCode),
        JSONSerializeError(json::BuilderError),
        UnexpectedResponseError(String),
        IoError(io::Error),
    }
    pub type SendSecureResult<T> = Result<T, SendSecureError>;


    impl From<io::Error> for SendSecureError {
        fn from(err: io::Error) -> SendSecureError {
            SendSecureError::IoError(err)
        }
    }

    impl From<error::Error> for SendSecureError {
        fn from(err: error::Error) -> SendSecureError {
            SendSecureError::UnexpectedServerError(err)
        }
    }

    impl From<json::BuilderError> for SendSecureError {
        fn from(err: json::BuilderError) -> SendSecureError {
            SendSecureError::JSONSerializeError(err)
        }
    }
}
