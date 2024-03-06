use crate::{read_string, ReadData, ReadLine};



/// Allows you to take a bool input
pub struct BoolInput;

impl<'a> ReadLine<'a> for BoolInput {
	type Output = bool;
	fn try_read_line(&self, mut read_data: ReadData<'a, Self::Output>) -> crate::BoxResult<Self::Output> {
		let mut prompt = read_data.prompt.unwrap_or(String::from("Enter a bool: "));
		if let Some(default) = read_data.default.as_ref() {
			prompt += &format!("(default: {default}) ");
		}
		loop {
			
			print!("{prompt}");
			let input = read_string(&mut read_data.input)?.to_lowercase();
			match (&*input, read_data.default) {
				("", Some(default)) => return Ok(default),
				("true", _) | ("t", _) => return Ok(true),
				("false", _) | ("f", _) => return Ok(false),
				(_, _) => println!("Invalid input."),
			}
			
		}
	}
}



/// Allows you to take a bool input
pub struct YesNoInput;

impl<'a> ReadLine<'a> for YesNoInput {
	type Output = bool;
	fn try_read_line(&self, mut read_data: ReadData<'a, Self::Output>) -> crate::BoxResult<Self::Output> {
		let mut prompt = read_data.prompt.unwrap_or(String::from("Enter 'Yes' or 'No': "));
		if let Some(default) = read_data.default.as_ref() {
			prompt += &format!("(default: {}) ", if *default {"Yes"} else {"No"});
		}
		loop {
			
			print!("{prompt}");
			let input = read_string(&mut read_data.input)?.to_lowercase();
			match (&*input, read_data.default) {
				("", Some(default)) => return Ok(default),
				("yes", _) | ("y", _) => return Ok(true),
				("no", _) | ("n", _) => return Ok(false),
				(_, _) => println!("Invalid input."),
			}
			
		}
	}
}



macro_rules! implement_number_input {
	($type_name:tt, $type_base:ty, $default_prompt:expr) => {
		impl<'a> ReadLine<'a> for $type_name {
			type Output = $type_base;
			fn try_read_line(&self, mut read_data: ReadData<'a, Self::Output>) -> crate::BoxResult<Self::Output> {
				let mut prompt = read_data.prompt.unwrap_or(String::from($default_prompt));
				if let Some(default) = read_data.default.as_ref() {
					prompt += &format!("(default: {default}) ");
				}
				loop {
					
					print!("{prompt}");
					let input_string = read_string(&mut read_data.input)?.to_lowercase();
					if input_string.is_empty() && let Some(default) = read_data.default {
						return Ok(default);
					}
					
					let Ok(input) = input_string.parse::<$type_base>() else {
						println!("Could not parse input.");
						continue;
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

/// Allows you take take a usize input
pub struct UsizeInput;
implement_number_input!(UsizeInput, usize, "Enter an unsigned int: ");

/// Allows you take take an isize input
pub struct IsizeInput;
implement_number_input!(IsizeInput, isize, "Enter an int: ");

/// Allows you take take a u8 input
pub struct U8Input;
implement_number_input!(U8Input, u8, "Enter an int (u8): ");

/// Allows you take take an i8 input
pub struct I8Input;
implement_number_input!(I8Input, i8, "Enter an int (i8): ");

/// Allows you take take a u16 input
pub struct U16Input;
implement_number_input!(U16Input, u16, "Enter an int (u16): ");

/// Allows you take take an i16 input
pub struct I16Input;
implement_number_input!(I16Input, i16, "Enter an int (i16): ");

/// Allows you take take a u32 input
pub struct U32Input;
implement_number_input!(U32Input, u32, "Enter an int (u32): ");

/// Allows you take take an i32 input
pub struct I32Input;
implement_number_input!(I32Input, i32, "Enter an int (i32): ");

/// Allows you take take a u64 input
pub struct U64Input;
implement_number_input!(U64Input, u64, "Enter an int (u64): ");

/// Allows you take take an i64 input
pub struct I64Input;
implement_number_input!(I64Input, i64, "Enter an int (i64): ");

/// Allows you take take a u128 input
pub struct U128Input;
implement_number_input!(U128Input, u128, "Enter an int (u128): ");

/// Allows you take take an i128 input
pub struct I128Input;
implement_number_input!(I128Input, i128, "Enter an int (i128): ");

/// Allows you take take an f32 input
pub struct F32Input;
implement_number_input!(F32Input, f32, "Enter a number: ");

/// Allows you take take an f64 input
pub struct F64Input;
implement_number_input!(F64Input, f64, "Enter a number: ");

