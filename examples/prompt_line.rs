use smart_read::prompt;

fn main() {
	let input = prompt!("Choose your number: "; = 10, 20, 30);
	println!("You chose: {input}");
}
