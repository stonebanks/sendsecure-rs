pub mod SendSecure {
    use std::io;
    use hyper::{client,error,status};
    // We derive `Debug` because all types should probably derive `Debug`.
    // This gives us a reasonable human readable description of `CliError` values.
    #[derive(Debug)]
    pub struct SendSecureError {
        message: String,
        code: String,
        error: SendSecureKind
    }
    #[derive(Debug)]
    pub enum SendSecureKind {
        UnexpectedServerError(error::Error),
        RequestError(status::StatusCode),
        IoError(io::Error),
    }
    pub type SendSecureResult<T> = Result<T, SendSecureError>;

    impl SendSecureError {
        pub fn new(code: String, message: String, error: SendSecureKind) -> SendSecureError {
            SendSecureError {
                message: message,
                code: code,
                error: error,
            }
        }
    }

    // impl From<io::Error> for SendSecureError {
    //     fn from(err: io::Error) -> SendSecureError{
    //         SendSecureError::IoError(err)
    //     }
    // } 
}
