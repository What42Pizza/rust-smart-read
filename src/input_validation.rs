use crate::*;



/// Keeps taking using input until a programmed condition is met
pub struct SimpleValidate<F: Fn(&str) -> Result<(), String>>(pub F);

impl<F: Fn(&str) -> Result<(), String>> TryRead for SimpleValidate<F> {
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
			if input.is_empty() && let Some(default) = default {
				return Ok(default.to_string());
			}
			match (self.0)(&input) {
				Ok(_) => return Ok(input),
				Err(error_message) => {
					println!();
					println!("{error_message}")
				}
			}
			
		}
	}
}



/// Keeps taking using input until a programmed condition and transformation is met
pub struct TransformValidate<F: Fn(String) -> Result<O, String>, O: Display>(pub F);

impl<F: Fn(String) -> Result<O, String>, O: Display> TryRead for TransformValidate<F, O> {
	type Output = O;
	type Default = O;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let mut prompt = prompt.unwrap_or_default();
		if let Some(default) = default.as_ref() {
			prompt += &format!("(default: {default}) ");
		}
		loop {
			
			print!("{prompt}");
			let input = read_stdin()?;
			if input.is_empty() && let Some(default) = default {
				return Ok(default);
			}
			match (self.0)(input) {
				Ok(output) => return Ok(output),
				Err(error_message) => {
					println!();
					println!("{error_message}")
				}
			}
			
		}
	}
}
