// started      24/03/05
// last updated 24/03/06



#![feature(let_chains)]



pub mod input_options;
pub mod ranges;



use std::{error::Error, io::Write};

pub type BoxResult<T> = Result<T, Box<dyn Error>>;



/// reads a line of text, a number, etc
/// 
/// General syntax:
/// read!([default_value] ...) // works with prompt, like prompt!("Give input: "; [3])
/// 
/// 
/// Existing functionalities:
/// 
/// 
/// Input Options
/// These allow you to specify which inputs are allowed. Example: read!(&["a", "b", "c"])
/// Special syntax: read!(= 1, 2, 3)
/// 
/// Implemented types:
/// impl<T: ToString + Clone> ReadLine for ReadData<T, &[T]>
/// impl<T: ToString + Clone> ReadLine for ReadData<T, &[T; LEN]>
/// impl<T: ToString + Clone> ReadLine for ReadData<T, Vec<T>>
/// impl<T: ToString + Clone> ReadLine for ReadData<T, VecDeque<T>>
/// impl<T: ToString + Clone> ReadLine for ReadData<T, LinkedList<T>>
/// 
/// 
/// Ranges
/// These allow you to take a number within a specified range. Example: read!(1. .. 100.), or read!(10..), etc
/// 
/// Implemented types:
/// impl<T: Display + FromStr + PartialOrd<T>> ReadLine for Range<T>
/// impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeInclusive<T>
/// impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeTo<T>
/// impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeFrom<T>
/// impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeToInclusive<T>
/// 
/// 
/// If you have ideas for more functionality, feel free to open an issue
#[macro_export]
macro_rules! read {
	($($args:tt)*) => {
		smart_read::try_read!($($args)*).unwrap()
	}
}

/// same as read!(), but returns a result
#[macro_export]
macro_rules! try_read {
	() => {
		smart_read::read_string()
	};
	([$default:expr]) => {{
		print!("(default: {}) ", $default);
		smart_read::read_string_or_default($default.to_string())
	}};
	($custom_input:expr) => {{
		use smart_read::ReadLine;
		$custom_input.try_read_line(None, None)
	}};
	([$default:expr] $custom_input:expr) => {{
		use smart_read::ReadLine;
		$custom_input.try_read_line(None, Some($default))
	}};
	(= $($choice:expr),*) => {{
		use smart_read::ReadLine;
		let choices = &[$($choice,)*];
		choices.try_read_line(None, None)
	}};
	([$default:expr] = $($choice:expr),*) => {{
		use smart_read::ReadLine;
		let choices = &[$($choice,)*];
		choices.try_read_line(None, Some($default))
	}};
}



/// prompts a line of text, a number, etc
#[macro_export]
macro_rules! prompt {
	($($args:tt)*) => {
		smart_read::try_prompt!($($args)*).unwrap()
	}
}

/// same as prompt!(), but returns a result
#[macro_export]
macro_rules! try_prompt {
	($prompt:expr) => {{
		print!("{}", $prompt);
		smart_read::read_string()
	}};
	($prompt:expr; [$default:expr]) => {
		print!("{}(default: {}) ", $prompt, $default);
		smart_read::read_string_or_default($default.to_string())
	};
	($prompt:expr; $custom_input:expr) => {{
		use smart_read::ReadLine;
		$custom_input.try_read_line(Some($prompt.to_string()), None)
	}};
	($prompt:expr; [$default:expr] $custom_input:expr) => {{
		use smart_read::ReadLine;
		$custom_input.try_read_line(Some($prompt.to_string()), Some($default))
	}};
	($prompt:expr; = $($choice:expr),*) => {{
		use smart_read::ReadLine;
		let choices = &[$($choice,)*];
		choices.try_read_line(Some($prompt.to_string()), None)
	}};
	($prompt:expr; [$default:expr] = $($choice:expr),*) => {{
		use smart_read::ReadLine;
		let choices = &[$($choice,)*];
		choices.try_read_line(Some($prompt.to_string()), Some($default))
	}};
}



/// This is what powers the whole crate. Any struct that implements this can be passed to read!(), try_read!(), prompt!(), and try_prompt!()
pub trait ReadLine: Sized {
	type Output;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output>;
	fn read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> Self::Output {
		self.try_read_line(prompt, default).unwrap()
	}
}





/// small utility function, mostly for internal use
pub fn read_string() -> BoxResult<String> {
	
	let mut output = String::new();
	std::io::stdout().flush()?;
	std::io::stdin().read_line(&mut output)?;
	
	if output.as_bytes().last() == Some(&10) {output.pop();} // pop \n
	if output.as_bytes().last() == Some(&13) {output.pop();} // pop \r
	
	Ok(output)
}

pub fn read_string_or_default(default: String) -> BoxResult<String> {
	
	let mut output = String::new();
	std::io::stdout().flush()?;
	std::io::stdin().read_line(&mut output)?;
	
	if output.as_bytes().last() == Some(&10) {output.pop();} // pop \n
	if output.as_bytes().last() == Some(&13) {output.pop();} // pop \r
	
	Ok(if output.is_empty() {
		default
	} else {
		output
	})
}
