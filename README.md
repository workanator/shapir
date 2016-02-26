# SHAPIR [![Build Status](https://travis-ci.org/workanator/shapir.svg?branch=master)](https://travis-ci.org/workanator/shapir)  
Unofficial [ShareFile REST API](http://api.sharefile.com/rest/) SDK for [Rust](https://www.rust-lang.org/).

## Example

```rust
use shapir::Connection;

let conn = Connection::new()
	.subdomain("your-subdomain")
	.username("your.username@mail.com")
	.password("your-password")
	.client_id("client-id")
	.client_secret("client-secret")
	.connect()
	.unwrap();
```
