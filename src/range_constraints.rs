use crate::*;
use std::{fmt::Display, ops::{Range, RangeBounds, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive}, str::FromStr};



/// Internal utility function
pub fn read_range<T: Display + FromStr + PartialOrd<T>, R: RangeBounds<T>>(range: &R, mut read_args: TryReadArgs<T>, format: fn(&R) -> String) -> BoxResult<T> {
	let mut prompt = match read_args.prompt {
		Some(v) => v,
		None => format!("Enter a number within the range {}: ", format(range)),
	};
	if let Some(default) = read_args.default.as_ref() {
		prompt += &format!(" (default: {default})");
	}
	loop {
		
		print!("{prompt}");
		let output_string = read_string(&mut read_args.input)?;
		if output_string.is_empty() && let Some(default) = read_args.default {
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



impl<T: Display + FromStr + PartialOrd<T>> TryRead for Range<T> {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &Range<impl Display>) -> String {
			format!("[{:.1}, {:.1})", range.start, range.end)
		}
		read_range(self, read_args, format)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> TryRead for RangeInclusive<T> {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeInclusive<impl Display>) -> String {
			format!("[{:.1}, {:.1}]", range.start(), range.end())
		}
		read_range(self, read_args, format)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> TryRead for RangeTo<T> {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeTo<impl Display>) -> String {
			format!(".., {:.1})", range.end)
		}
		read_range(self, read_args, format)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> TryRead for RangeFrom<T> {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeFrom<impl Display>) -> String {
			format!("[{:.1}, ..", range.start)
		}
		read_range(self, read_args, format)
	}
}

impl<T: Display + FromStr + PartialOrd<T>> TryRead for RangeToInclusive<T> {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeToInclusive<impl Display>) -> String {
			format!(".., {:.1}]", range.end)
		}
		read_range(self, read_args, format)
	}
}
