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
//! ## Types that implement TryRead &nbsp; (basically, a list of all default functionality):
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
//! impl<F: Fn(String) -> Result<O, String>, O: Display> TryRead for TransformValidate<F, O>
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
//! // NOTE: If the options are filtered before being fed into smart-read, you should probably use OptionWithData<T>
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
//! impl<T> TryRead for Range<T>            where T: Display + FromStr + PartialOrd<T>, <T as FromStr>::Err: Display,
//! impl<T> TryRead for RangeInclusive<T>   where T: Display + FromStr + PartialOrd<T>, <T as FromStr>::Err: Display,
//! impl<T> TryRead for RangeTo<T>          where T: Display + FromStr + PartialOrd<T>, <T as FromStr>::Err: Display,
//! impl<T> TryRead for RangeFrom<T>        where T: Display + FromStr + PartialOrd<T>, <T as FromStr>::Err: Display,
//! impl<T> TryRead for RangeToInclusive<T> where T: Display + FromStr + PartialOrd<T>, <T as FromStr>::Err: Display,
//! ```
//! 
//! <br>
//! <br>
//! 
//! # Macro Syntax
//! 
//! There are three items that can be included in a macro call (all optional): the prompt message, the default value, and the input type.
//! 
//! <br>
//! 
//! The prompt message is simply an expression, followed by `;` if there's more afterwards. This is required when using the prompt macro, and not available with the read macro.
//! 
//! Examples: &nbsp; `prompt!("Enter any string: ")`, &nbsp; `prompt!(messages[i]; YesNoInput)`
//! 
//! <br>
//! 
//! The default value comes after the prompt message (if given), and must be enclosed in `[]`.
//! 
//! Examples: &nbsp; `read!([1] 0..10)`, &nbsp; `prompt!("Confirm action? "; [true] YesNoInput)`
//! 
//! <br>
//! 
//! The input type is a value that determines how the input is read. You could give a range to read a number within a range, or a `UsizeInput` to read an int, or whatever else implements `TryRead` from this crate (fun fact, leaving this blank will use the impl for `()`).
//! 
//! Examples: &nbsp; `read!()`, &nbsp; `prompt!("Enter a color: "; ["red"] &["red", "green", "blue"])`, &nbsp; `read!(ExampleStruct {arg: 42})`
//! 
//! <br>
//! <br>
//! 
//! # Feature-Specific Syntax
//! 
//! Currently, only one feature has custom syntax, which is the implementation for slices. Instead of `read!(&[item1, item2, ...])`, you can write: `read!(= item1, item2, ...)`
//! 
//! And of course, you can combine this with any other piece of syntax: &nbsp; `prompt!("Enter a color: "; ["red"] = "red", "green", "blue")`
//! 
//! <br>
//! <br>
//! 
//! If you have ideas for more functionality (including things that you've found to be useful for yourself), feel free to open an issue / pull request
//! 
//! <br>
//! <br>



#![feature(let_chains)]
#![allow(clippy::tabs_in_doc_comments, clippy::neg_multiply)]
#![warn(missing_docs, clippy::todo, clippy::unwrap_used, clippy::panic, clippy::expect_used)]

use std::{error::Error, fmt::{Debug, Display}, io::Write};



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
	
	($($args:tt)*) => {|| -> smart_read::BoxResult<_> {
		use smart_read::TryRead;
		let (default, (tryread_struct)) = smart_read::parse_default_arg!($($args)*);
		tryread_struct.try_read_line(None, default)
	}()};
	
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
	
	($prompt:expr; $($args:tt)*) => {|| -> smart_read::BoxResult<_> {
		use smart_read::TryRead;
		let (default, (tryread_struct)) = smart_read::parse_default_arg!($($args)*);
		tryread_struct.try_read_line(Some($prompt.to_string()), default)
	}()};
	
}



#[macro_export]
#[doc(hidden)]
macro_rules! parse_default_arg {
	
	([$default:expr] $($args:tt)*) => {
		(Some($default.into()), smart_read::parse_final_args!($($args)*))
	};
	
	($($args:tt)*) => {
		(None, smart_read::parse_final_args!($($args)*))
	};
	
}



#[macro_export]
#[doc(hidden)]
macro_rules! parse_final_args {
	
	() => {
		()
	};
	
	(= $($choice:expr),*) => {
		vec!($($choice,)*)
	};
	
	($tryread_struct:expr) => {
		$tryread_struct
	}
	
}





// ================================ TYPES ================================ //



/// Just `Result<T, Box<dyn Error>>`, mostly for internal use
pub type BoxResult<T> = Result<T, Box<dyn Error>>;



/// This is what powers the whole crate. Any struct that implements this can be used with the macros
pub trait TryRead {
	/// Defines the output of `read` and `prompt` macros
	type Output;
	/// This is what's called by the `read` and `prompt` macros
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output>;
}



/// Useful pre-made error
#[derive(Debug)]
pub struct DefaultNotAllowedError;

impl Error for DefaultNotAllowedError {}

impl Display for DefaultNotAllowedError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Default value is not allowed for input type.")
	}
}

impl DefaultNotAllowedError {
	/// Easily get a return value
	pub fn new_box_result<T>() -> BoxResult<T> {
		Err(Box::new(Self))
	}
}



/// Useful pre-made error
#[derive(Debug)]
pub struct PromptNotAllowedError;

impl Error for PromptNotAllowedError {}

impl Display for PromptNotAllowedError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "Prompt value is not allowed for input type.")
	}
}

impl PromptNotAllowedError {
	/// Easily get a return value
	pub fn new_box_result<T>() -> BoxResult<T> {
		Err(Box::new(Self))
	}
}





// ================================ FUNCTIONS ================================ //



/// Utility function, mostly for internal use
pub fn read_stdin() -> BoxResult<String> {
	std::io::stdout().flush()?;
	let mut output = String::new();
	std::io::stdin().read_line(&mut output)?;
	if output.ends_with('\n') {output.pop();}
	if output.ends_with('\r') {output.pop();}
	Ok(output)
}



/// Tiny utility function, clears the terminal output, but you should probably use the [ClearScreen](https://crates.io/crates/clearscreen) crate instead
pub fn clear_term() {
	print!("{esc}c", esc = 27 as char);
}
