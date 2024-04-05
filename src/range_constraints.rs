use crate::*;
use std::{ops::{Range, RangeBounds, RangeFrom, RangeInclusive, RangeTo, RangeToInclusive}, str::FromStr};



/// Internal utility function
pub fn read_range<T, R>(range: &R, read_args: TryReadArgs<T>, format: fn(&R) -> String) -> BoxResult<T>
where
	T: Display + FromStr + PartialOrd<T>,
	R: RangeBounds<T>,
	<T as FromStr>::Err: Display,
{
	let mut prompt = match read_args.prompt {
		Some(v) => v,
		None => format!("Enter a number within the range {}: ", format(range)),
	};
	if let Some(default) = read_args.default.as_ref() {
		prompt += &format!(" (default: {default})");
	}
	loop {
		
		print!("{prompt}");
		let output_string = read_stdin()?;
		if output_string.is_empty() && let Some(default) = read_args.default {
			return Ok(default);
		}
		
		let output = match output_string.parse::<T>() {
			Ok(v) => v,
			Err(err) => {
				println!();
				println!("Could not parse input (error: {err})");
				continue;
			}
		};
		if range.contains(&output) {
			return Ok(output);
		}
		
		println!();
		println!("Invalid input, not within bounds");
	}
}



impl<T> TryRead for Range<T>
where
	T: Display + FromStr + PartialOrd<T>,
	<T as FromStr>::Err: Display,
{
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &Range<impl Display>) -> String {
			format!("[{:.1}, {:.1})", range.start, range.end)
		}
		read_range(self, read_args, format)
	}
}

impl<T> TryRead for RangeInclusive<T>
where
	T: Display + FromStr + PartialOrd<T>,
	<T as FromStr>::Err: Display,
{
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeInclusive<impl Display>) -> String {
			format!("[{:.1}, {:.1}]", range.start(), range.end())
		}
		read_range(self, read_args, format)
	}
}

impl<T> TryRead for RangeTo<T>
where
	T: Display + FromStr + PartialOrd<T>,
	<T as FromStr>::Err: Display,
{
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeTo<impl Display>) -> String {
			format!(".., {:.1})", range.end)
		}
		read_range(self, read_args, format)
	}
}

impl<T> TryRead for RangeFrom<T>
where
	T: Display + FromStr + PartialOrd<T>,
	<T as FromStr>::Err: Display,
{
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeFrom<impl Display>) -> String {
			format!("[{:.1}, ..", range.start)
		}
		read_range(self, read_args, format)
	}
}

impl<T> TryRead for RangeToInclusive<T>
where
	T: Display + FromStr + PartialOrd<T>,
	<T as FromStr>::Err: Display,
{
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		fn format(range: &RangeToInclusive<impl Display>) -> String {
			format!(".., {:.1}]", range.end)
		}
		read_range(self, read_args, format)
	}
}
