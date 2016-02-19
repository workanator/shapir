use std;
use hyper::Url;
use url::ParseError;

pub fn to_url(url: String) -> std::result::Result<Url, ParseError> {
	Url::parse(&url)
}
