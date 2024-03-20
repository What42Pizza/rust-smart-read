use smart_read::prelude::*;

fn main() {
	
	println!("\n==== `read!()` ====");
	let input = read!(); // read a line of text
	println!("You entered: \"{input}\"");
	
	println!("\n==== `prompt!(\"Enter a string: \")` ====");
	let input = prompt!("Enter a string: "); // prompt a line of text
	println!("You entered: \"{input}\"");
	
	println!("\n==== `read!([\"two\"])` ====");
	let input = read!(["two"]); // read a line of text with a default 
	println!("You entered: \"{input}\"");
	
	println!("\n==== `read!(= \"red\", \"green\", \"blue\")` ====");
	let input = read!(= "red", "green", "blue"); // receive specific inputs
	println!("You entered: \"{input}\"");
	println!("\n==== `prompt!(\"Enter a color: \"; = \"red\", \"green\", \"blue\")` ====");
	let input = prompt!("Enter a color: "; = "red", "green", "blue");
	println!("You entered: \"{input}\"");
	println!("\n==== `prompt!(\"Enter a color: \"; &[\"red\", \"green\", \"blue\"])` ====");
	let input = prompt!("Enter a color: "; &["red", "green", "blue"]); // same as line 2 above
	println!("You entered: \"{input}\"");
	
	println!("\n==== `prompt!(\"Which color do you want to remove?\"; EnumerateInput (&*colors));` ====");
	let mut colors = vec!("red", "green", "blue");
	let (index, item) = prompt!("Which color do you want to remove?"; EnumerateInput (&*colors)); // get index of chosen option
	colors.remove(index);
	println!("You entered: ({index}, \"{item}\")");
	
	println!("\n==== `read!(0. ..= 100.)` ====");
	let input = read!(0. ..= 100.); // take a number within a range
	println!("You entered: \"{input}\"");
	
	println!("\n==== `prompt!(\"Confirm input: \"; [true] YesNoInput)` ====");
	let input = prompt!("Confirm input: "; [true] YesNoInput); // read a bool
	println!("You entered: \"{input}\"");
	
	println!("\n==== `prompt!(\"This input will come from a string\"; \"input is already given\\r\\n\" >>)` ====");
	let input = prompt!("This input will come from a string\n"; "input is already given\r\n" >>); // take input from any Result<u8> iterator
	println!("You entered: \"{input}\"");
	
	println!("\n==== `prompt!(\"Enter an int: \"; [1] = 1, 2, 3, 4, 5)` ====");
	let input = prompt!("Enter an int: "; [1] = 1, 2, 3, 4, 5); // combine anything
	println!("You entered: \"{input}\"");
	
	println!();
	prompt!("read_lines finished, press enter to exit.");
	
}
