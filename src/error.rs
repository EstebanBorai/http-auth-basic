use base64::DecodeError;
use std::string::FromUtf8Error;
use thiserror::Error as ThisError;

/// Authorization Header Error
#[derive(Debug, ThisError)]
pub enum Error {
    /// The HTTP Authorization header value is invalid
    #[error("Invalid authorization header provided")]
    InvalidAuthorizationHeader,
    /// The HTTP Authorization header contains a valid
    /// value but the scheme is other than `Basic`
    #[error("Inavlid authorization header scheme, {0}")]
    InvalidScheme(String),
    /// The value expected as a base64 encoded `String` is not
    /// encoded correctly
    #[error("Invalid Base64 provided, {0}")]
    InvalidBase64Value(String),
    /// The provided binary is not a valid UTF-8 character
    #[error("The provided value is not a valid UTF-8")]
    InvalidUTF8Value(String),
}

impl From<DecodeError> for Error {
    fn from(decode_error: DecodeError) -> Self {
        Self::InvalidBase64Value(decode_error.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(utf8_err: FromUtf8Error) -> Self {
        Self::InvalidUTF8Value(utf8_err.to_string())
    }
}
