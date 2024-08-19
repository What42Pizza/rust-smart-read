use smart_read::prelude::*;

fn main() {
	
	println!("\n==== custom fuzzy-search testing ====");
	let input = read!(["Lanercoast", "Windrip", "Redwick Bush", "Brickelwhyte", "Sirencester", "Conriston", "Inverness", "Norwich", "Elinmylly", "Murlayfield"]).1;
	println!("You entered: \"{input}\"");
	
	
	
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
	
	
	
	println!("\n==== `read!([\"red\", \"green\", \"blue\"])` ====");
	let input = read!(["red", "green", "blue"]).1; // choose from a list of options
	println!("You entered: \"{input}\"");
	println!("\n==== `read!(= \"red\", \"green\", \"blue\")` ====");
	let input = read!(= "red", "green", "blue").1; // some inputs have special syntax
	println!("You entered: \"{input}\"");
	
	println!("\n==== `read!([InputOption::new(...), ...])` ====");
	let options = [
		InputOption::new("red", vec!("1", "r"), ()),
		InputOption::new("green", vec!("2", "g"), ()),
		InputOption::new("blue", vec!("3", "b"), ()),
	];
	let input = read!(options);
	println!("You entered: index {}, \"{}\"", input.0, input.1.display_name);
	
	
	
	println!("\n==== `prompt!(\"Enter an even int: \"; TransformValidate(...));` ====");
	// one-time custom logic
	let input = prompt!("Enter an even int: "; TransformValidate (|x| {
		let Ok(x) = x.parse::<isize>() else {return Err(String::from("Could not parse input"));};
		if x % 2 != 0 {return Err(String::from("Input is not even."));}
		Ok(x)
	}));
	println!("You entered: \"{input}\"");
	
	
	
	println!("\n==== `prompt!(\"Enter an int: \"; [1usize] = 1, 2, 3, 4, 5)` ====");
	let input = prompt!("Enter an int: "; [1usize] = 1, 2, 3, 4, 5).1; // combine any features
	println!("You entered: \"{input}\"");
	
	
	
	println!();
	prompt!("read_lines finished, press enter to exit.");
	
}
