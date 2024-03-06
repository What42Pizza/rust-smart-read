use crate::{read_string, BoxResult, ReadLine};
use std::{collections::{LinkedList, VecDeque}, fmt::Display};





/// Internal utility function
pub fn read_input_option<T: Display + Clone>(prompt: Option<&str>, default: Option<usize>, choices: &[T]) -> BoxResult<T> {
	let prompt = prompt.unwrap_or("Enter one of the following:");
	let choice_strings =
		choices.iter()
		.map(ToString::to_string)
		.collect::<Vec<_>>();
	loop {
		
		println!("{prompt}");
		for (i, choice) in choice_strings.iter().enumerate() {
			if let Some(default) = default {
				if i == default {
					println!("[{choice}]");
				} else {
					println!(" {choice}");
				}
			} else {
				println!("{choice}");
			}
		}
		
		let output = read_string()?;
		if output.is_empty() && let Some(default) = default {
			return Ok(choices[default].clone());
		}
		
		for (i, choice) in choice_strings.iter().enumerate() {
			if choice.eq_ignore_ascii_case(&output) {
				return Ok(choices[i].clone());
			}
		}
		
		println!("Invalid option.");
	}
}





impl<T: Display + Clone + PartialEq> ReadLine for &[T] {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<T>) -> BoxResult<Self::Output> {
		let prompt = prompt.as_ref().map(String::as_str);
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_input_option(prompt, default, self)
	}
}

impl<T: Display + Clone + PartialEq, const LEN: usize> ReadLine for &[T; LEN] {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<T>) -> BoxResult<Self::Output> {
		let prompt = prompt.as_ref().map(String::as_str);
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_input_option(prompt, default, *self)
	}
}

impl<T: Display + Clone + PartialEq> ReadLine for Vec<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<T>) -> BoxResult<Self::Output> {
		let prompt = prompt.as_ref().map(String::as_str);
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_input_option(prompt, default, &self)
	}
}

impl<T: Display + Clone + PartialEq> ReadLine for VecDeque<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<T>) -> BoxResult<Self::Output> {
		let prompt = prompt.as_ref().map(String::as_str);
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_input_option(prompt, default, &self.iter().cloned().collect::<Vec<_>>())
	}
}

impl<T: Display + Clone + PartialEq> ReadLine for LinkedList<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<T>) -> BoxResult<Self::Output> {
		let prompt = prompt.as_ref().map(String::as_str);
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_input_option(prompt, default, &self.iter().cloned().collect::<Vec<_>>())
	}
}
