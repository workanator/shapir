/// OData query options/parameters
///
/// # Example
///
/// ```rust
/// use shapir::odata::QueryOptions;
///
///	let opts = QueryOptions::new()
///		.select(Some(vec!["Field1", "Field2", "3"]))
///		.expand(Some(vec!["Children", "Siblings"]))
///		.filter(Some(vec!["A eq B", "true"]))
///		.order_by(Some(vec!["Date asc", "Time desc", "Id"]))
///		.top(Some(10u32))
///		.skip(Some(9u32));
/// ```
///

#[derive(Debug, Clone, Default)]
pub struct QueryOptions {
	select: Option<Vec<String>>,
	expand: Option<Vec<String>>,
	filter: Option<Vec<String>>,
	order_by: Option<Vec<String>>,
	top: Option<u32>,
	skip: Option<u32>,
}

impl QueryOptions {
	/// Create a new instance of `QueryOptions` with no options specified.
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

	/// Set `$select` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#theselectsystemqueryoption)
	/// for more details.
	pub fn select<T>(mut self, opt: Option<T>) -> Self where T: IntoIterator, T::Item: Into<String> {
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

	/// Set `$expand` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#theexpandsystemqueryoption)
	/// for more details.
	pub fn expand<T>(mut self, opt: Option<T>) -> Self where T: IntoIterator, T::Item: Into<String> {
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

	/// Set `$filter` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#thefiltersystemqueryoption)
	/// for more details.
	pub fn filter<T>(mut self, opt: Option<T>) -> Self where T: IntoIterator, T::Item: Into<String> {
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

	/// Set `$orderBy` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#theorderbysystemqueryoption)
	/// for more details.
	pub fn order_by<T>(mut self, opt: Option<T>) -> Self where T: IntoIterator, T::Item: Into<String> {
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

	/// Set `$top` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#thetopsystemqueryoption)
	/// for more details.
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

	/// Set `$skip` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#theskipsystemqueryoption)
	/// for more details.
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


impl Into<String> for QueryOptions {
	fn into(self) -> String {
		use url::form_urlencoded;

		// Fill in options
		let mut options: Vec<(&str, String)> = Vec::new();

		if let Some(items) = self.select {
			options.push(("$select", items.into_boxed_slice().join(",")));
		};

		if let Some(items) = self.expand {
			options.push(("$expand", items.into_boxed_slice().join(",")));
		};

		if let Some(items) = self.filter {
			options.push(("$filter", items.into_boxed_slice().join(" and ")));
		};

		if let Some(items) = self.order_by {
			options.push(("$orderBy", items.into_boxed_slice().join(",")));
		};

		if let Some(num) = self.top {
			options.push(("$top", num.to_string()));
		};

		if let Some(num) = self.skip {
			options.push(("$skip", num.to_string()));
		};

		// Convert OData options into URL query
		form_urlencoded::serialize(options)
	}
}


#[cfg(test)]
mod tests {
	use super::QueryOptions;

	fn encode_pairs(v: Vec<(&str, &str)>) -> String {
		use url::form_urlencoded;
		form_urlencoded::serialize(v)
	}

	#[test]
	fn query_options_none() {
		let opts: String = QueryOptions::new()
			.into();
		assert_eq!(opts, "".to_string());
	}

	#[test]
	fn query_options_select() {
		let opts: String = QueryOptions::new()
			.select(Some(vec!["Field1", "Field2", "3"]))
			.into();
		assert_eq!(opts, encode_pairs(vec![("$select", "Field1,Field2,3")]));
	}

	#[test]
	fn query_options_expand() {
		let opts: String = QueryOptions::new()
			.expand(Some(vec!["Children", "Siblings"]))
			.into();
		assert_eq!(opts, encode_pairs(vec![("$expand", "Children,Siblings")]));
	}

	#[test]
	fn query_options_filter() {
		let opts: String = QueryOptions::new()
			.filter(Some(vec!["A eq B", "true"]))
			.into();
		assert_eq!(opts, encode_pairs(vec![("$filter", "A eq B and true")]));
	}

	#[test]
	fn query_options_order_by() {
		let opts: String = QueryOptions::new()
			.order_by(Some(vec!["Date asc", "Time desc", "Id"]))
			.into();
		assert_eq!(opts, encode_pairs(vec![("$orderBy", "Date asc,Time desc,Id")]));
	}

	#[test]
	fn query_options_top() {
		let opts: String = QueryOptions::new()
			.top(Some(10u32))
			.into();
		assert_eq!(opts, encode_pairs(vec![("$top", "10")]));
	}

	#[test]
	fn query_options_skip() {
		let opts: String = QueryOptions::new()
			.skip(Some(9u32))
			.into();
		assert_eq!(opts, encode_pairs(vec![("$skip", "9")]));
	}

	#[test]
	fn query_options_all() {
		let opts: String = QueryOptions::new()
			.select(Some(vec!["Field1", "Field2", "3"]))
			.expand(Some(vec!["Children", "Siblings"]))
			.filter(Some(vec!["A eq B", "true"]))
			.order_by(Some(vec!["Date asc", "Time desc", "Id"]))
			.top(Some(10u32))
			.skip(Some(9u32))
			.into();

		let should_be = encode_pairs(vec![
				("$select", "Field1,Field2,3"),
				("$expand", "Children,Siblings"),
				("$filter", "A eq B and true"),
				("$orderBy", "Date asc,Time desc,Id"),
				("$top", "10"),
				("$skip", "9")
			]);

		assert_eq!(opts, should_be);
	}
}
