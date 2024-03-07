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
//! <br>
//! 
//! ## Extra Functionality:
//! 
//! Additional data can be added at the start of `read!()` / `prompt!()`. In order, these additions are:
//! 
//! <br>
//! 
//! ### Prompt Message
//! 
//! `prompt_value;` (only available with prompt!())
//! 
//! <br>
//! 
//! ### Default Value
//! 
//! `[default_value]`
//! 
//! <br>
//! 
//! #### Example: &nbsp; `prompt!("Enter an int: "; [1] = 1, 2, 3, 4, 5)`
//! 
//! <br>
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



/// ## Reads a line of text, a number, etc
#[macro_export]
macro_rules! read {
	($($args:tt)*) => {
		smart_read::try_read!($($args)*).unwrap()
	}
}

/// Same as read!(), but returns a result
#[macro_export]
macro_rules! try_read {
	
	($($args:tt)*) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, parse_input_arg, stdin_as_input};
		let args = parse_input_arg!($($args)*);
		let input = Box::new(stdin_as_input()?);
		let (read_args, readline_struct) = args.finalize(input);
		readline_struct.try_read_line(read_args)
	}()}};
	
}



/// Same as read!(), but also prints a prompt
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
		use smart_read::{Input, read_string, stdin_as_input};
		print!("{}", $prompt);
		let stdin = Box::new(stdin_as_input()?);
		let mut input = Input {iter: stdin, needs_std_flush: true};
		read_string(&mut input)
	}()}};
	
	($prompt:expr; $($args:tt)*) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{ReadLine, parse_input_arg, stdin_as_input};
		let mut args = parse_input_arg!($($args)*);
		args.set_prompt = Some($prompt.to_string());
		let stdin = Box::new(stdin_as_input()?);
		let (read_args, readline_struct) = args.finalize(stdin);
		readline_struct.try_read_line(read_args)
	}()}};
	
}



#[macro_export]
#[doc(hidden)]
macro_rules! parse_input_arg {
	
	() => {{
		use smart_read::ReadArgs;
		let mut output = ReadArgs::default();
		output.set_readline_struct = Some(());
		output
	}};
	
	($input:expr => $($args:tt)*) => {{
		use smart_read::{Input, IterBytes, ReadArgs, parse_default_arg};
		let input = Input::new(Box::new($input.iter_bytes()));
		smart_read::ReadArgs {
			set_input: Some(input),
			set_prompt: None,
			set_default: None,
			set_readline_struct: None,
		}.extend(parse_default_arg!($($args)*))
	}};
	
	($($args:tt)*) => {smart_read::parse_default_arg!($($args)*)}
	
}



#[macro_export]
#[doc(hidden)]
macro_rules! parse_default_arg {
	
	() => {{
		use smart_read::ReadArgs;
		let mut output = ReadArgs::default();
		output.set_readline_struct = Some(());
		output
	}};
	
	([$default:expr] $($args:tt)*) => {{
		use smart_read::{ReadArgs, parse_final_args};
		smart_read::ReadArgs {
			set_input: None,
			set_prompt: None,
			set_default: Some($default.into()),
			set_readline_struct: None,
		}.extend(parse_final_args!($($args)*))
	}};
	
	($($args:tt)*) => {smart_read::parse_final_args!($($args)*)}
	
}



#[macro_export]
#[doc(hidden)]
macro_rules! parse_final_args {
	
	() => {{
		let mut output = smart_read::ReadArgs::default();
		output.set_readline_struct = Some(());
		output
	}};
	
	(= $($choice:expr),*) => {{
		let choices = vec!($($choice,)*);
		smart_read::ReadArgs {
			set_input: None,
			set_prompt: None,
			set_default: None,
			set_readline_struct: Some(choices),
		}
	}};
	
	($readline_struct:expr) => {{
		smart_read::ReadArgs {
			set_input: None,
			set_prompt: None,
			set_default: None,
			set_readline_struct: Some($readline_struct),
		}
	}}
	
}





/// This is what powers the whole crate. Any struct that implements this can be used with the macros
pub trait ReadLine {
	type Output;
	fn try_read_line(&self, read_data: ReadData<Self::Output>) -> BoxResult<Self::Output>;
	fn read_line(&self, read_data: ReadData<Self::Output>) -> Self::Output {
		self.try_read_line(read_data).unwrap()
	}
}

/// This contains all possible information about the read / prompt
pub struct ReadData<Item> {
	pub input: Input,
	pub prompt: Option<String>,
	pub default: Option<Item>,
}

/// Specifies the source of user input
pub struct Input {
	pub iter: Box<dyn Iterator<Item = BoxResult<u8>>>,
	pub needs_std_flush: bool,
}

impl Input {
	pub fn new(iter: Box<dyn Iterator<Item = BoxResult<u8>>>) -> Self {
		Self {
			iter,
			needs_std_flush: false,
		}
	}
}



#[doc(hidden)]
#[derive(Default)]
pub struct ReadArgs<Item, Struct: ReadLine> {
	pub set_input: Option<Input>,
	pub set_prompt: Option<String>,
	pub set_default: Option<Item>,
	pub set_readline_struct: Option<Struct>,
}

impl<Item, Struct: ReadLine> ReadArgs<Item, Struct> {
	pub fn extend(mut self, other: ReadArgs<Item, Struct>) -> Self {
		if other.set_input.is_some() {
			self.set_input = other.set_input;
		}
		if other.set_prompt.is_some() {
			self.set_prompt = other.set_prompt;
		}
		if other.set_default.is_some() {
			self.set_default = other.set_default;
		}
		if other.set_readline_struct.is_some() {
			self.set_readline_struct = other.set_readline_struct;
		}
		self
	}
	pub fn finalize(self, stdin: Box<dyn Iterator<Item = BoxResult<u8>>>) -> (ReadData<Item>, Struct) {
		let input = Input {iter: stdin, needs_std_flush: true};
		let read_data = ReadData {
			input: self.set_input.unwrap_or(input),
			prompt: self.set_prompt,
			default: self.set_default,
		};
		(read_data, self.set_readline_struct.unwrap())
	}
}





impl ReadLine for () {
	type Output = String;
	fn try_read_line(&self, mut read_data: ReadData<Self::Output>) -> BoxResult<Self::Output> {
		match (read_data.prompt, &read_data.default) {
			(Some(prompt), Some(default)) => print!("{prompt}(default: {default}) "),
			(None, Some(default)) => print!("(default: {default}) "),
			(Some(prompt), None) => print!("{prompt}"),
			(None, None) => {},
		}
		let output = read_string(&mut read_data.input)?;
		Ok(if output.is_empty() && let Some(default) = read_data.default {
			default
		} else {
			output
		})
	}
}





pub trait IterBytes {
	fn iter_bytes(&self) -> impl Iterator<Item = BoxResult<u8>>;
}

impl<T: AsRef<str>> IterBytes for T {
	fn iter_bytes(&self) -> impl Iterator<Item = BoxResult<u8>> {
		self.as_ref().bytes().map(Ok)
	}
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
pub fn stdin_as_input() -> BoxResult<impl Iterator<Item = BoxResult<u8>>> {
	let output = std::io::stdin()
		.bytes()
		.map(|b|
			b.map_err(|e| Box::new(e) as Box<dyn Error>)
		);
	Ok(output)
}
