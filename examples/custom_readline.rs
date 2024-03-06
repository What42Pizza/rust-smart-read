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
