#[derive(Debug, Clone)]
pub struct QueryOptions {
	select: Option<Vec<String>>,
	expand: Option<Vec<String>>,
	filter: Option<Vec<String>>,
	order_by: Option<Vec<String>>,
	top: Option<u32>,
	skip: Option<u32>,
}


impl QueryOptions {
	pub fn new() -> Self {
		QueryOptions {
			select: None,
			expand: None,
			filter: None,
			order_by: None,
			top: None,
			skip: None,
		}
	}

	pub fn select<T>(mut self, opt: Option<Vec<T>>) -> Self where T: Into<String> {
		match opt {
			Some(items) => {
				let items = items.into_iter()
					.map(|v| v.into())
					.collect();
				self.select = Some(items);
			},
			None => {
				self.select = None;
			},
		};

		self
	}

	pub fn expand<T>(mut self, opt: Option<Vec<T>>) -> Self where T: Into<String> {
		match opt {
			Some(items) => {
				let items = items.into_iter()
					.map(|v| v.into())
					.collect();
				self.expand = Some(items);
			},
			None => {
				self.expand = None;
			},
		};

		self
	}

	pub fn filter<T>(mut self, opt: Option<Vec<T>>) -> Self where T: Into<String> {
		match opt {
			Some(items) => {
				let items = items.into_iter()
					.map(|v| v.into())
					.collect();
				self.filter = Some(items);
			},
			None => {
				self.filter = None;
			},
		};

		self
	}

	pub fn order_by<T>(mut self, opt: Option<Vec<T>>) -> Self where T: Into<String> {
		match opt {
			Some(items) => {
				let items = items.into_iter()
					.map(|v| v.into())
					.collect();
				self.order_by = Some(items);
			},
			None => {
				self.order_by = None;
			},
		};

		self
	}

	pub fn top<T>(mut self, opt: Option<T>) -> Self where T: Into<u32> {
		match opt {
			Some(amount) => {
				self.top = Some(amount.into());
			},
			None => {
				self.top = None;
			},
		};

		self
	}

	pub fn skip<T>(mut self, opt: Option<T>) -> Self where T: Into<u32> {
		match opt {
			Some(amount) => {
				self.skip = Some(amount.into());
			},
			None => {
				self.skip = None;
			},
		};

		self
	}
}
