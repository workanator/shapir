use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};


pub struct Helper;


impl Helper {

	/// Helper metod for creating headers containing `Content-Type: application/json`.
	pub fn json_headers() -> Headers {
		let mut headers = Headers::new();
		headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));
		headers
	}

}
