extern crate shapir;

use shapir::Connection;

fn main() {
	let conn = Connection::new()
		.subdomain("webbula")
		.username("tcms@webbula.com")
		.password("Doh7zohw")
		.client_id("n2UbsBsxpgUHeIGDRYlow8LNz5EKk2wv")
		.client_secret("ZFvMRFVde1PM59alz33PSIDYXG5PrIghTfzMARKUHTx80eLd")
		.connect();

	match conn {
		Ok(_) => println!("Awesome!"),
		Err(err) => panic!("{}", err)
	};
}
