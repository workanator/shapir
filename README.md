# shapir
Unofficial [ShareFile REST API](http://api.sharefile.com/rest/) SDK for [Rust](https://www.rust-lang.org/).

## Example

```
use shapir::Connection;

let conn = Connection::new()
	.subdomain("your-subdomain")
	.username("your.username@mail.com")
	.password("your-password")
	.client_id("client-id")
	.client_secret("client-secret")
	.connect();
```
