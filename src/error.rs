use std::fmt;
use base64::DecodeError;
use std::string::FromUtf8Error;

/// Authorization Header Error
#[derive(Debug)]
pub enum AuthBasicError {
  /// The HTTP Authorization header value is invalid
  InvalidAuthorizationHeader,
  /// The HTTP Authorization header contains a valid
  /// value but the scheme is other than `Basic`
  InvalidScheme(String),
  /// The value expected as a base64 encoded `String` is not
  /// encoded correctly
  InvalidBase64Value(String),
  /// The provided binary is not a valid UTF-8 character
  InvalidUTF8Value(String)
}

impl fmt::Display for AuthBasicError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        AuthBasicError::InvalidAuthorizationHeader => write!(f, "Invalid value provided for the HTTP Authorization header"),
        AuthBasicError::InvalidScheme(scheme) => write!(f, "The scheme provided ({}) is not Basic", scheme),
        AuthBasicError::InvalidBase64Value(message) => write!(f, "The value have an invalid base64 encoding\n{}", message),
        AuthBasicError::InvalidUTF8Value(message) => write!(f, "Invalid UTF-8 Provided\n{}", message),
      }
  }
}

impl From<DecodeError> for AuthBasicError {
  fn from(decode_error: DecodeError) -> Self {
      AuthBasicError::InvalidBase64Value(decode_error.to_string())
  }
}

impl From<FromUtf8Error> for AuthBasicError {
  fn from(utf8_err: FromUtf8Error) -> Self {
      AuthBasicError::InvalidUTF8Value(utf8_err.to_string())
  }
}
