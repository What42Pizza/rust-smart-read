use smart_read::read;



#[derive(Clone, PartialEq)]
pub struct Car {
	pub name: String,
	pub color: String,
}

fn main() {
	let input = read!(= new_car("Red", "Toyota"), new_car("Silver", "Ram"));
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
