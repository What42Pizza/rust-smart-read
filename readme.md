<center>

# Smart Read

[![Crates.io](https://img.shields.io/crates/v/smart-read.svg)](https://crates.io/crates/smart-read)
[![Crates.io](https://img.shields.io/crates/d/smart-read.svg)](https://crates.io/crates/smart-read)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/What42Pizza/rust-smart-read/blob/main/LICENSE)

#### Complex but easy ways to read user input

Anything that implements [TryRead](https://docs.rs/smart-read/latest/smart_read/trait.TryRead.html) can be used with the [read](https://docs.rs/smart-read/latest/smart_read/macro.read.html) and [prompt](https://docs.rs/smart-read/latest/smart_read/macro.prompt.html) macros, and features are added by creating new implementations of this trait. For a list of default implementations, just go to the main [docs](https://docs.rs/smart-read/latest/smart_read/) page.

</center>

<br>

## Qualities

- **Extremely Customizable**, basically anything can be implemented
- **Extremely Simple**, for both using and extending
- **Extremely Ergonomic**, everything is as effortless as possible

<br>

### Basic Usage

```
use smart_read::prelude::*;

// read a line of text
let input = read!();

// prompt a line of text
let input = prompt!("Enter a string: ");

// read specific types
let input = read!(UsizeInput);
let input = read!(BoolInput);
let input = read!(NonWhitespaceInput);
let input = read!(I32Input);

// read a number within a range
let input = read!(0. ..= 100.);


// read a bool
let input = prompt!("Confirm input: "; YesNoInput);

// set a default value
let input = prompt!("Confirm input: "; [true] YesNoInput);


// choose from a list of options
let (index, input) = read!(["red", "green", "blue"]);
// some input types have special syntax
let (index, input) = read!(= "red", "green", "blue");

// give options bulletins, alternate matching strings, and extra data
let (index, input) = read!([
	InputOption::new("1", "red", vec!("r", "the first color"), ()), // displayed as "1: red", and so on
	InputOption::new("2", "green", vec!("g", "the second color"), ()),
	InputOption::new("3", "blue", vec!("b", "the third color"), ()),
]);

// same as above, but using special syntax
let (index, input) = read!(=
	["1", "red", "r", "the first color"], (),
	["2", "green", "g", "the second color"], (),
	["3", "blue", "b", "the third color"], (),
);


// one-time custom logic
let input = prompt!("Enter an even int: "; TransformValidate (|x: String| -> Result<isize, String> { // explicit types here are optional, only added for demonstration
	// validate input as an integer
	let Ok(x) = x.parse::<isize>() else {return Err(String::from("Could not parse input"));};
	// validate input as even
	if x % 2 != 0 {return Err(String::from("Input is not even."));}
	Ok(x)
}));


// combine any features
let (index, input) = prompt!("Enter an int: "; [1usize] = 1, 2, 3, 4, 5); // uses prompt message, default value, and special list_constraint syntax
```

<br>

### Example stdout (from the read_lines example)

```
==== `read!(0. ..= 100.)` ====
Enter a number within the range [0.0, 100.0]: 100.0001

Invalid input, not within bounds
Enter a number within the range [0.0, 100.0]: aa 

Could not parse input (error: invalid float literal)
Enter a number within the range [0.0, 100.0]: 1.
You entered: "1"
```

<br>

### Extend Existing Functionality

```
use smart_read::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Car {
	pub name: String,
	pub color: String,
}

// choose from a list of cars
fn main() {
	let (index, input) = read!(= new_car("Red", "Toyota"), new_car("Silver", "Ram"));
	println!("You chose: {input} (index {index})");
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

```
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

impl<'a> TryRead<'a> for PasswordInput {
	type Output = String;
	type Default = (); // ensure no default can be given
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Default>) -> smart_read::BoxResult<Self::Output> {
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
```
