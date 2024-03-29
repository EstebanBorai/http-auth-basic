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
    use std::str::FromStr;

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
        let auth_header_value = String::from("dXNlcm5hbWU6OnBhc3M6d29yZDo=");
        let credentials = Credentials::decode(auth_header_value).unwrap();

        assert_eq!(credentials.user_id, String::from("username"));
        assert_eq!(credentials.password, String::from(":pass:word:"));
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
        let auth_header_value = String::from("Basic dXNlcm5hbWU6OnBhc3M6d29yZDo=");
        let credentials = Credentials::from_header(auth_header_value).unwrap();

        assert_eq!(credentials.user_id, String::from("username"));
        assert_eq!(credentials.password, String::from(":pass:word:"));
    }

    #[test]
    fn it_creates_header_value() {
        let credentials = Credentials::new("username", "password");
        let credentials = credentials.as_http_header();

        assert_eq!(String::from("Basic dXNlcm5hbWU6cGFzc3dvcmQ="), credentials);
    }

    #[test]
    fn it_creates_header_value_with_colon() {
        let credentials = Credentials::new("username", ":pass:word:");
        let credentials = credentials.as_http_header();

        assert_eq!(
            String::from("Basic dXNlcm5hbWU6OnBhc3M6d29yZDo="),
            credentials
        );
    }

    #[test]
    fn it_creates_credentials_from_str_value() {
        let auth_header_value_str = "dXNlcm5hbWU6cGFzc3dvcmQ=";
        let credentials = Credentials::from_str(auth_header_value_str).unwrap();

        assert_eq!(credentials.user_id, String::from("username"));
        assert_eq!(credentials.password, String::from("password"));
    }

    #[test]
    fn it_creates_credentials_from_str_header() {
        let auth_header_str = "Basic dXNlcm5hbWU6cGFzc3dvcmQ=";
        let credentials = Credentials::from_str(auth_header_str).unwrap();

        assert_eq!(credentials.user_id, String::from("username"));
        assert_eq!(credentials.password, String::from("password"));
    }

    #[test]
    fn from_header_returns_err_when_input_has_no_whitespace() {
        let auth_header_value = String::from("BasicdXNlcm5hbWU6OnBhc3M6d29yZDo=");
        let credentials = Credentials::from_header(auth_header_value);

        assert!(credentials.is_err());
    }

    #[test]
    fn from_header_returns_err_when_input_contains_multiple_whitespaces() {
        let auth_header_value = String::from("Basic dXNlcm5hbWU6On Bhc3M6d29yZDo=");
        let credentials = Credentials::from_header(auth_header_value);

        assert!(credentials.is_err());
    }

    #[test]
    fn from_header_returns_err_when_input_is_not_basic_auth() {
        let auth_header_value = String::from("Bearer dXNlcm5hbWU6OnBhc3M6d29yZDo=");
        let credentials = Credentials::from_header(auth_header_value);

        assert!(credentials.is_err());
    }

    #[test]
    fn decode_returns_err_when_input_has_no_colons() {
        // base64::encode("usernamepassword") = "dXNlcm5hbWVwYXNzd29yZA==""
        let auth_header_value = String::from("dXNlcm5hbWVwYXNzd29yZA==");
        let credentials = Credentials::from_header(auth_header_value);

        assert!(credentials.is_err());
    }
}
