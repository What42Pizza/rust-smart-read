use crate::*;
use std::{collections::{LinkedList, VecDeque}, fmt::Display};





/// Internal utility function
pub fn read_input_option_enumerated<T: Display + Clone>(choices: &[T], default: Option<usize>, mut read_args: TryReadArgs<T>) -> BoxResult<(usize, T)> {
	if choices.len() == 0 {panic!("Cannot read input because there are no choices. (empty list constraint)")}
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
			return Ok((default, choices[default].clone()));
		}
		
		for (i, choice) in choice_strings.iter().enumerate() {
			if choice.eq_ignore_ascii_case(&output) {
				return Ok((i, choices[i].clone()));
			}
		}
		
		println!("Invalid option.");
	}
}

/// Internal utility function
pub fn read_input_option<T: Display + Clone>(choices: &[T], default: Option<usize>, read_args: TryReadArgs<T>) -> BoxResult<T> {
	read_input_option_enumerated(choices, default, read_args).map(|(_index, output)| output)
}





impl<T: Display + Clone + PartialEq> TryRead for &[T] {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == read_args.default.as_ref())
			.map(|v| v.0);
		read_input_option(self, default, read_args)
	}
}

impl<T: Display + Clone + PartialEq, const LEN: usize> TryRead for &[T; LEN] {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == read_args.default.as_ref())
			.map(|v| v.0);
		#[allow(clippy::explicit_auto_deref)] // false positive
		read_input_option(*self, default, read_args)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for Vec<T> {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == read_args.default.as_ref())
			.map(|v| v.0);
		read_input_option(self, default, read_args)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for VecDeque<T> {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == read_args.default.as_ref())
			.map(|v| v.0);
		read_input_option(&self.iter().cloned().collect::<Vec<_>>(), default, read_args)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for LinkedList<T> {
	type Output = T;
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == read_args.default.as_ref())
			.map(|v| v.0);
		read_input_option(&self.iter().cloned().collect::<Vec<_>>(), default, read_args)
	}
}



/// Returns the index of the chosen item along with the item. Remember to NOT use this if, for example, you filter the choices before feeding them to smart-read
pub struct EnumerateInput<T: TryRead> (pub T);

impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<&[T]> {
	type Output = (usize, T);
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default_index = if let Some((index, _item)) = &read_args.default {
			Some(*index)
		} else {
			None
		};
		let read_args = TryReadArgs {
			input: read_args.input,
			prompt: read_args.prompt,
			default: read_args.default.map(|(_index, item)| item),
		};
		read_input_option_enumerated(self.0, default_index, read_args)
	}
}

impl<T: Display + Clone + PartialEq, const LEN: usize> TryRead for EnumerateInput<&[T; LEN]> {
	type Output = (usize, T);
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default_index = if let Some((index, _item)) = &read_args.default {
			Some(*index)
		} else {
			None
		};
		let read_args = TryReadArgs {
			input: read_args.input,
			prompt: read_args.prompt,
			default: read_args.default.map(|(_index, item)| item),
		};
		read_input_option_enumerated(self.0, default_index, read_args)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<Vec<T>> {
	type Output = (usize, T);
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default_index = if let Some((index, _item)) = &read_args.default {
			Some(*index)
		} else {
			None
		};
		let read_args = TryReadArgs {
			input: read_args.input,
			prompt: read_args.prompt,
			default: read_args.default.map(|(_index, item)| item),
		};
		read_input_option_enumerated(&self.0, default_index, read_args)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<VecDeque<T>> {
	type Output = (usize, T);
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default_index = if let Some((index, _item)) = &read_args.default {
			Some(*index)
		} else {
			None
		};
		let read_args = TryReadArgs {
			input: read_args.input,
			prompt: read_args.prompt,
			default: read_args.default.map(|(_index, item)| item),
		};
		let slice = self.0.iter().cloned().collect::<Vec<_>>();
		read_input_option_enumerated(&slice, default_index, read_args)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<LinkedList<T>> {
	type Output = (usize, T);
	fn try_read_line(&self, read_args: TryReadArgs<Self::Output>) -> BoxResult<Self::Output> {
		let default_index = if let Some((index, _item)) = &read_args.default {
			Some(*index)
		} else {
			None
		};
		let read_args = TryReadArgs {
			input: read_args.input,
			prompt: read_args.prompt,
			default: read_args.default.map(|(_index, item)| item),
		};
		let slice = self.0.iter().cloned().collect::<Vec<_>>();
		read_input_option_enumerated(&slice, default_index, read_args)
	}
}
