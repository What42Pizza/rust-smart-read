## Smart Read

#### Complex but easy ways to read user input

<br>

### Basic Usage

```
let _ = read!(); // read a line of text

let _ = prompt!(); // prompt a line of text

let _ = read!(["two"]); // read a line of text with a default 

let _ = read!(= "red", "green", "blue"); // receive specific inputs
let _ = prompt!("Enter a color: "; = "red", "green", "blue");
let _ = prompt!("Enter a color: "; &["red", "green", "blue"]); // same as line above

let _ = read!(0. ..= 100.); // take a number within a range

let _ = prompt!("Enter an int: "; [1] = 1, 2, 3, 4, 5); // combine anything
```

<br>

### Example stdout

```
Enter a number within the range [0.0, 100.0]:
100.0001
Invalid input.
Enter a number within the range [0.0, 100.0]:
aa
Could not parse input
Enter a number within the range [0.0, 100.0]:
1.
```

<br>

### Extend Existing Functionality

```
use smart_read::read;

fn main() {
	let input = read!(= Car::new("Red", "Toyota"), Car::new("Silver", "Ram"));
	println!("You chose: {input}");
}

#[derive(Clone, PartialEq)]
pub struct Car {
	pub name: String,
	pub color: String,
}

impl Car {
	pub fn new(color: impl Into<String>, name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			color: color.into(),
		}
	}
}

impl std::fmt::Display for Car {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {}", self.color, self.name)
	}
}
```

<br>

### Add New Functionality

```
use smart_read::{read, read_string, ReadLine};

fn main() {
	let input = read!(PasswordInput {min_len: 10, min_digits: 1});
	println!("You entered: \"{input}\"");
}

struct PasswordInput {
	pub min_len: usize,
	pub min_digits: usize,
}

impl ReadLine for PasswordInput {
	type Output = String;
	fn try_read_line(&self, prompt: Option<String>, default: Option<String>) -> smart_read::BoxResult<Self::Output> {
		assert!(default.is_none());
		let prompt = prompt.unwrap_or_else(|| format!("Enter a password (must have {}+ characters and have {}+ digits): ", self.min_len, self.min_digits));
		loop {
			
			print!("{prompt}");
			let password = read_string()?;
			
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

```
