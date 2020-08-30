<div>
  <div align="center" style="display: block; text-align: center;">
    <img src="https://via.placeholder.com/120" height="120" width="120" />
  </div>
  <h1 align="center">http-auth-basic</h1>
  <h4 align="center">HTTP Basic Authentication Scheme (RFC 7617 base64-encoded credentials) for Rust applications</h4>
</div>

## Description

The "Basic" Hypertext Transfer Protocol (HTTP) authentication scheme, transmits credentials as user-id/password pairs, encoded using Base64.

## Installation

To install add the crate as follows to your dependencies list in your `Cargo.toml`.

```toml
[dependencies]
http-auth-basic = "0.1.0"
```

If you want to use a specific version, you must add the crate to your `Cargo.toml` as follows:

```toml
[dependencies]
http-auth-basic = { git = "https://github.com/EstebanBorai/http-auth-basic.git", tag = "v0.1.0" }
```

## Usage

Decoding a basic authorization value and creating a `Credentials` struct
from it

```rust
use http_auth_basic::Credentials;

let auth_header_value = String::from("Basic dXNlcm5hbWU6cGFzc3dvcmQ=");
let credentials = Credentials::from_header(auth_header_value).unwrap();

assert_eq!(credentials.user_id, String::from("username"));
assert_eq!(credentials.password, String::from("password"));
```

Encoding `Credentials` into a basic authorization header value.

```rust
use http_auth_basic::Credentials;

let credentials = Credentials::new("username", "password");
let credentials = credentials.as_http_header();

assert_eq!(String::from("Basic dXNlcm5hbWU6cGFzc3dvcmQ="), credentials);
```

## Release

```bash
git tag -a v0.1.0 -m "Release Message"
git push origin main --follow-tags
```

## Contributing

Every contribution to this project is welcome! Feel free to open a pull request or an issue.

## References

- [MDN The general HTTP Authentication Framework](https://developer.mozilla.org/en-US/docs/Web/HTTP/Authentication)
- [RFC7617](https://tools.ietf.org/html/rfc7617)

## License

Licensed under the GNU General Public License.