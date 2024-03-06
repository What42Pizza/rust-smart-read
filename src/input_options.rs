use crate::{ReadLine, read_string, BoxResult};
use std::collections::{HashMap, LinkedList, VecDeque};





pub fn read_input_option<T: ToString + Clone>(choices: &[T]) -> BoxResult<T> {
	let choice_strings = choices.iter().map(ToString::to_string).collect::<Vec<_>>();
	loop {
		println!("Enter one of the following:");
		for choice in &choice_strings {
			println!("'{choice}'");
		}
		let output = read_string()?;
		for (i, choice) in choice_strings.iter().enumerate() {
			if choice.eq_ignore_ascii_case(&output) {
				return Ok(choices[i].clone());
			}
		}
		println!("Invalid option.");
	}
}






impl<T> ReadLine for &[T]
	where T: ToString + Clone
{
	type Output = T;
	fn try_read_line(&self) -> BoxResult<Self::Output> {
		read_input_option(self)
	}
}

impl<T, const LEN: usize> ReadLine for &[T; LEN]
	where T: ToString + Clone
{
	type Output = T;
	fn try_read_line(&self) -> BoxResult<Self::Output> {
		read_input_option(*self)
	}
}

impl<T> ReadLine for Vec<T>
	where T: ToString + Clone
{
	type Output = T;
	fn try_read_line(&self) -> BoxResult<Self::Output> {
		read_input_option(self)
	}
}

impl<T> ReadLine for VecDeque<T>
	where T: ToString + Clone
{
	type Output = T;
	fn try_read_line(&self) -> BoxResult<Self::Output> {
		read_input_option(&self.iter().cloned().collect::<Vec<_>>())
	}
}

impl<K, T> ReadLine for HashMap<K, T>
	where T: ToString + Clone
{
	type Output = T;
	fn try_read_line(&self) -> BoxResult<Self::Output> {
		read_input_option(&self.values().cloned().collect::<Vec<_>>())
	}
}

impl<T> ReadLine for LinkedList<T>
	where T: ToString + Clone
{
	type Output = T;
	fn try_read_line(&self) -> BoxResult<Self::Output> {
		read_input_option(&self.iter().cloned().collect::<Vec<_>>())
	}
}
