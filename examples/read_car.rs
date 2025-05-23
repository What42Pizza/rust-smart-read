use smart_read::prelude::*;



// choose from a list of cars
fn main() {
	let (index, input) = read!(= new_car("Red", "Toyota"), new_car("Silver", "Ram"));
	println!("You chose: {input} (index {index})");
}



#[derive(Clone, PartialEq)]
pub struct Car {
	pub name: String,
	pub color: String,
}

impl std::fmt::Display for Car {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {}", self.color, self.name)
	}
}

pub fn new_car(color: impl Into<String>, name: impl Into<String>) -> Car {
	Car {name: name.into(), color: color.into()}
}
