## Smart Read

#### Complex but easy ways to read user input

<br>

If you just want to read a line of text: `read!()`

Or if you want to prompt the user for input: `prompt!("Enter a string: ")`

Or if you want to receive certain inputs: `read!(= "red", "green", "blue")` or `prompt!("Enter a color: "; = "red", "green", "blue")`

Or even take a number within a range: `read!(0. ..= 100.)`

And of course, you can take a Result instead: `try_read!(&["option 1", "option 2"])`

And all with clean(?) outputs:
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

You can even extend functionality with little effort:

```
fn main() {
	let input = read!(= Car::new("Red", "Toyota"), Car::new("Silver", "Ram"));
	println!("You chose: {input}");
}

#[derive(Clone)]
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

Or, you can even add brand new functionality:

```
use smart_read::{read, read_string, ReadLine};

fn main() {
	let input = read!(CustomInput);
	println!("You entered: \"{input}\"");
}

struct CustomInput;

impl ReadLine for CustomInput {
	type Output = &'static std::ffi::FromBytesUntilNulError;
	fn try_read_line(&self) -> smart_read::BoxResult<Self::Output> {
		loop {
			print!("Enter a string: ");
			let _ = read_string()?;
			println!("Input not accepted");
		}
	}
}
```
