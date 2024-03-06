use smart_read::read;



fn main() {
	let input = read!(= Car::new("Red", "Toyota"), Car::new("Silver", "Ram"));
	println!("You chose: {input}");
}



#[derive(Clone, PartialEq)]
pub struct Car {
	pub name: String,
	pub color: String,
}

impl Car {
	pub fn new(color: impl Into<String>, name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			color: color.into(),
		}
	}
}

impl std::fmt::Display for Car {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{} {}", self.color, self.name)
	}
}
