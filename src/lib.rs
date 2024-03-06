//! ## Smart-Read
//! 
//! Complex but easy ways to read user input
//! 
//! <br>
//! 
//! ### Anything that implements `ReadLine` can be used with smart-read's macros, and many implementations are already given
//! 
//! <br>
//! <br>
//! 
//! ## Existing functionalities:
//! 
//! <br>
//! 
//! ### Boundless
//! 
//! These allow you to take any `usize`, `bool`, etc. Example: `read!(YesNoInput)`
//! 
//! Implemented types:
//! ```
//! impl ReadLine for BoolInput
//! impl ReadLine for YesNoInput
//! impl ReadLine for CharInput
//! impl ReadLine for UsizeInput
//! impl ReadLine for IsizeInput
//! impl ReadLine for U8Input, U16Input, U32Input, U64Input, U128Input
//! impl ReadLine for I8Input, I16Input, I32Input, I64Input, I128Input
//! impl ReadLine for F32Input
//! impl ReadLine for F64Input
//! ```
//! 
//! <br>
//! 
//! ### Input Options
//! 
//! These allow you to specify which inputs are allowed. Example: `read!(&["a", "b", "c"])`
//! 
//! Special syntax: `read!(= 1, 2, 3)`
//! 
//! Implemented types:
//! ```
//! impl<T: Display + Clone + PartialEq> ReadLine for &[T]
//! impl<T: Display + Clone + PartialEq> ReadLine for &[T; _]
//! impl<T: Display + Clone + PartialEq> ReadLine for Vec<T>
//! impl<T: Display + Clone + PartialEq> ReadLine for VecDeque<T>
//! impl<T: Display + Clone + PartialEq> ReadLine for LinkedList<T>
//! ```
//! 
//! <br>
//! 
//! ### Ranges
//! 
//! These allow you to take a number within a specified range. Example: `read!(1. .. 100.)`, or `read!(10..)`, etc
//! 
//! Implemented types:
//! ```
//! impl<T: Display + FromStr + PartialOrd<T>> ReadLine for Range<T>
//! impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeInclusive<T>
//! impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeTo<T>
//! impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeFrom<T>
//! impl<T: Display + FromStr + PartialOrd<T>> ReadLine for RangeToInclusive<T>
//! ```
//! 
//! <br>
//! 
//! If you have ideas for more functionality, feel free to open an issue
//! 
//! <br>
//! <br>



#![feature(let_chains)]



pub mod input_options;
pub mod ranges;
pub mod boundless;



use std::{error::Error, io::Write};

/// Just `Result<T, Box<dyn Error>>`, mostly for internal use
pub type BoxResult<T> = Result<T, Box<dyn Error>>;



/// ## Reads a line of text, a number, etc
/// 
/// ## Syntax Options: (every value "struct" must implement ReadLine)
/// 
/// ```
/// read!()
/// read!([default_value])
/// read!(struct)
/// read!([default_value] struct)
/// read!(= option1, option2, ..)
/// read!([default_value] = option1, option2, ..)
/// ```
#[macro_export]
macro_rules! read {
	($($args:tt)*) => {
		smart_read::try_read!($($args)*).unwrap()
	}
}

/// Same as read!(), but returns a result
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



/// Same as read!(), but also gives a prompt
#[macro_export]
macro_rules! prompt {
	($($args:tt)*) => {
		smart_read::try_prompt!($($args)*).unwrap()
	}
}

/// Same as prompt!(), but returns a result
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



/// This is what powers the whole crate. Any struct that implements this can be used with the macros
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

/// small utility function, mostly for internal use
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
