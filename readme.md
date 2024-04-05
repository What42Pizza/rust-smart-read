## Smart Read

#### Complex but easy ways to read user input

<br>

### Basic Usage

```rust
let _ = read!(); // read a line of text

let _ = prompt!("Enter a string: "); // prompt a line of text

let _ = read!(["two"]); // read a line of text with a default 

let _ = prompt!("Enter a color: "; &["red", "green", "blue"]); // receive specific inputs
let _ = prompt!("Enter a color: "; = "red", "green", "blue"); // some inputs have special syntax

let _ = read!(0. ..= 100.); // take a number within a range

let _ = prompt!("Confirm input: "; [true] YesNoInput); // read a bool

// get index of chosen option
let mut colors = vec!("red", "green", "blue");
let (index, _item) = prompt!("Which color do you want to remove?"; EnumerateInput(&*colors));
colors.remove(index);

// one-time custom logic
let _ = prompt!("Enter an even int: "; TransformValidate (|x: String| -> Result<isize, String> { // explicit types here are optional, only added for demonstration
	let Ok(x) = x.parse::<isize>() else {return Err(String::from("Could not parse input."));};
	if x % 2 != 0 {return Err(String::from("Input is not even."));}
	Ok(x)
}));

let _ = prompt!("Enter an int: "; [1] = 1, 2, 3, 4, 5); // combine any features
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

```rust
use smart_read::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Car {
	pub name: String,
	pub color: String,
}

fn main() {
	let input = read!(= new_car("Red", "Toyota"), new_car("Silver", "Ram"));
	println!("You chose: {input}");
}

pub fn new_car(color: impl Into<String>, name: impl Into<String>) -> Car {
	Car {name: name.into(), color: color.into()}
}

impl std::fmt::Display for Car {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {}", self.color, self.name)
	}
}
```

<br>

### Add New Functionality

```rust
use smart_read::*;

struct PasswordInput {
	pub min_len: usize,
	pub min_digits: usize,
}

fn main() {
	let input = read!(PasswordInput {min_len: 10, min_digits: 1});
	println!("You entered: \"{input}\"");
}

impl TryRead for PasswordInput {
	type Output = String;
	fn try_read_line(&self, read_data: TryReadArgs<Self::Output>) -> smart_read::BoxResult<Self::Output> {
		assert!(read_data.default.is_none());
		let prompt = read_data.prompt.unwrap_or_else(
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

```
