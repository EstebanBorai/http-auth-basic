<div>
  <div align="center" style="display: block; text-align: center;">
    <img
      src="https://camo.githubusercontent.com/734a3468bce992fbc3b729562d41c92f4912c99a/68747470733a2f2f7777772e727573742d6c616e672e6f72672f7374617469632f696d616765732f727573742d6c6f676f2d626c6b2e737667"
      height="120"
      width="120"
    />
  </div>
  <h1 align="center">http-auth-basic</h1>
  <h4 align="center">
    HTTP Basic Authentication Scheme (RFC 7617 and RFC 2617 compilant, base64-encoded credentials) for Rust applications
  </h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/http-auth-basic.svg)](https://crates.io/crates/http-auth-basic)
  [![Documentation](https://docs.rs/http-auth-basic/badge.svg)](https://docs.rs/http-auth-basic)
  ![Build](https://github.com/EstebanBorai/http-auth-basic/workflows/build/badge.svg)
  ![Clippy](https://github.com/EstebanBorai/http-auth-basic/workflows/clippy/badge.svg)
  ![Fmt](https://github.com/EstebanBorai/http-auth-basic/workflows/fmt/badge.svg)
  ![Release](https://github.com/EstebanBorai/http-auth-basic/workflows/release/badge.svg)
  ![Tests](https://github.com/EstebanBorai/http-auth-basic/workflows/tests/badge.svg)

</div>

## Description

The "Basic" Hypertext Transfer Protocol (HTTP) authentication scheme, transmits credentials as user-id/password pairs, encoded using Base64.

<div align="center">
  <img src="https://raw.githubusercontent.com/EstebanBorai/http-auth-basic/main/assets/basic-auth-workflow.png" />
</div>

The server will gather the credentials from the base64 encoded header value, and will validate them
to authenticate the user in question.

This crate covers the credentials encoding and decoding. The `Credentials` struct provides two fields
`user_id` and `password`, these are filled with they raw values.

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
- [RFC2617](https://tools.ietf.org/html/rfc2617)

## License

Distributed under the terms of both the MIT license and the Apache License (Version 2.0)
