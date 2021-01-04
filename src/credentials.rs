use crate::error::Error;

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

    /// Create a `Credentials` instance from a base64 `String`
    /// which must encode user credentials as `username:password`
    pub fn decode(auth_header_value: String) -> Result<Self, Error> {
        let decoded = base64::decode(auth_header_value)?;
        let as_utf8 = String::from_utf8(decoded)?;
        let parts = as_utf8.split(':');
        let parts: Vec<String> = parts.map(|p| p.to_string()).collect();

        if parts.len() == 2 {
            let credentials = Credentials {
                user_id: parts.get(0).unwrap().to_string(),
                password: parts.get(1).unwrap().to_string(),
            };

            return Ok(credentials);
        }

        Err(Error::InvalidAuthorizationHeader)
    }

    /// Encode a `Credentials` instance into a base64 `String`
    pub fn encode(&self) -> String {
        let credentials = format!("{}:{}", self.user_id, self.password);

        base64::encode(credentials.as_bytes())
    }

    /// Creates a `Credentials` instance from an HTTP Authorization header
    /// which schema is a valid `Basic` HTTP Authorization Schema.
    pub fn from_header(auth_header: String) -> Result<Credentials, Error> {
        // check if its a valid basic auth header
        let parts = auth_header.split(' ');
        let parts: Vec<String> = parts.map(|part| part.to_string()).collect();

        if parts.len() > 2 {
            // invalid authorization token received
            return Err(Error::InvalidAuthorizationHeader);
        }

        if let Some(auth_type) = parts.get(0) {
            // check the provided authorization header
            // to be a "Basic" authorization header
            if auth_type.to_lowercase() != "basic" {
                return Err(Error::InvalidScheme(auth_type.to_string()));
            }
        }

        if let Some(encoded_credentials) = parts.get(1) {
            let credentials = Credentials::decode(encoded_credentials.clone())?;

            return Ok(credentials);
        }

        Err(Error::InvalidAuthorizationHeader)
    }

    /// Creates a HTTP Authorization header value for the basic schema
    /// from the `Credentials` instance
    pub fn as_http_header(&self) -> String {
        let as_base64 = self.encode();

        format!("Basic {}", as_base64)
    }
}
