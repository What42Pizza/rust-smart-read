//! ## Smart-Read
//! 
//! Complex but easy ways to read user input
//! 
//! <br>
//! 
//! ### Anything that implements the `TryRead` trait can be used with smart-read's macros, and many implementations are already given
//! 
//! <br>
//! <br>
//! 
//! ## Types that implement TryRead:
//! 
//! <br>
//! 
//! ### Basics
//! 
//! ```
//! impl TryRead for ()
//! impl TryRead for NonEmptyInput
//! impl TryRead for NonWhitespaceInput
//! impl TryRead for BoolInput
//! impl TryRead for YesNoInput
//! impl TryRead for CharInput
//! impl TryRead for UsizeInput
//! impl TryRead for IsizeInput
//! impl TryRead for U8Input, U16Input, U32Input, U64Input, U128Input
//! impl TryRead for I8Input, I16Input, I32Input, I64Input, I128Input
//! impl TryRead for F32Input
//! impl TryRead for F64Input
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
//! impl<T: Display + Clone + PartialEq> TryRead for &[T]
//! impl<T: Display + Clone + PartialEq> TryRead for &[T; _]
//! impl<T: Display + Clone + PartialEq> TryRead for Vec<T>
//! impl<T: Display + Clone + PartialEq> TryRead for VecDeque<T>
//! impl<T: Display + Clone + PartialEq> TryRead for LinkedList<T>
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
//! impl<T: Display + FromStr + PartialOrd<T>> TryRead for Range<T>
//! impl<T: Display + FromStr + PartialOrd<T>> TryRead for RangeInclusive<T>
//! impl<T: Display + FromStr + PartialOrd<T>> TryRead for RangeTo<T>
//! impl<T: Display + FromStr + PartialOrd<T>> TryRead for RangeFrom<T>
//! impl<T: Display + FromStr + PartialOrd<T>> TryRead for RangeToInclusive<T>
//! ```
//! 
//! <br>
//! <br>
//! 
//! ## Extra Functionality:
//! 
//! In addition to the type of input, data can be added at the start of `read!()` / `prompt!()`. In order, these additions are:
//! 
//! <br>
//! 
//! ### Custom Input
//! 
//! `input >>` (must implement crate's `IntoInput`)
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
//! #### Example: &nbsp; `prompt!("Enter a color: "; prev_user_input >> ["red"] = "red", "green", "blue")`
//! 
//! <br>
//! <br>
//! 
//! If you have ideas for more functionality (including things that you've found to be useful for yourself), feel free to open an issue
//! 
//! <br>
//! <br>



#![feature(let_chains)]

use std::{error::Error, io::{Read, Write}};



pub mod basics;
pub mod list_constraints;
pub mod range_constraints;

pub mod prelude {
	pub use super::{
		read,
		try_read,
		prompt,
		try_prompt,
		basics::*,
		list_constraints::*,
		range_constraints::*,
	};
}





// ================================ Macros ================================ //



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
		use smart_read::{TryRead, parse_input_arg, stdin_as_input};
		let args = parse_input_arg!($($args)*);
		let (read_args, readline_struct) = args.finalize();
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
	
	($prompt:expr) => {smart_read::try_prompt!($prompt;)};
	
	($prompt:expr; $($args:tt)*) => {{|| -> smart_read::BoxResult<_> {
		use smart_read::{TryRead, parse_input_arg, stdin_as_input};
		let mut args = parse_input_arg!($($args)*);
		args.set_prompt = Some($prompt.to_string());
		let (read_args, readline_struct) = args.finalize();
		readline_struct.try_read_line(read_args)
	}()}};
	
}



#[macro_export]
#[doc(hidden)]
macro_rules! parse_input_arg {
	
	() => {{
		use smart_read::MacroArgs;
		let mut output = MacroArgs::default();
		output.set_readline_struct = Some(());
		output
	}};
	
	($input:tt >> $($args:tt)*) => {{
		use smart_read::{Input, IntoInput, MacroArgs, parse_default_arg};
		//let input = Input::new(Box::new($input.iter_bytes()));
		smart_read::MacroArgs {
			set_input: Some($input.into_input()),
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
		use smart_read::MacroArgs;
		let mut output = MacroArgs::default();
		output.set_readline_struct = Some(());
		output
	}};
	
	([$default:expr] $($args:tt)*) => {{
		use smart_read::{MacroArgs, parse_final_args};
		smart_read::MacroArgs {
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
		let mut output = smart_read::MacroArgs::default();
		output.set_readline_struct = Some(());
		output
	}};
	
	(= $($choice:expr),*) => {{
		let choices = vec!($($choice,)*);
		smart_read::MacroArgs {
			set_input: None,
			set_prompt: None,
			set_default: None,
			set_readline_struct: Some(choices),
		}
	}};
	
	($readline_struct:expr) => {{
		smart_read::MacroArgs {
			set_input: None,
			set_prompt: None,
			set_default: None,
			set_readline_struct: Some($readline_struct),
		}
	}}
	
}





// ================================ TYPES ================================ //



/// Just `Result<T, Box<dyn Error>>`, mostly for internal use
pub type BoxResult<T> = Result<T, Box<dyn Error>>;



/// This is what powers the whole crate. Any struct that implements this can be used with the macros
pub trait TryRead {
	type Output;
	fn try_read_line(&self, read_data: TryReadArgs<Self::Output>) -> BoxResult<Self::Output>;
}



/// This contains all possible information about the read / prompt
pub struct TryReadArgs<Output> {
	pub input: Input,
	pub prompt: Option<String>,
	pub default: Option<Output>,
}



/// Specifies the source of user input
/// 
/// If should_stop is None, it defaults to stopping once \n is read
/// 
/// If clean_output is None, it defaults to removing a trailing \n (if found) then a trailing \r (if found)
pub struct Input {
	pub iter: Box<dyn Iterator<Item = BoxResult<u8>>>,
	pub needs_std_flush: bool,
	pub should_stop: Option<fn(&[u8]) -> bool>,
	pub clean_output: Option<fn(Vec<u8>) -> Vec<u8>>,
}

impl Input {
	pub fn flush_std_if_needed(&self) -> BoxResult<()>{
		if self.needs_std_flush {std::io::stdout().flush()?;}
		Ok(())
	}
}



/// Allows a type to be used as input. Example:
/// 
/// ```
/// pub struct TerminalInput;
/// impl IntoInput for TerminalInput {
/// 	...
/// }
/// 
/// read!(TerminalInput >>);
/// ```
pub trait IntoInput {
	fn into_input(self) -> Input;
}

impl<T: Into<String>> IntoInput for T {
	fn into_input(self) -> Input {
		Input {
			iter: Box::new(self.into().into_bytes().into_iter().map(Ok)),
			needs_std_flush: false,
			should_stop: None,
			clean_output: None,
		}
	}
}

impl IntoInput for Input {
	fn into_input(self) -> Input {
		self
	}
}



#[doc(hidden)]
#[derive(Default)]
pub struct MacroArgs<Output, Struct: TryRead> {
	pub set_input: Option<Input>,
	pub set_prompt: Option<String>,
	pub set_default: Option<Output>,
	pub set_readline_struct: Option<Struct>,
}

impl<Output, Struct: TryRead> MacroArgs<Output, Struct> {
	pub fn extend(mut self, other: MacroArgs<Output, Struct>) -> Self {
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
	pub fn finalize(self) -> (TryReadArgs<Output>, Struct) {
		let read_data = TryReadArgs {
			input: match self.set_input {
				Some(v) => v,
				None => stdin_as_input(),
			},
			prompt: self.set_prompt,
			default: self.set_default,
		};
		(read_data, self.set_readline_struct.unwrap())
	}
}





// ================================ FUNCTIONS ================================ //



/// Utility function, mostly for internal use
pub fn read_string(input: &mut Input) -> BoxResult<String> {
	
	fn default_should_stop(input: &[u8]) -> bool {input.last() == Some(&10)}
	let should_stop = input.should_stop.unwrap_or(default_should_stop);
	fn default_clean_output(mut output: Vec<u8>) -> Vec<u8> {
		if output.last() == Some(&10) {output.pop();} // pop \n
		if output.last() == Some(&13) {output.pop();} // pop \r
		output
	}
	let clean_output = input.clean_output.unwrap_or(default_clean_output);
	
	input.flush_std_if_needed()?;
	let mut output = vec!();
	loop {
		let Some(next) = input.iter.next() else {break};
		output.push(next?);
		if should_stop(&output) {break}
	}
	let output = clean_output(output);
	let output = String::from_utf8(output)?;
	
	Ok(output)
}



/// Utility function, mostly for internal use
pub fn stdin_as_input() -> Input {
	let output = std::io::stdin()
		.bytes()
		.map(|b|
			b.map_err(|e| Box::new(e) as Box<dyn Error>)
		);
	let output = Input {
		iter: Box::new(output),
		needs_std_flush: true,
		should_stop: None,
		clean_output: None,
	};
	output
}
