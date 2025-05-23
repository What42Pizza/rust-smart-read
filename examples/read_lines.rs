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
	let (index, input) = read!(["red", "green", "blue"]); // choose from a list of options
	println!("You entered: index {index}, \"{input}\"");
	println!("\n==== `read!(= \"red\", \"green\", \"blue\")` ====");
	let (index, input) = read!(= "red", "green", "blue"); // some inputs types have special syntax
	println!("You entered: index {index}, \"{input}\"");
	
	println!("\n==== `read!([InputOption::new(...), ...])` ====");
	let (index, input) = read!([
		InputOption::new("1", &["red"  , "r", "choose first" ], ()), // displayed as "1: red", can be chosen with "1", "red", "r", or "choose first"
		InputOption::new("2", &["green", "g", "choose second"], ()),
		InputOption::new("3", &["blue" , "b", "choose third" ], ()),
	]);
	println!("You entered: index {index}, \"{}\"", input.names[0]);
	
	println!("\n==== `read!(= ...)` ====");
	let (index, input) = read!(=
		"1"; "red"  ; ["r", "choose first" ]; (), // displayed as "1: red", can be chosen with "1", "red", "r", or "choose first"
		"2"; "green"; ["g", "choose second"]; (),
		"3"; "blue" ; ["b", "choose third" ]; (),
	);
	println!("You entered: index {index}, \"{}\"", input.names[0]);
	
	println!("\n==== `prompt!(\"Enter an even int: \"; TransformValidate(...));` ====");
	// one-time custom logic
	let input = prompt!("Enter an even int: "; TransformValidate (|x| {
		// validate input as an integer
		let Ok(x) = x.parse::<isize>() else {return Err(String::from("Could not parse input"));};
		// validate input as even
		if x % 2 != 0 {return Err(String::from("Input is not even."));}
		Ok(x)
	}));
	println!("You entered: \"{input}\"");
	
	
	println!("\n==== `prompt!(\"Enter an int: \"; [2usize] = \"a\", \"b\", \"c\")` ====");
	let (index, input) = prompt!("Enter an int: "; [2usize] = "a", "b", "c"); // combine any features
	println!("You entered: index {index}, \"{input}\"");
	
	
	println!();
	println!("read_lines example finished");
	wait_for_enter();
	
}
