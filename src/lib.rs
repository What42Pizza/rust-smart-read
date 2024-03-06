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



use std::{error::Error, io::{Read, Write}};

/// Just `Result<T, Box<dyn Error>>`, mostly for internal use
pub type BoxResult<T> = Result<T, Box<dyn Error>>;

pub struct Input<'a> {
	pub iter: &'a mut dyn Iterator<Item = BoxResult<u8>>,
	pub needs_std_flush: bool,
}



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
	
	() => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, Input, stdin_as_input, read_string};
		let mut input = stdin_as_input()?;
		read_string(&mut Input {iter: &mut input, needs_std_flush: true})
	}()}};
	
	([$default:expr]) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, Input, stdin_as_input, read_string_or_default};
		print!("(default: {}) ", $default);
		let mut input = stdin_as_input()?;
		read_string_or_default(&mut Input {iter: &mut input, needs_std_flush: true}, $default.to_string())
	}()}};
	
	($custom_input:expr) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, ReadData, Input, stdin_as_input};
		let mut input = stdin_as_input()?;
		let read_data = ReadData {
			input: Input {iter: &mut input, needs_std_flush: true},
			prompt: None,
			default: None,
		};
		$custom_input.try_read_line(read_data)
	}()}};
	
	([$default:expr] $custom_input:expr) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, ReadData, Input, stdin_as_input};
		let mut input = stdin_as_input()?;
		let read_data = ReadData {
			input: Input {iter: &mut input, needs_std_flush: true},
			prompt: None,
			default: Some($default),
		};
		$custom_input.try_read_line(read_data)
	}()}};
	
	(= $($choice:expr),*) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, ReadData, Input, stdin_as_input};
		let mut input = stdin_as_input()?;
		let read_data = ReadData {
			input: Input {iter: &mut input, needs_std_flush: true},
			prompt: None,
			default: None,
		};
		let choices = &[$($choice,)*];
		choices.try_read_line(read_data)
	}()}};
	
	([$default:expr] = $($choice:expr),*) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, ReadData, Input, stdin_as_input};
		let mut input = stdin_as_input()?;
		let read_data = ReadData {
			input: Input {iter: &mut input, needs_std_flush: true},
			prompt: None,
			default: Some($default),
		};
		let choices = &[$($choice,)*];
		choices.try_read_line(read_data)
	}()}};
	
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
	
	($prompt:expr) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, Input, stdin_as_input, read_string};
		print!("{}", $prompt);
		let mut input = stdin_as_input()?;
		read_string(&mut Input {iter: &mut input, needs_std_flush: true})
	}()}};
	
	($prompt:expr; [$default:expr]) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, Input, stdin_as_input, read_string_or_default};
		print!("{}(default: {}) ", $prompt, $default);
		let mut input = stdin_as_input()?;
		read_string_or_default(&mut Input {iter: &mut input, needs_std_flush: true}, $default.to_string())
	}()}};
	
	($prompt:expr; $custom_input:expr) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, ReadData, Input, stdin_as_input};
		let mut input = stdin_as_input()?;
		let read_data = ReadData {
			input: Input {iter: &mut input, needs_std_flush: true},
			prompt: Some($prompt.to_string()),
			default: None,
		};
		$custom_input.try_read_line(read_data)
	}()}};
	
	($prompt:expr; [$default:expr] $custom_input:expr) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, ReadData, Input, stdin_as_input};
		let mut input = stdin_as_input()?;
		let read_data = ReadData {
			input: Input {iter: &mut input, needs_std_flush: true},
			prompt: Some($prompt.to_string()),
			default: Some($default),
		};
		$custom_input.try_read_line(read_data)
	}()}};
	
	($prompt:expr; = $($choice:expr),*) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, ReadData, Input, stdin_as_input};
		let mut input = stdin_as_input()?;
		let read_data = ReadData {
			input: Input {iter: &mut input, needs_std_flush: true},
			prompt: Some($prompt.to_string()),
			default: None,
		};
		let choices = &[$($choice,)*];
		choices.try_read_line(read_data)
	}()}};
	
	($prompt:expr; [$default:expr] = $($choice:expr),*) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, ReadData, Input, stdin_as_input};
		let mut input = stdin_as_input()?;
		let read_data = ReadData {
			input: Input {iter: &mut input, needs_std_flush: true},
			prompt: Some($prompt.to_string()),
			default: Some($default),
		};
		let choices = &[$($choice,)*];
		choices.try_read_line(read_data)
	}()}};
	
}



/// This is what powers the whole crate. Any struct that implements this can be used with the macros
pub trait ReadLine<'a>: Sized {
	type Output;
	fn try_read_line(&self, read_data: ReadData<'a, Self::Output>) -> BoxResult<Self::Output>;
	fn read_line(&self, read_data: ReadData<'a, Self::Output>) -> Self::Output {
		self.try_read_line(read_data).unwrap()
	}
}

pub struct ReadData<'a, T> {
	pub input: Input<'a>,
	pub prompt: Option<String>,
	pub default: Option<T>,
}





/// Utility function, mostly for internal use
pub fn read_string(input: &mut Input) -> BoxResult<String> {
	
	if input.needs_std_flush {std::io::stdout().flush()?;}
	
	let mut output = vec!();
	while output.last() != Some(&b'\n') {
		output.push(input.iter.next().unwrap()?);
	}
	if output.last() == Some(&10) {output.pop();} // pop \n
	if output.last() == Some(&13) {output.pop();} // pop \r
	let output = String::from_utf8(output)?;
	
	Ok(output)
}

/// Utility function, mostly for internal use
pub fn read_string_or_default(input: &mut Input, default: String) -> BoxResult<String> {
	let output = read_string(input)?;
	Ok(if output.is_empty() {
		default
	} else {
		output
	})
}

/// Utility function, mostly for internal use
pub fn stdin_as_input() -> BoxResult<impl Iterator<Item = BoxResult<u8>>> {
	let output = std::io::stdin()
		.bytes()
		.map(|b|
			b.map_err(|e| Box::new(e) as Box<dyn Error>)
		);
	Ok(output)
}
