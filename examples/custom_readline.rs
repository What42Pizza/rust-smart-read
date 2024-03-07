use smart_read::{read, read_string, ReadData, ReadLine};



struct PasswordInput {
	pub min_len: usize,
	pub min_digits: usize,
}

fn main() {
	let input = read!(PasswordInput {min_len: 10, min_digits: 1});
	println!("You entered: \"{input}\"");
}



impl ReadLine for PasswordInput {
	type Output = String;
	fn try_read_line(&self, mut read_data: ReadData<Self::Output>) -> smart_read::BoxResult<Self::Output> {
		assert!(read_data.default.is_none());
		let prompt = read_data.prompt.unwrap_or_else(
			|| format!("Enter a password (must have {}+ characters and have {}+ digits): ", self.min_len, self.min_digits)
		);
		loop {
			
			print!("{prompt}");
			let password = read_string(&mut read_data.input)?;
			
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
