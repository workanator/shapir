use std::io;
use hyper::method::Method;
use serde_json;
use ::{Result, Error};
use ::connection::Connection;
use ::odata::Parameters;
use super::Path;


/// Item content reader/writer
pub struct Content {
	stream: Box<io::Read>,
}


impl Content {
	pub fn open_for_read(conn: Connection, id: String) -> Result<Self> {
		let params = Parameters::new().custom_add(("redirect", "false"));
		let uri = Path::Id(id).entity_and_parameters(Some("/Download"), Some(params));

		// Try to obtain download specifications
		let specs: serde_json::Value = match conn.query_string(Method::Get, uri, None, None) {
			Ok(json) => serde_json::from_str(&json).unwrap(),
			Err(err) =>  return err.result()
		};

		// Get download URL from the specs and open the stream
		let download_url = match specs.find("DownloadUrl") {
			Some(v) => v.as_string().unwrap(),
			None => return Error::new("DownloadSpecification.DownloadUrl property is missing.").result()
		};

		let reader = match conn.custom_request(Method::Get, download_url.to_owned(), None, None) {
			Ok(response) => Box::new(response),
			Err(err) => return Error::new("Cannot start download").because(err).result()
		};

		Ok(Content {
			stream: reader,
		})
	}
}


impl io::Read for Content {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		self.stream.read(buf)
	}
}