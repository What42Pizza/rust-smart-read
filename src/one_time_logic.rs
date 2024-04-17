use crate::*;



/// Allows you to keep reading until a condition is valid
pub struct SimpleValidate<F: Fn(&str) -> Result<(), String>>(pub F);

impl<F: Fn(&str) -> Result<(), String>> TryRead for SimpleValidate<F> {
	type Output = String;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let mut prompt = prompt.unwrap_or_default();
		if let Some(default) = default.as_ref() {
			prompt += &format!("(default: {default}) ");
		}
		loop {
			
			print!("{prompt}");
			let input = read_stdin()?;
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



/// Allows you to keep reading until a transform is valid
pub struct TransformValidate<F: Fn(String) -> Result<O, String>, O: Display>(pub F);

impl<F: Fn(String) -> Result<O, String>, O: Display> TryRead for TransformValidate<F, O> {
	type Output = O;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let mut prompt = prompt.unwrap_or_default();
		if let Some(default) = default.as_ref() {
			prompt += &format!("(default: {default}) ");
		}
		loop {
			
			print!("{prompt}");
			let input = read_stdin()?;
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
