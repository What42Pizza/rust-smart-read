//! Complex but easy ways to read user input
//! 
//! <br>
//! 
//! ### Functionality in this crate is defined by types that implement `TryRead`.
//! 
//! <br>
//! <br>
//! 
//! # Types that implement `TryRead`:
//! 
//! This is basically a list of all default functionality, if you want to know more about one of these types, the header name is the same as the module which contains the type
//! 
//! <br>
//! 
//! ### Basics
//! 
//! ```
//! impl TryRead for ()                  // requests any string from the user
//! impl TryRead for NonEmptyInput       // requests a non-empty string from the user
//! impl TryRead for NonWhitespaceInput  // requests a non-whitespace string from the user
//! impl TryRead for BoolInput           // requests a true/false/t/f string from the user
//! impl TryRead for YesNoInput          // requests a yes/no/y/n string from the user
//! impl TryRead for CharInput           // requests a single-char string from the user
//! // requests a number from the user that can be converted to a specific type:
//! impl TryRead for U8Input, U16Input, U32Input, U64Input, U128Input, USizeInput
//! impl TryRead for I8Input, I16Input, I32Input, I64Input, I128Input, ISizeInput
//! impl TryRead for F32Input
//! impl TryRead for F64Input
//! ```
//! 
//! <br>
//! 
//! ### Input Validations
//! 
//! These allow you to easily add custom logic to specific reads
//! 
//! ```
//! // requests a string from the user which passes the programmed validation:
//! impl<F> TryRead for SimpleValidate<F> where F: Fn(&str) -> Result<(), String>
//! // similar to `SimpleValidate`, but also transforms the output:
//! impl<F, O> TryRead for TransformValidate<F, O> where F: Fn(String) -> Result<O, String>, O: Display
//! ```
//! 
//! <br>
//! 
//! ### List Constraints
//! 
//! These allow you to specify which inputs are allowed. Example: `read!(["a", "b", "c"])`
//! 
//! NOTE: The default value for these types denotes the index of the default option
//! 
//! ```
//! // requests a string from the user that matches any of the names from the `InputOption`s:
//! impl<Data> TryRead for &[InputOption<Data>]
//! impl<Data> TryRead for &[InputOption<Data>; N]
//! impl<Data> TryRead for [InputOption<Data>; N]
//! // requests a string from the user that matches any value in the list:
//! impl<T: Display> TryRead for &[T] 
//! impl<T: Display> TryRead for [T; N]
//! impl<T: Display> TryRead for Vec<T>
//! impl<T: Display> TryRead for VecDeque<T>
//! impl<T: Display> TryRead for LinkedList<T>
//! ```
//! 
//! <br>
//! 
//! ### Range Constraints
//! 
//! These allow you to take a number within a specified range. Example: `read!(1. ..= 100.)`, `read!(10..)`, etc
//! 
//! ```
//! impl<T> TryRead for Range<T>            where T: Display + FromStr + PartialOrd<T>, <T as FromStr>::Err: Display
//! impl<T> TryRead for RangeInclusive<T>   where T: Display + FromStr + PartialOrd<T>, <T as FromStr>::Err: Display
//! impl<T> TryRead for RangeTo<T>          where T: Display + FromStr + PartialOrd<T>, <T as FromStr>::Err: Display
//! impl<T> TryRead for RangeFrom<T>        where T: Display + FromStr + PartialOrd<T>, <T as FromStr>::Err: Display
//! impl<T> TryRead for RangeToInclusive<T> where T: Display + FromStr + PartialOrd<T>, <T as FromStr>::Err: Display
//! ```
//! 
//! <br>
//! <br>
//! 
//! # Macro Syntax
//! 
//! Prompt macros: &nbsp; `prompt!("message to user"; [default_value] input_type)`
//! 
//! Read macros: &nbsp; `read!([default_value] input_type)`
//! 
//! All components are optional (except the message in prompts) and all are expressions.
//! 
//! Some examples:
//! ```
//! read!([2] 1..=10);  // take a number from 1 to 10, with 2 as the default
//! prompt!(messages[i]; UsizeInput);  // request a positive integer for the current prompt
//! prompt!("continue?"; [true] YesNoInput);  // request a yes/no input with yes being the default
//! ```
//! 
//! <br>
//! 
//! The input type is what determines the functionality of the input. It is another expression, and the type of the resulting value is what determines which impl of `TryRead` is used. For example, if you have `read!(1..10)` then the impl for `Range<i32>` is used. Also, when you have something like `read!(UsizeInput)`, you are creating a new `UsizeInput` value and passing it to the macro.
//! 
//! Some input types have special syntax that can be substituted for the input_type component, they are:
//! 
//! ```
//! // this:
//! read!()
//! // is this:
//! read!(())
//! 
//! // this:
//! read!(= 1, 2, 3)
//! // is this:
//! read!([1, 2, 3])
//! 
//! // this:
//! read!(=
//! 	"1_bulletin"; "1_display_name"; ["1_alt_name_1", ...]; 1_data,
//! 	"2_bulletin"; "2_display_name"; ["2_alt_name_1", ...]; 2_data,
//! 	...
//! )
//! // is this:
//! read!([
//! 	InputOption::new("1_bulletin", vec!("1_display_name", "1_alt_name_1", ...), 1_data),
//! 	InputOption::new("2_bulletin", vec!("2_display_name", "2_alt_name_1", ...), 2_data),
//! 	...
//! ])
//! ```
//! 
//! <br>
//! 
//! And of course, you can combine this special input type syntax with everything else: &nbsp; `prompt!("Enter a color: "; ["red"]  = "red", "green", "blue")`
//! 
//! <br>
//! 
//! If you have ideas for more functionality (including things you've found to be useful yourself), feel free to open an issue / pull request
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
pub mod input_validation;
/// Contains implementations for `&[T]`, `[T; N]`, `Vec<T>`, `read!(= a, b, c)`, etc
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
		wait_for_enter,
		basics::*,
		input_validation::*,
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

/// Same as `read!()`, but returns a result
#[macro_export]
macro_rules! try_read {
	($($args:tt)*) => {
		smart_read::run_with_prompt!(None; $($args)*)
	};
}



/// Same as `read!()`, but also prints a prompt
#[macro_export]
macro_rules! prompt {
	($($args:tt)*) => {
		smart_read::try_prompt!($($args)*).unwrap()
	}
}

/// Same as `prompt!()`, but returns a result
#[macro_export]
macro_rules! try_prompt {
	($prompt:expr) => {
		smart_read::run_with_prompt!(Some($prompt.to_string());)
	};
	($prompt:expr; $($args:tt)*) => {
		smart_read::run_with_prompt!(Some($prompt.to_string()); $($args)*)
	};
}



#[macro_export]
#[doc(hidden)]
macro_rules! run_with_prompt {
	($prompt:expr; [$default:expr] $($args:tt)*) => {
		smart_read::run_with_prompt_and_default!($prompt; Some($default.into()); $($args)*)
	};
	($prompt:expr; $($args:tt)*) => {
		smart_read::run_with_prompt_and_default!($prompt; None; $($args)*)
	};
}



#[macro_export]
#[doc(hidden)]
macro_rules! run_with_prompt_and_default {
	
	($prompt:expr; $default:expr;) => {{
		use smart_read::TryRead;
		().try_read_line($prompt, $default)
	}};
	
	($prompt:expr; $default:expr; = $($option_bulletin:expr; $option_name:expr; [$($option_alt:expr),*]; $option_data:expr,)*) => {{
		use smart_read::TryRead;
		[$(InputOption::new($option_bulletin, &[$option_name.to_string() $(,$option_alt.to_string())*], $option_data)),*].try_read_line($prompt, $default)
	}};
	
	($prompt:expr; $default:expr; = $($option:expr),*) => {{
		use smart_read::TryRead;
		[$($option),*].try_read_line($prompt, $default)
	}};
	
	($prompt:expr; $default:expr; $tryread_type:expr) => {{
		use smart_read::TryRead;
		($tryread_type).try_read_line($prompt, $default)
	}};
	
}





// ================================ TYPES ================================ //



/// Just `Result<T, Box<dyn Error>>`, mostly for internal use
pub type BoxResult<T> = Result<T, Box<dyn Error>>;



/// This is what powers the whole crate. Any type that implements this can be used with the macros
pub trait TryRead {
	/// Defines the output type of `read` and `prompt` macros
	type Output;
	/// Defines the type of the default input
	type Default;
	/// This is what's called by the `read` and `prompt` macros
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output>;
}





// ================================ FUNCTIONS ================================ //



/// Utility function, mostly for internal use
pub fn read_stdin() -> Result<String, std::io::Error> {
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



/// Waits for the user to press enter, prints "Press enter to continue "
/// 
/// This is basically a wrapper for `prompt!("Press enter to continue ")`
pub fn wait_for_enter() {
	// this would be `prompt!("Press...")`, but that causes an error because of scopes
	print!("Press enter to continue ");
	let _ = read_stdin();
}
