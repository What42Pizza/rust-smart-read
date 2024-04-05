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
#![warn(clippy::todo, clippy::unwrap_used, clippy::panic)]

use std::{error::Error, io::Write};



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
		use smart_read::{TryRead, TryReadArgs};
		let (default, (tryread_struct)) = smart_read::parse_default_arg!($($args)*);
		tryread_struct.try_read_line(TryReadArgs {
			prompt: None,
			default: default,
		})
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
		use smart_read::{TryRead, TryReadArgs};
		let (default, (tryread_struct)) = smart_read::parse_default_arg!($($args)*);
		tryread_struct.try_read_line(TryReadArgs {
			prompt: Some($prompt.to_string()),
			default: default,
		})
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
	type Output;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output>;
}



/// This contains all possible information about the read / prompt
pub struct TryReadArgs<Output> {
	pub prompt: Option<String>,
	pub default: Option<Output>,
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
