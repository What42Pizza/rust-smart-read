use crate::{ReadLine, read_string, BoxResult};
use std::{fmt::Display, ops::{RangeBounds, Range, RangeInclusive, RangeTo, RangeFrom, RangeToInclusive}, str::FromStr};



pub fn read_range<T: Display + FromStr + PartialOrd<T>, R: RangeBounds<T>>(range: &R, format: fn(&R) -> String) -> BoxResult<T> {
	loop {
		println!("Enter a number within the range {}:", format(&range));
		let output_string = read_string()?;
		let output = match output_string.parse::<T>() {
			Ok(v) => v,
			Err(_) => {
				println!("Could not parse input");
				continue;
			}
		};
		if range.contains(&output) {
			return Ok(output);
		}
		println!("Invalid input.");
	}
}



impl<T: Display + FromStr + PartialOrd<T>> ReadLine for Range<T> {
	type Output = T;
	fn try_read_line(&self) -> BoxResult<Self::Output> {
		fn format(range: &Range<impl Display>) -> String {
			format!("[{:.1}, {:.1})", range.start, range.end)
		}
		Ok(read_range(self, format)?)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeInclusive<T> {
	type Output = T;
	fn try_read_line(&self) -> BoxResult<Self::Output> {
		fn format(range: &RangeInclusive<impl Display>) -> String {
			format!("[{:.1}, {:.1}]", range.start(), range.end())
		}
		Ok(read_range(self, format)?)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeTo<T> {
	type Output = T;
	fn try_read_line(&self) -> BoxResult<Self::Output> {
		fn format(range: &RangeTo<impl Display>) -> String {
			format!(".., {:.1})", range.end)
		}
		Ok(read_range(self, format)?)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeFrom<T> {
	type Output = T;
	fn try_read_line(&self) -> BoxResult<Self::Output> {
		fn format(range: &RangeFrom<impl Display>) -> String {
			format!("[{:.1}, ..", range.start)
		}
		Ok(read_range(self, format)?)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeToInclusive<T> {
	type Output = T;
	fn try_read_line(&self) -> BoxResult<Self::Output> {
		fn format(range: &RangeToInclusive<impl Display>) -> String {
			format!(".., {:.1}]", range.end)
		}
		Ok(read_range(self, format)?)
	}
}
