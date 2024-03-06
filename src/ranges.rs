use crate::{read_string, BoxResult, ReadLine};
use std::{fmt::Display, ops::{Range, RangeBounds, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive}, str::FromStr};



pub fn read_range<T: Display + FromStr + PartialOrd<T>, R: RangeBounds<T>>(range: &R, prompt: Option<String>, default: Option<T>, format: fn(&R) -> String) -> BoxResult<T> {
	let prompt = match prompt {
		Some(v) => v,
		None => format!("Enter a number within the range {}:", format(&range)),
	};
	loop {
		println!("{prompt}");
		let output_string = read_string()?;
		if output_string.is_empty() && let Some(default) = default {
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



impl<T: Display + FromStr + PartialOrd<T>> ReadLine for Range<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<T>) -> BoxResult<Self::Output> {
		fn format(range: &Range<impl Display>) -> String {
			format!("[{:.1}, {:.1})", range.start, range.end)
		}
		Ok(read_range(self, prompt, default, format)?)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeInclusive<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<T>) -> BoxResult<Self::Output> {
		fn format(range: &RangeInclusive<impl Display>) -> String {
			format!("[{:.1}, {:.1}]", range.start(), range.end())
		}
		Ok(read_range(self, prompt, default, format)?)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeTo<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<T>) -> BoxResult<Self::Output> {
		fn format(range: &RangeTo<impl Display>) -> String {
			format!(".., {:.1})", range.end)
		}
		Ok(read_range(self, prompt, default, format)?)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeFrom<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<T>) -> BoxResult<Self::Output> {
		fn format(range: &RangeFrom<impl Display>) -> String {
			format!("[{:.1}, ..", range.start)
		}
		Ok(read_range(self, prompt, default, format)?)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeToInclusive<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<T>) -> BoxResult<Self::Output> {
		fn format(range: &RangeToInclusive<impl Display>) -> String {
			format!(".., {:.1}]", range.end)
		}
		Ok(read_range(self, prompt, default, format)?)
	}
}
