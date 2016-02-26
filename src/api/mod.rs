pub mod items;


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
}
