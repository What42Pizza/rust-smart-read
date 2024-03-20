use crate::*;
use std::{collections::{LinkedList, VecDeque}, fmt::Display};





/// Internal utility function
pub fn read_input_option<T: Display + Clone>(choices: &[T], default: Option<usize>, mut read_args: TryReadArgs<T>) -> BoxResult<T> {
	let prompt = read_args.prompt.unwrap_or(String::from("Enter one of the following:"));
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
		
		let output = read_string(&mut read_args.input)?;
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





impl<'a, T: Display + Clone + PartialEq> TryRead for &[T] {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == read_args.default.as_ref())
			.map(|v| v.0);
		read_input_option(self, default, read_args)
	}
}

impl<'a, T: Display + Clone + PartialEq, const LEN: usize> TryRead for &[T; LEN] {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == read_args.default.as_ref())
			.map(|v| v.0);
		read_input_option(*self, default, read_args)
	}
}

impl<'a, T: Display + Clone + PartialEq> TryRead for Vec<T> {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == read_args.default.as_ref())
			.map(|v| v.0);
		read_input_option(self, default, read_args)
	}
}

impl<'a, T: Display + Clone + PartialEq> TryRead for VecDeque<T> {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == read_args.default.as_ref())
			.map(|v| v.0);
		read_input_option(&self.iter().cloned().collect::<Vec<_>>(), default, read_args)
	}
}

impl<'a, T: Display + Clone + PartialEq> TryRead for LinkedList<T> {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == read_args.default.as_ref())
			.map(|v| v.0);
		read_input_option(&self.iter().cloned().collect::<Vec<_>>(), default, read_args)
	}
}
