use smart_read::*;



// take in a password input
fn main() {
	let input = read!(PasswordInput {min_len: 10, min_digits: 1});
	println!("You entered: \"{input}\"");
}



struct PasswordInput {
	pub min_len: usize,
	pub min_digits: usize,
}

impl TryRead for PasswordInput {
	type Output = String;
	type Default = (); // ensure no default can be given
	fn try_read_line(self, prompt: Option<String>, _default: Option<Self::Default>) -> smart_read::BoxResult<Self::Output> {
		let prompt = prompt.unwrap_or_else(
			|| format!("Enter a password (must have {}+ characters and have {}+ digits): ", self.min_len, self.min_digits)
		);
		loop {
			
			// you could also do `let password = try_prompt!(prompt)?;`
			print!("{prompt}");
			let password = read_stdin()?;
			
			if password.len() < 10 {
				println!("Invalid, password must have at least 10 characters");
				continue;
			}
			if password.chars().filter(|c| c.is_digit(10)).count() < 1 {
				println!("Invalid, password must have at least 1 digit");
				continue;
			}
			
			return Ok(password)
			
		}
	}
}
