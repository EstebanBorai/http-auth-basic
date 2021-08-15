use std::str::FromStr;

use crate::error::AuthBasicError;

/// A `struct` to represent the `user_id` and `password` fields
/// from an _Authorization Basic_ header value
#[derive(Debug, PartialEq)]
pub struct Credentials {
    pub user_id: String,
    pub password: String,
}

impl Credentials {
    /// Create a new `Credentials` instance
    /// this is equivalent to writing:
    ///
    /// ```
    /// use http_auth_basic::Credentials;
    ///
    /// let credentials = Credentials {
    ///     user_id: String::from("Foo"),
    ///     password: String::from("Bar"),
    /// };
    ///
    /// ```
    ///
    pub fn new(user_id: &str, password: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            password: password.to_string(),
        }
    }

    /// Creates a `Credentials` instance from a base64 `String`
    /// which must encode user credentials as `username:password`
    pub fn decode(auth_header_value: String) -> Result<Self, AuthBasicError> {
        let decoded = base64::decode(auth_header_value)?;
        let as_utf8 = String::from_utf8(decoded)?;

        // RFC 2617 provides support for passwords with colons
        if let Some((user_id, password)) = as_utf8.split_once(':') {
            return Ok(Self::new(user_id, password));
        }

        Err(AuthBasicError::InvalidAuthorizationHeader)
    }

    /// Encode a `Credentials` instance into a base64 `String`
    pub fn encode(&self) -> String {
        let credentials = format!("{}:{}", self.user_id, self.password);

        base64::encode(credentials.as_bytes())
    }

    /// Creates a `Credentials` instance from an HTTP Authorization header
    /// which schema is a valid `Basic` HTTP Authorization Schema.
    pub fn from_header(auth_header: String) -> Result<Credentials, AuthBasicError> {
        // check if its a valid basic auth header
        if let Some((auth_type, encoded_credentials)) = auth_header.split_once(' ') {
            if encoded_credentials.contains(' ') {
                // Invalid authorization token received
                return Err(AuthBasicError::InvalidAuthorizationHeader);
            }

            // Check the provided authorization header
            // to be a "Basic" authorization header
            if auth_type.to_lowercase() != "basic" {
                return Err(AuthBasicError::InvalidScheme(auth_type.to_string()));
            }

            let credentials = Credentials::decode(encoded_credentials.to_string())?;

            return Ok(credentials);
        }

        Err(AuthBasicError::InvalidAuthorizationHeader)
    }

    /// Creates a HTTP Authorization header value for the basic schema
    /// from the `Credentials` instance
    pub fn as_http_header(&self) -> String {
        let as_base64 = self.encode();

        format!("Basic {}", as_base64)
    }
}

impl FromStr for Credentials {
    type Err = AuthBasicError;

    /// Creates a `Credentials` instance from either a base64 `&str`
    /// which must encode user credentials as `username:password`
    /// or an HTTP Authorization header which schema is a
    /// valid `Basic` HTTP Authorization Schema.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(' ') {
            return Self::from_header(s.into());
        }

        Self::decode(s.into())
    }
}
