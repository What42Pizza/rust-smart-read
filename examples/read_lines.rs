use smart_read::prelude::*;

fn main() {
	
	
	
	println!("\n==== `read!()` ====");
	let input = read!(); // read a line of text
	println!("You entered: \"{input}\"");
	
	println!("\n==== `prompt!(\"Enter a string: \")` ====");
	let input = prompt!("Enter a string: "); // prompt a line of text
	println!("You entered: \"{input}\"");
	
	println!("\n==== `read!(UsizeInput)` ====");
	let input = read!(UsizeInput); // read specific types
	println!("You entered: \"{input}\"");
	println!("\n==== `read!(BoolInput)` ====");
	let input = read!(BoolInput);
	println!("You entered: \"{input}\"");
	println!("\n==== `read!(NonWhitespaceInput)` ====");
	let input = read!(NonWhitespaceInput);
	println!("You entered: \"{input}\"");
	println!("\n==== `read!(I32Input)` ====");
	let input = read!(I32Input);
	println!("You entered: \"{input}\"");
	
	println!("\n==== `read!(0. ..= 100.)` ====");
	let input = read!(0. ..= 100.); // read a number within a range
	println!("You entered: \"{input}\"");
	
	
	
	println!("\n==== `prompt!(\"Confirm input: \"; YesNoInput)` ====");
	let input = prompt!("Confirm input: "; YesNoInput); // read a bool
	println!("You entered: \"{input}\"");
	
	println!("\n==== `prompt!(\"Confirm input: \"; [true] YesNoInput)` ====");
	let input = prompt!("Confirm input: "; [true] YesNoInput); // set a default value
	println!("You entered: \"{input}\"");
	
	
	
	println!("\n==== `read!(&[\"red\", \"green\", \"blue\"])` ====");
	let input = read!(&["red", "green", "blue"]); // choose from a list of options
	println!("You entered: \"{input}\"");
	println!("\n==== `read!(= \"red\", \"green\", \"blue\")` ====");
	let input = read!(= "red", "green", "blue"); // some inputs have special syntax
	println!("You entered: \"{input}\"");
	
	
	
	println!("\n==== `prompt!(\"Enter an even int: \"; TransformValidate(...));` ====");
	// one-time custom logic
	let input = prompt!("Enter an even int: "; TransformValidate (|x| {
		let Ok(x) = x.parse::<isize>() else {return Err(String::from("Could not parse input"));};
		if x % 2 != 0 {return Err(String::from("Input is not even."));}
		Ok(x)
	}));
	println!("You entered: \"{input}\"");
	
	
	
	println!("\n==== `prompt!(\"Enter an int: \"; [1] = 1, 2, 3, 4, 5)` ====");
	let input = prompt!("Enter an int: "; [1] = 1, 2, 3, 4, 5); // combine any features
	println!("You entered: \"{input}\"");
	
	
	
	println!();
	prompt!("read_lines finished, press enter to exit.");
	
}
