use crate::{read_string, BoxResult, ReadData, ReadLine};
use std::{fmt::Display, ops::{Range, RangeBounds, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive}, str::FromStr};



/// Internal utility function
pub fn read_range<'a, T: Display + FromStr + PartialOrd<T>, R: RangeBounds<T>>(range: &R, mut read_data: ReadData<T>, format: fn(&R) -> String) -> BoxResult<T> {
	let mut prompt = match read_data.prompt {
		Some(v) => v,
		None => format!("Enter a number within the range {}: ", format(range)),
	};
	if let Some(default) = read_data.default.as_ref() {
		prompt += &format!(" (default: {default})");
	}
	loop {
		
		print!("{prompt}");
		let output_string = read_string(&mut read_data.input)?;
		if output_string.is_empty() && let Some(default) = read_data.default {
			return Ok(default);
		}
		
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



impl<'a, T: Display + FromStr + PartialOrd<T>> ReadLine for Range<T> {
	type Output = T;
	fn try_read_line(&self, read_data: ReadData<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &Range<impl Display>) -> String {
			format!("[{:.1}, {:.1})", range.start, range.end)
		}
		read_range(self, read_data, format)
	}
}

impl<'a, T: Display + FromStr + PartialOrd<T>> ReadLine for RangeInclusive<T> {
	type Output = T;
	fn try_read_line(&self, read_data: ReadData<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeInclusive<impl Display>) -> String {
			format!("[{:.1}, {:.1}]", range.start(), range.end())
		}
		read_range(self, read_data, format)
	}
}

impl<'a, T: Display + FromStr + PartialOrd<T>> ReadLine for RangeTo<T> {
	type Output = T;
	fn try_read_line(&self, read_data: ReadData<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeTo<impl Display>) -> String {
			format!(".., {:.1})", range.end)
		}
		read_range(self, read_data, format)
	}
}

impl<'a, T: Display + FromStr + PartialOrd<T>> ReadLine for RangeFrom<T> {
	type Output = T;
	fn try_read_line(&self, read_data: ReadData<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeFrom<impl Display>) -> String {
			format!("[{:.1}, ..", range.start)
		}
		read_range(self, read_data, format)
	}
}

impl<'a, T: Display + FromStr + PartialOrd<T>> ReadLine for RangeToInclusive<T> {
	type Output = T;
	fn try_read_line(&self, read_data: ReadData<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeToInclusive<impl Display>) -> String {
			format!(".., {:.1}]", range.end)
		}
		read_range(self, read_data, format)
	}
}
