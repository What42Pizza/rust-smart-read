// started      24/03/05
// last updated 24/03/05



pub mod input_options;
pub mod ranges;



use std::{error::Error, io::Write};

pub type BoxResult<T> = Result<T, Box<dyn Error>>;



/// reads a line of text, a number, etc
/// 
/// Note: Some ReadLine implementations (currently, `ReadLine for Vec<T>`) have special syntax. All other implementations have to use read!(argument) or prompt!(argument)
/// 
/// Existing functionalities:
/// 
/// 
/// Input Options
/// These allow you to choose between multiple options. Example: read!(= 1, 2, 3), or read!(&["a", "b", "c"]), etc
/// 
/// Implemented types:
/// impl<T: ToString + Clone>    ReadLine for &[T]
/// impl<T: ToString + Clone>    ReadLine for &[T; LEN]
/// impl<T: ToString + Clone>    ReadLine for Vec<T>
/// impl<T: ToString + Clone>    ReadLine for VecDeque<T>
/// impl<K, T: ToString + Clone> ReadLine for HashMap<K, T>
/// impl<T: ToString + Clone>    ReadLine for LinkedList<T>
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
	($input:expr) => {{
		use smart_read::ReadLine;
		$input.try_read_line()
	}};
	(= $($input:expr),*) => {{
		use smart_read::ReadLine;
		let mut choices = vec!();
		$(choices.push($input);)*
		choices.try_read_line()
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
	($prompt:expr; $input:expr) => {{
		use smart_read::ReadLine;
		println!("{}", $prompt);
		$input.try_read_line()
	}};
	($prompt:expr; = $($input:expr),*) => {{
		use smart_read::ReadLine;
		let mut choices = vec!();
		$(choices.push($input);)*
		println!("{}", $prompt);
		choices.try_read_line()
	}};
}



/// This is what powers the whole crate. Anything that implements this can be passed to `read!()` and `try_read!()`
pub trait ReadLine {
	type Output;
	fn try_read_line(&self) -> BoxResult<Self::Output>;
	fn read_line(&self) -> Self::Output {
		self.try_read_line().unwrap()
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
