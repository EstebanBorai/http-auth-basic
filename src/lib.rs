//! # http_auth_basic
//!
//! HTTP Basic Authentication Scheme (RFC 7617 base64-encoded credentials) for Rust applications
//!
//! # Example
//!
//! Decoding a basic authorization value and creating a `Credentials` struct
//! from it
//!
//! ```
//! use http_auth_basic::Credentials;
//!
//! let auth_header_value = String::from("Basic dXNlcm5hbWU6cGFzc3dvcmQ=");
//! let credentials = Credentials::from_header(auth_header_value).unwrap();
//!
//! assert_eq!(credentials.user_id, String::from("username"));
//! assert_eq!(credentials.password, String::from("password"));
//! ```
//!
//! Encoding `Credentials` into a basic authorization header value.
//!
//! ```
//! use http_auth_basic::Credentials;
//!
//! let credentials = Credentials::new("username", "password");
//! let credentials = credentials.as_http_header();
//!
//! assert_eq!(String::from("Basic dXNlcm5hbWU6cGFzc3dvcmQ="), credentials);
//! ```
//!
mod credentials;
mod error;

pub use credentials::*;
pub use error::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_credentials_from_value() {
        let auth_header_value = String::from("dXNlcm5hbWU6cGFzc3dvcmQ=");
        let credentials = Credentials::decode(auth_header_value).unwrap();

        assert_eq!(credentials.user_id, String::from("username"));
        assert_eq!(credentials.password, String::from("password"));
    }

    #[test]
    fn it_creates_credentials_from_value_with_colon() {
        let auth_header_value = String::from("dXNlcm5hbWU6cGFzczp3b3Jk");
        let credentials = Credentials::decode(auth_header_value).unwrap();

        assert_eq!(credentials.user_id, String::from("username"));
        assert_eq!(credentials.password, String::from("pass:word"));
    }

    #[test]
    fn it_encodes_credentials() {
        let credentials = Credentials::new("username", "password");
        let credentials = credentials.encode();

        assert_eq!(String::from("dXNlcm5hbWU6cGFzc3dvcmQ="), credentials);
    }

    #[test]
    fn it_creates_credentials_from_header_value() {
        let auth_header_value = String::from("Basic dXNlcm5hbWU6cGFzc3dvcmQ=");
        let credentials = Credentials::from_header(auth_header_value).unwrap();

        assert_eq!(credentials.user_id, String::from("username"));
        assert_eq!(credentials.password, String::from("password"));
    }

    #[test]
    fn it_creates_credentials_from_header_value_with_colon() {
        let auth_header_value = String::from("Basic dXNlcm5hbWU6cGFzczp3b3Jk");
        let credentials = Credentials::from_header(auth_header_value).unwrap();

        assert_eq!(credentials.user_id, String::from("username"));
        assert_eq!(credentials.password, String::from("pass:word"));
    }

    #[test]
    fn it_creates_header_value() {
        let credentials = Credentials::new("username", "password");
        let credentials = credentials.as_http_header();

        assert_eq!(String::from("Basic dXNlcm5hbWU6cGFzc3dvcmQ="), credentials);
    }

    #[test]
    fn it_creates_header_value_with_colon() {
        let credentials = Credentials::new("username", "pass:word");
        let credentials = credentials.as_http_header();

        assert_eq!(String::from("Basic dXNlcm5hbWU6cGFzczp3b3Jk"), credentials);
    }
}
