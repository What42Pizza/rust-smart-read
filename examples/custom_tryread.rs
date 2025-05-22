use smart_read::*;



struct PasswordInput {
	pub min_len: usize,
	pub min_digits: usize,
}

// take in a password input
fn main() {
	let input = read!(PasswordInput {min_len: 10, min_digits: 1});
	println!("You entered: \"{input}\"");
}



impl TryRead for PasswordInput {
	type Output = String;
	type Default = (); // ensure no default can be given
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> smart_read::BoxResult<Self::Output> {
		if default.is_some() {return DefaultNotAllowedError::new_box_result();}
		let prompt = prompt.unwrap_or_else(
			|| format!("Enter a password (must have {}+ characters and have {}+ digits): ", self.min_len, self.min_digits)
		);
		loop {
			
			print!("{prompt}");
			let password = read_stdin()?;
			
			if password.len() < 10 {
				println!("Password must have at least 10 characters");
				continue;
			}
			if password.chars().filter(|c| c.is_digit(10)).count() < 1 {
				println!("Password must have at least 1 digit");
				continue;
			}
			
			return Ok(password)
			
		}
	}
}
