use std::io;
use hyper::method::Method;
use md5;
use rustc_serialize::hex::ToHex;
use url::form_urlencoded;
use ::error::{Result, Error, IoError, IoErrorKind};
use ::connection::Connection;
use ::odata::Parameters;
use super::Path;


// Define the uploading data chunk size
const CHUNK_SIZE: usize = 1024 * 16;


struct WriteBuf {
	conn: Connection,
	size: u64,
	written: u64,
	buffer: Vec<u8>,
	chunk_uri: String,
	chunk_no: u32,
}


impl WriteBuf {
	fn new(conn: Connection, size: u64, chunk_uri: String) -> Self {
		WriteBuf {
			conn: conn,
			size: size,
			buffer: Vec::new(),
			written: 0,
			chunk_uri: chunk_uri,
			chunk_no: 0,
		}
	}
}


/// Item content reader/writer.  
///
/// The struct implements `std::io::Read` and `std::io::Write` traits so it can be used
/// neat abilities of the standard Rust library to read and write data.
///
/// ## Panics 
/// The instance of `Content` can only read or only write data during the lifetime.
/// Writing data will panic if `Content` was created fo reading data and reading data
/// will panic of it was created for writing.
///
/// Also it can panic if the uploaded amount of bytes exceeded the file size given.
pub struct Content {
	reader: Option<Box<io::Read>>,
	writer: Option<WriteBuf>,
}


impl Content {
	/// Create the new instance of `Content` for reading data. Most time there is no need
	/// to create this struct directly. The better practice is to use method `download()`
	/// of the `Items` instance.
	pub fn open_for_read(conn: Connection, path: Path) -> Result<Self> {
		let params = Parameters::new()
			.custom_add(("redirect", "false"));

		let uri = path.entity_and_parameters(Some("/Download"), Some(params));

		// Try to obtain download specifications and start downloading process
		conn.query_json(Method::Get, uri, None, None)
			.and_then(|specs| {
				// Get download URL from the specs and open the stream
				let download_url = match specs.find("DownloadUrl") {
					Some(v) => v.as_string().unwrap(),
					None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "DownloadSpecification.DownloadUrl property is missing."))
				};

				let reader = match conn.custom_request(Method::Get, download_url.to_owned(), None, None) {
					Ok(response) => Box::new(response),
					Err(err) => return Error::io_result(IoError::new(IoErrorKind::NotConnected, err))
				};

				Ok(Content {
					reader: Some(reader),
					writer: None,
				})
			})
	}

	/// Create the new instance of `Content` for writing data. Most time there is no need
	/// to create this struct directly. The better practice is to use method `upload()`
	/// of the `Items` instance.
	pub fn open_for_write(conn: Connection, parent: Path, name: String, size: u64, unzip: bool, overwrite: bool) -> Result<Self> {
		let params = Parameters::new()
			.custom(vec![
				("method", "streamed"),
				("raw", "true"),
				("responseFormat", "json"),
				("unzip", &::api::bool_to_string(unzip)),
				("overwrite", &::api::bool_to_string(overwrite)),
				("fileName", &name),
				("fileSize", &size.to_string()) ]);

		let uri = parent.entity_and_parameters(Some("/Upload"), Some(params));

		// Try to obtain upload specifications and start uploading process
		conn.query_json(Method::Get, uri, None, None)
			.and_then(|specs| {
				// Get download URL from the specs and open the stream
				let chunk_uri = match specs.find("ChunkUri") {
					Some(v) => v.as_string().unwrap(),
					None => return Error::io_result(IoError::new(IoErrorKind::InvalidInput, "UploadSpecification.ChunkUri property is missing."))
				};

				Ok(Content {
					reader: None,
					writer: Some(WriteBuf::new(conn, size, chunk_uri.to_owned())),
				})
			})
	}
}


// Implement std::io::Read trait
// Panic in read operation if the content is not opened for read.
impl io::Read for Content {
	fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
		if let Some(ref mut reader) = self.reader {
			// Read the next chunk of data
			reader.read(buf)
		}
		else {
			panic!("Content stream is not opened for reading data");
		}
	}
}


// Implement std::io::Write trait
// Panic in write operations if the content is not opened for write or 
// if the uploaded amount of bytes exceeded those given in `size`.
impl io::Write for Content {
	fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
		if let Some(ref mut write_buf) = self.writer {
			if write_buf.written < write_buf.size {
				write_buf.buffer.extend_from_slice(buf);
				let finish = (write_buf.written + write_buf.buffer.len() as u64) >= write_buf.size;

				// Force upload if the chunk size is reached or the file size if reached
				if write_buf.buffer.len() >= CHUNK_SIZE || finish {
					// Compute MD5 digest of the data chunk
					let digest = md5::compute(write_buf.buffer.as_slice());

					// Build chunk parameters
					let mut params = form_urlencoded::Serializer::new(String::new());

					if finish {
						params.append_pair("finish", "true");
					}

					let params = params
						.append_pair("index", &write_buf.chunk_no.to_string())
						.append_pair("offset", &write_buf.written.to_string())
						.append_pair("filehash", &digest.to_hex())
						.finish();

					// Upload the chunk
					let mut url = write_buf.chunk_uri.clone();
					url.push('&');
					url.push_str(&params);

					match write_buf.conn.custom_request(Method::Post, url, None, Some(write_buf.buffer.as_slice())) {
						Ok(response) => {
							if response.status.is_success() {
								write_buf.written = write_buf.written + write_buf.buffer.len() as u64;
								write_buf.chunk_no = write_buf.chunk_no + 1;
								write_buf.buffer.clear();

								Ok(buf.len())
							}
							else {
								Err(io::Error::new(io::ErrorKind::Other, format!("Chunk upload failed with status {}", response.status)))
							}
						},
						Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Cannot upload chunk"))
					}
				}
				else {
					Ok(buf.len())
				}
			}
			else {
				panic!("Amount of bytes uploaded exceeded the file size");
			}
		}
		else {
			panic!("Content stream is not opened for writing data");
		}
	}

    fn flush(&mut self) -> io::Result<()> {
    	Ok(())
    }
}
