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
//! ### One-Time Logic
//! 
//! ```
//! impl<F: Fn(&str) -> Result<(), String>> TryRead for SimpleValidate<F>
//! impl<F: Fn(String) -> Result<O, String>, O: Display> TryRead for TransformValidate<O, F>
//! ```
//! 
//! <br>
//! 
//! ### List Constraints
//! 
//! These allow you to specify which inputs are allowed. Example: `read!(&["a", "b", "c"])`
//! 
//! If the choices are wrapped in EnumerateInput, it also returns the index of the chosen option
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
//! impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<&[T]>
//! impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<&[T; _]>
//! impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<Vec<T>>
//! impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<VecDeque<T>>
//! impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<LinkedList<T>>
//! ```
//! 
//! <br>
//! 
//! ### Range Constraints
//! 
//! These allow you to take a number within a specified range. Example: `read!(1. .. 100.)`, `read!(10..)`, etc
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
//! ### Prompt Message
//! 
//! `prompt_value;` (only available with prompt!())
//! 
//! <br>
//! 
//! ### Custom Input
//! 
//! `input >>` (must implement crate's `IntoInput`)
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
#![allow(clippy::tabs_in_doc_comments)]
#![warn(clippy::todo, clippy::unwrap_used)]

use std::{error::Error, io::{Read, Write}};



/// Contains implementations for `()`, `UsizeInput`, `NonEmptyInput`, etc
pub mod basics;
/// Contains implementations for `SimpleValidate` and `TransformValidate`
pub mod one_time_logic;
/// Contains implementations for `Vec<T>`, `read!(= a, b, c)`, etc
pub mod list_constraints;
/// Contains implementations for `Range<T>`, `RangeFrom<T>`, etc
pub mod range_constraints;

/// Easy way to use existing functionality. If you want to extend functionality instead, you can do `use smart_read::*;`
pub mod prelude {
	pub use super::{
		read,
		try_read,
		prompt,
		try_prompt,
		basics::*,
		one_time_logic::*,
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
		use smart_read::TryRead;
		let stage_3 = smart_read::parse_input_arg!($($args)*);
		let (tryread_struct, args) = stage_3.finalize(None);
		tryread_struct.try_read_line(args)
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
		use smart_read::TryRead;
		let stage_3 = smart_read::parse_input_arg!($($args)*);
		let (tryread_struct, args) = stage_3.finalize(Some($prompt.to_string()));
		tryread_struct.try_read_line(args)
	}()}};
	
}



#[macro_export]
#[doc(hidden)]
macro_rules! parse_input_arg {
	
	($input:tt >> $($args:tt)*) => {{
		use smart_read::IntoInput;
		let stage_2 = smart_read::parse_default_arg!($($args)*);
		smart_read::MacroArgsStage3::new(stage_2, $input.into_input())
	}};
	
	($($args:tt)*) => {{
		let stage_2 = smart_read::parse_default_arg!($($args)*);
		smart_read::MacroArgsStage3::new(stage_2, smart_read::stdin_as_input())
	}};
	
}



#[macro_export]
#[doc(hidden)]
macro_rules! parse_default_arg {
	
	([$default:expr] $($args:tt)*) => {{
		let stage_1 = smart_read::parse_final_args!($($args)*);
		smart_read::MacroArgsStage2::new(stage_1, Some($default.into()))
	}};
	
	($($args:tt)*) => {{
		let stage_1 = smart_read::parse_final_args!($($args)*);
		smart_read::MacroArgsStage2::new(stage_1, None)
	}};
	
}



#[macro_export]
#[doc(hidden)]
macro_rules! parse_final_args {
	
	() => {{
		smart_read::MacroArgsStage1 {
			tryread_struct: (),
		}
	}};
	
	(= $($choice:expr),*) => {{
		let choices = vec!($($choice,)*);
		smart_read::MacroArgsStage1 {
			tryread_struct: choices,
		}
	}};
	
	($tryread_struct:expr) => {{
		smart_read::MacroArgsStage1 {
			tryread_struct: $tryread_struct,
		}
	}}
	
}



#[doc(hidden)]
pub struct MacroArgsStage3<Output, Struct: TryRead> {
	pub input: Input,
	pub default: Option<Output>,
	pub tryread_struct: Struct,
}

impl<Output, Struct: TryRead> MacroArgsStage3<Output, Struct> {
	pub fn new(stage_2: MacroArgsStage2<Output, Struct>, input: Input) -> Self {
		Self {
			input,
			default: stage_2.default,
			tryread_struct: stage_2.tryread_struct,
		}
	}
	pub fn finalize(self, prompt: Option<String>) -> (Struct, TryReadArgs<Output>) {
		let args = TryReadArgs {
			input: self.input,
			prompt,
			default: self.default,
		};
		(self.tryread_struct, args)
	}
}



#[doc(hidden)]
pub struct MacroArgsStage2<Output, Struct: TryRead> {
	pub default: Option<Output>,
	pub tryread_struct: Struct,
}

impl<Output, Struct: TryRead> MacroArgsStage2<Output, Struct> {
	pub fn new(stage_1: MacroArgsStage1<Struct>, default: Option<Output>) -> Self {
		Self {
			default,
			tryread_struct: stage_1.tryread_struct,
		}
	}
}



#[doc(hidden)]
pub struct MacroArgsStage1<Struct: TryRead> {
	pub tryread_struct: Struct,
}

impl<Struct: TryRead> MacroArgsStage1<Struct> {
	pub fn new(tryread_struct: Struct) -> Self {
		Self {
			tryread_struct,
		}
	}
}



//#[doc(hidden)]
//#[derive(Default)]
//pub struct MacroArgs<Output, Struct: TryRead> {
//	pub set_input: Option<Input>,
//	pub set_prompt: Option<String>,
//	pub set_default: Option<Output>,
//	pub set_tryread_struct: Option<Struct>,
//}

//impl<Output, Struct: TryRead> MacroArgs<Output, Struct> {
//	pub fn extend(mut self, other: MacroArgs<Output, Struct>) -> Self {
//		if other.set_input.is_some() {
//			self.set_input = other.set_input;
//		}
//		if other.set_prompt.is_some() {
//			self.set_prompt = other.set_prompt;
//		}
//		if other.set_default.is_some() {
//			self.set_default = other.set_default;
//		}
//		if other.set_tryread_struct.is_some() {
//			self.set_tryread_struct = other.set_tryread_struct;
//		}
//		self
//	}
//	pub fn finalize(self) -> (TryReadArgs<Output>, Struct) {
//		let read_data = TryReadArgs {
//			input: match self.set_input {
//				Some(v) => v,
//				None => stdin_as_input(),
//			},
//			prompt: self.set_prompt,
//			default: self.set_default,
//		};
//		(read_data, self.set_tryread_struct.unwrap_or_else(|| panic!("Internal macro error, MacroArgs.set_tryread_struct is None")))
//	}
//}





// ================================ TYPES ================================ //



/// Just `Result<T, Box<dyn Error>>`, mostly for internal use
pub type BoxResult<T> = Result<T, Box<dyn Error>>;



/// This is what powers the whole crate. Any struct that implements this can be used with the macros
pub trait TryRead {
	type Output;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output>;
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
	/// Needs to be called to prevent prints before the read appearing after the read
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
	Input {
		iter: Box::new(output),
		needs_std_flush: true,
		should_stop: None,
		clean_output: None,
	}
}



/// Tiny utility function, clears the terminal output, but you should probably use the [ClearScreen](https://crates.io/crates/clearscreen) crate instead
pub fn clear_term() {
	print!("{esc}c", esc = 27 as char);
}
