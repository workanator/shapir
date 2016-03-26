/// The option which can hold no, one, or many values of some type.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MultiOption<T> {
	/// The option contains no values
	None,
	/// The option contains one value
	One(T),
	/// The option contains many values
	Many(Vec<T>)
}


impl<T> MultiOption<T> {
	/// Test if the option is `None`
	pub fn is_none(&self) -> bool {
		match self {
			&MultiOption::None => true,
			_ => false
		}
	}

	/// Test if the option is `One`
	pub fn is_one(&self) -> bool {
		match self {
			&MultiOption::One(_) => true,
			_ => false
		}
	}

	/// Test if the option is `Many`
	pub fn is_many(&self) -> bool {
		match self {
			&MultiOption::Many(_) => true,
			_ => false
		}
	}
}


#[cfg(test)]
mod tests {
	use super::MultiOption;

	#[test]
	fn multi_option_create() {
		let opt: MultiOption<String> = MultiOption::None;
		assert_eq!(opt, MultiOption::None);

		let opt = MultiOption::One(1u8);
		assert_eq!(opt, MultiOption::One(1u8));

		let opt = MultiOption::Many(vec!["o", "p", "t", "i", "o", "n"]);
		assert_eq!(opt, MultiOption::Many(vec!["o", "p", "t", "i", "o", "n"]));
	}

	#[test]
	fn multi_option_none() {
		let opt: MultiOption<usize> = MultiOption::None;
		assert!(opt.is_none());
	}

	#[test]
	fn multi_option_one() {
		let opt: MultiOption<usize> = MultiOption::One(1);
		assert!(opt.is_one());
	}

	#[test]
	fn multi_option_many() {
		let opt: MultiOption<usize> = MultiOption::Many(vec![1, 2, 3]);
		assert!(opt.is_many());
	}
}
