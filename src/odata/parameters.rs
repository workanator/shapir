/// OData query parameters
///
/// ## Example
///
/// ```rust
/// use shapir::odata::Parameters;
///
///	let opts = Parameters::new()
///		.select(vec!["Field1", "Field2", "3"])
///		.expand(vec!["Children", "Siblings"])
///		.filter(vec!["A eq B", "true"])
///		.order_by(vec!["Date asc", "Time desc", "Id"])
///		.top(10u32)
///		.skip(9u32);
/// ```
///

#[derive(Debug, Clone, Default)]
pub struct Parameters {
	custom: Option<Vec<(&'static str, String)>>,
	select: Option<Vec<String>>,
	expand: Option<Vec<String>>,
	filter: Option<Vec<String>>,
	order_by: Option<Vec<String>>,
	top: Option<u32>,
	skip: Option<u32>,
}

impl Parameters {
	/// Create a new instance of `Parameters` with no options specified.
	pub fn new() -> Self {
		Parameters {
			custom: None,
			select: None,
			expand: None,
			filter: None,
			order_by: None,
			top: None,
			skip: None,
		}
	}

	/// Set custom parameters
	pub fn custom<V>(mut self, opt: Vec<(&'static str, V)>) -> Self where V: Into<String> {
		let items = opt.into_iter()
			.map(|v| (v.0, v.1.into()))
			.collect();
		self.custom = Some(items);
		self
	}

	/// Add custom parameter
	pub fn custom_add<V>(mut self, param: (&'static str, V)) -> Self where V: Into<String> {
		match self.custom {
			Some(ref mut items) => {
				items.push((param.0, param.1.into()));
			},
			None => {
				self.custom = Some(vec![(param.0, param.1.into())]);
			}
		};

		self
	}

	/// Set `$select` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#theselectsystemqueryoption)
	/// for more details.
	pub fn select<T>(mut self, opt: T) -> Self where T: IntoIterator, T::Item: Into<String> {
		let items = opt.into_iter()
			.map(|v| v.into())
			.collect();
		self.select = Some(items);
		self
	}

	/// Add `$select` option
	pub fn select_add<V>(mut self, opt: V) -> Self where V: Into<String> {
		match self.select {
			Some(ref mut items) => {
				items.push(opt.into());
			},
			None => {
				self.select = Some(vec![opt.into()]);
			}
		};

		self
	}

	/// Set `$expand` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#theexpandsystemqueryoption)
	/// for more details.
	pub fn expand<T>(mut self, opt: T) -> Self where T: IntoIterator, T::Item: Into<String> {
		let items = opt.into_iter()
			.map(|v| v.into())
			.collect();
		self.expand = Some(items);
		self
	}

	/// Add `$expand` option
	pub fn expand_add<V>(mut self, opt: V) -> Self where V: Into<String> {
		match self.expand {
			Some(ref mut items) => {
				items.push(opt.into());
			},
			None => {
				self.expand = Some(vec![opt.into()]);
			}
		};

		self
	}

	/// Set `$filter` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#thefiltersystemqueryoption)
	/// for more details.
	pub fn filter<T>(mut self, opt: T) -> Self where T: IntoIterator, T::Item: Into<String> {
		let items = opt.into_iter()
			.map(|v| v.into())
			.collect();
		self.filter = Some(items);
		self
	}

	/// Add `$filter` option
	pub fn filter_add<V>(mut self, opt: V) -> Self where V: Into<String> {
		match self.filter {
			Some(ref mut items) => {
				items.push(opt.into());
			},
			None => {
				self.filter = Some(vec![opt.into()]);
			}
		};

		self
	}

	/// Set `$orderBy` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#theorderbysystemqueryoption)
	/// for more details.
	pub fn order_by<T>(mut self, opt: T) -> Self where T: IntoIterator, T::Item: Into<String> {
		let items = opt.into_iter()
			.map(|v| v.into())
			.collect();
		self.order_by = Some(items);
		self
	}

	/// Add `$orderBy` option
	pub fn order_by_add<V>(mut self, opt: V) -> Self where V: Into<String> {
		match self.order_by {
			Some(ref mut items) => {
				items.push(opt.into());
			},
			None => {
				self.order_by = Some(vec![opt.into()]);
			}
		};

		self
	}

	/// Set `$top` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#thetopsystemqueryoption)
	/// for more details.
	pub fn top<T>(mut self, opt: T) -> Self where T: Into<u32> {
		self.top = Some(opt.into());
		self
	}

	/// Set `$skip` option. See [OData documentation](http://www.odata.org/documentation/odata-version-3-0/odata-version-3-0-core-protocol/#theskipsystemqueryoption)
	/// for more details.
	pub fn skip<T>(mut self, opt: T) -> Self where T: Into<u32> {
		self.skip = Some(opt.into());
		self
	}
}


impl Into<String> for Parameters {
	fn into(self) -> String {
		use url::form_urlencoded;

		// Fill in options
		let mut options: Vec<(&str, String)> = Vec::new();

		if let Some(items) = self.custom {
			options.extend(items);
		};

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


impl ToString for Parameters {
	fn to_string(&self) -> String {
		self.clone().into()
	}
}


#[cfg(test)]
mod tests {
	use super::Parameters;

	fn encode_pairs(v: Vec<(&str, &str)>) -> String {
		use url::form_urlencoded;
		form_urlencoded::serialize(v)
	}

	#[test]
	fn query_options_none() {
		let opts: String = Parameters::new()
			.into();
		assert_eq!(opts, "".to_string());
	}

	#[test]
	fn query_options_custom() {
		let opts: String = Parameters::new()
			.custom(vec![("a", "hello"), ("enable", "true")])
			.custom_add(("price", "20"))
			.into();
		assert_eq!(opts, encode_pairs(vec![("a", "hello"), ("enable", "true"), ("price", "20")]));
	}

	#[test]
	fn query_options_select() {
		let opts: String = Parameters::new()
			.select(vec!["Field1", "Field2", "3"])
			.into();
		assert_eq!(opts, encode_pairs(vec![("$select", "Field1,Field2,3")]));
	}

	#[test]
	fn query_options_select_add() {
		let opts: String = Parameters::new()
			.select_add("Field1")
			.select_add("Field2")
			.select_add("3")
			.into();
		assert_eq!(opts, encode_pairs(vec![("$select", "Field1,Field2,3")]));
	}

	#[test]
	fn query_options_expand() {
		let opts: String = Parameters::new()
			.expand(vec!["Children", "Siblings"])
			.into();
		assert_eq!(opts, encode_pairs(vec![("$expand", "Children,Siblings")]));
	}

	#[test]
	fn query_options_expand_add() {
		let opts: String = Parameters::new()
			.expand_add("Children")
			.expand_add("Siblings")
			.into();
		assert_eq!(opts, encode_pairs(vec![("$expand", "Children,Siblings")]));
	}

	#[test]
	fn query_options_filter() {
		let opts: String = Parameters::new()
			.filter(vec!["A eq B", "true"])
			.into();
		assert_eq!(opts, encode_pairs(vec![("$filter", "A eq B and true")]));
	}

	#[test]
	fn query_options_filter_add() {
		let opts: String = Parameters::new()
			.filter_add("A eq B")
			.filter_add("true")
			.into();
		assert_eq!(opts, encode_pairs(vec![("$filter", "A eq B and true")]));
	}

	#[test]
	fn query_options_order_by() {
		let opts: String = Parameters::new()
			.order_by(vec!["Date asc", "Time desc", "Id"])
			.into();
		assert_eq!(opts, encode_pairs(vec![("$orderBy", "Date asc,Time desc,Id")]));
	}

	#[test]
	fn query_options_order_by_add() {
		let opts: String = Parameters::new()
			.order_by_add("Date asc")
			.order_by_add("Time desc")
			.order_by_add("Id")
			.into();
		assert_eq!(opts, encode_pairs(vec![("$orderBy", "Date asc,Time desc,Id")]));
	}

	#[test]
	fn query_options_top() {
		let opts: String = Parameters::new()
			.top(10u32)
			.into();
		assert_eq!(opts, encode_pairs(vec![("$top", "10")]));
	}

	#[test]
	fn query_options_skip() {
		let opts: String = Parameters::new()
			.skip(9u32)
			.into();
		assert_eq!(opts, encode_pairs(vec![("$skip", "9")]));
	}

	#[test]
	fn query_options_all() {
		let opts: String = Parameters::new()
			.select(vec!["Field1", "Field2"])
			.select_add("3")
			.expand(vec!["Children"])
			.expand_add("Siblings")
			.filter(vec!["A eq B"])
			.filter_add("true")
			.order_by(vec!["Date asc", "Time desc"])
			.order_by_add("Id")
			.top(10u32)
			.skip(9u32)
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
