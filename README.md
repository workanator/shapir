# SHAPIR [![Build Status](https://travis-ci.org/workanator/shapir.svg?branch=master)](https://travis-ci.org/workanator/shapir) [![DUB](https://img.shields.io/dub/l/vibe-d.svg)](https://github.com/workanator/shapir/blob/master/LICENSE)  
Unofficial [ShareFile REST API](http://api.sharefile.com/rest/) SDK for [Rust](https://www.rust-lang.org/).

## First things first

To work with ShareFile REST API you should register you appliction first. Please follow instructions from [the official documentation](http://api.sharefile.com/rest/login.aspx?displayMessage=1&referrer=/rest/oauth2-request.aspx) to obtain API key.

## SDK design

The SDK follows the composition of API Entities of the REST API. So for example to access [Items](http://api.sharefile.com/rest/docs/resource.aspx?name=Items) functionality you should obtain instance of that API Entity using method `items()` of the connection opened.

The workflow of the SDK usage looks like this:

1. Open `Connection` (behind the scene it does more things like authentication).  
2. Obtain the required API Entity using the right method of the `Connection`.  
3. Perform required operations on the API entity.  

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

let items = conn.items();
let files = items.list(Path::Home, None).unwrap();
```
