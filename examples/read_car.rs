use smart_read::prelude::*;



#[derive(Clone, PartialEq)]
pub struct Car {
	pub name: String,
	pub color: String,
}

// choose from a list of cars
fn main() {
	let options = &[new_car("Red", "Toyota"), new_car("Silver", "Ram")];
	let input = read!(options).1;
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
