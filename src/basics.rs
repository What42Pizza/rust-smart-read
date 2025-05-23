use crate::*;



impl TryRead for () {
	type Output = String;
	type Default = String;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		match (prompt, &default) {
			(Some(prompt), Some(default)) => print!("{prompt}(default: {default}) "),
			(None, Some(default)) => print!("(default: {default}) "),
			(Some(prompt), None) => print!("{prompt}"),
			(None, None) => {},
		}
		let output = read_stdin()?;
		Ok(if output.is_empty() && let Some(default) = default {
			default.to_string()
		} else {
			output
		})
	}
}



/// Takes an input that isn't empty
pub struct NonEmptyInput;

impl TryRead for NonEmptyInput {
	type Output = String;
	type Default = String;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let mut prompt = prompt.unwrap_or_default();
		if let Some(default) = default.as_ref() {
			prompt += &format!("(default: {default}) ");
		}
		loop {
			
			print!("{prompt}");
			let input = read_stdin()?;
			if input.is_empty() {
				println!();
				println!("Invalid input, must not be empty");
				continue;
			}
			return BoxResult::Ok(input);
			
		}
	}
}



/// Takes an input that contains non-whitespace chars
pub struct NonWhitespaceInput;

impl TryRead for NonWhitespaceInput {
	type Output = String;
	type Default = String;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let mut prompt = prompt.unwrap_or_default();
		if let Some(default) = default.as_ref() {
			prompt += &format!("(default: {default}) ");
		}
		loop {
			
			print!("{prompt}");
			let input = read_stdin()?;
			if input.trim().is_empty() {
				println!();
				println!("Invalid input, must contain non-whitespace characters");
				continue;
			}
			return BoxResult::Ok(input);
			
		}
	}
}



/// Allows you to take a bool input
pub struct BoolInput;

impl TryRead for BoolInput {
	type Output = bool;
	type Default = bool;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let mut prompt = prompt.unwrap_or(String::from("Enter a bool: "));
		if let Some(default) = default.as_ref() {
			prompt += &format!("(default: {default}) ");
		}
		loop {
			
			print!("{prompt}");
			let input = read_stdin()?.to_lowercase();
			match (&*input, default) {
				("", Some(default)) => return Ok(default),
				("true", _) | ("t", _) => return Ok(true),
				("false", _) | ("f", _) => return Ok(false),
				(_, _) => {
					println!();
					println!("Invalid input, please enter \"true\" or \"false\"");
				}
			}
			
		}
	}
}



/// Allows you to take a bool input
pub struct YesNoInput;

impl TryRead for YesNoInput {
	type Output = bool;
	type Default = bool;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let mut prompt = prompt.unwrap_or(String::from("Enter 'Yes' or 'No': "));
		if let Some(default) = default.as_ref() {
			prompt += &format!("(default: {}) ", if *default {"Yes"} else {"No"});
		}
		loop {
			
			print!("{prompt}");
			let input = read_stdin()?.to_lowercase();
			match (&*input, default) {
				("", Some(default)) => return Ok(default),
				("yes", _) | ("y", _) => return Ok(true),
				("no", _) | ("n", _) => return Ok(false),
				(_, _) => {
					println!();
					println!("Invalid input, please enter \"yes\" or \"no\"");
				}
			}
			
		}
	}
}



macro_rules! implement_number_input {
	($type_name:tt, $type_base:ty, $default_prompt:expr) => {
		impl TryRead for $type_name {
			type Output = $type_base;
			type Default = $type_base;
			fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
				let mut prompt = prompt.unwrap_or(String::from($default_prompt));
				if let Some(default) = default.as_ref() {
					prompt += &format!("(default: {default}) ");
				}
				loop {
					
					print!("{prompt}");
					let input_string = read_stdin()?;
					if input_string.is_empty() && let Some(default) = default {
						return Ok(default);
					}
					
					let input = match input_string.parse::<$type_base>() {
						Ok(v) => v,
						Err(err) => {
							println!();
							println!("Could not parse input (error: {err})");
							continue;
						}
					};
					return Ok(input);
					
				}
			}
		}
	};
}

/// Allows you take take a char input
pub struct CharInput;
implement_number_input!(CharInput, char, "Enter a character: ");

/// Allows you take take a u8 input
pub struct U8Input;
implement_number_input!(U8Input, u8, "Enter a number (positive integer): ");

/// Allows you take take an i8 input
pub struct I8Input;
implement_number_input!(I8Input, i8, "Enter a number (integer): ");

/// Allows you take take a u16 input
pub struct U16Input;
implement_number_input!(U16Input, u16, "Enter a number (positive integer): ");

/// Allows you take take an i16 input
pub struct I16Input;
implement_number_input!(I16Input, i16, "Enter a number (integer): ");

/// Allows you take take a u32 input
pub struct U32Input;
implement_number_input!(U32Input, u32, "Enter a number (positive integer): ");

/// Allows you take take an i32 input
pub struct I32Input;
implement_number_input!(I32Input, i32, "Enter a number (integer): ");

/// Allows you take take a u64 input
pub struct U64Input;
implement_number_input!(U64Input, u64, "Enter a number (positive integer): ");

/// Allows you take take an i64 input
pub struct I64Input;
implement_number_input!(I64Input, i64, "Enter a number (integer): ");

/// Allows you take take a u128 input
pub struct U128Input;
implement_number_input!(U128Input, u128, "Enter a number (positive integer): ");

/// Allows you take take an i128 input
pub struct I128Input;
implement_number_input!(I128Input, i128, "Enter a number (integer): ");

/// Allows you take take a usize input
pub struct UsizeInput;
implement_number_input!(UsizeInput, usize, "Enter a number (positive integer): ");

/// Allows you take take an isize input
pub struct IsizeInput;
implement_number_input!(IsizeInput, isize, "Enter a number (integer): ");

/// Allows you take take an f32 input
pub struct F32Input;
implement_number_input!(F32Input, f32, "Enter a number: ");

/// Allows you take take an f64 input
pub struct F64Input;
implement_number_input!(F64Input, f64, "Enter a number: ");
