use smart_read::read;

fn main() {
	
	let input = read!("input is already given\r\n" >>); // read a line of existing text
	println!("You entered: \"{input}\"");
	
}
