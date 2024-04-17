use crate::*;
use std::collections::{LinkedList, VecDeque};





/// Internal utility function
pub fn read_input_option_enumerated<T: Display + Clone>(choices: &[T], prompt: Option<String>, default: Option<usize>) -> BoxResult<(usize, T)> {
	if choices.is_empty() {return Err(Box::new(ListConstraintError::EmptyList));}
	
	let prompt = prompt.unwrap_or(String::from("Enter one of the following:"));
	let choice_strings =
		choices.iter()
		.map(ToString::to_string)
		.collect::<Vec<_>>();
	
	let print_prompt = || {
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
	};
	
	if choices.len() == 1 {
		print_prompt();
		println!();
		println!("Automatically choosing {} since it is the only option", choices[0]);
		return Ok((0, choices[0].clone()));
	}
	
	loop {
		
		print_prompt();
		
		let output = read_stdin()?;
		if output.is_empty() && let Some(default) = default {
			return Ok((default, choices[default].clone()));
		}
		
		for (i, choice) in choice_strings.iter().enumerate() {
			if choice.eq_ignore_ascii_case(&output) {
				return Ok((i, choices[i].clone()));
			}
		}
		
		println!();
		println!("Invalid option");
	}
}

/// Internal utility function
pub fn read_input_option<T: Display + Clone>(choices: &[T], prompt: Option<String>, default: Option<usize>) -> BoxResult<T> {
	read_input_option_enumerated(choices, prompt, default).map(|(_index, output)| output)
}

/// Error type
#[derive(Debug)]
pub enum ListConstraintError {
	/// This exists because an empty list would be a softlock
	EmptyList,
}

impl Error for ListConstraintError {}

impl Display for ListConstraintError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::EmptyList => write!(f, "List Constraint is empty"),
		}
	}
}



/// Allows you to add more data to an option
/// 
/// Example:
/// 
/// ```
/// // example data
/// let mut colors = vec!("Red", "green", "Blue");
/// 
/// // prepare options, only capitalized colors can be removed
/// let choosable_colors =
/// 	colors.iter().enumerate()
/// 	.filter_map(|(i, color_name)| {
/// 		let first_char = color_name.chars().next()?;
/// 		if first_char.is_lowercase() {return None;}
/// 		Some(OptionWithData {name: color_name.to_string(), data: i})
/// 	})
/// 	.collect::<Vec<_>>();
/// 
/// // prompt
/// let OptionWithData {name: _, data: index_to_remove} = prompt!("Choose a color to remove: "; choosable_colors);
/// colors.remove(index_to_remove);
/// ```
#[derive(Clone, PartialEq)]
pub struct OptionWithData<T: Clone + PartialEq> {
	/// What's shown to the user
	pub display_name: String,
	/// What isn't shown to the user
	pub data: T,
}

impl<T: Clone + PartialEq> Display for OptionWithData<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", self.display_name)
	}
}





impl<T: Display + Clone + PartialEq> TryRead for &[T] {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_input_option(self, prompt, default)
	}
}

impl<T: Display + Clone + PartialEq, const LEN: usize> TryRead for &[T; LEN] {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		#[allow(clippy::explicit_auto_deref)] // false positive
		read_input_option(*self, prompt, default)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for Vec<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_input_option(self, prompt, default)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for VecDeque<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_input_option(&self.iter().cloned().collect::<Vec<_>>(), prompt, default)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for LinkedList<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_input_option(&self.iter().cloned().collect::<Vec<_>>(), prompt, default)
	}
}



/// Returns the index of the chosen item along with the item. &nbsp; <b> NOTE </b> : If you filter the inputs before feeding them into EnumerateInput, the indices returns won't match the indices of the initial input. In this case, you might want to use OptionWithData instead
pub struct EnumerateInput<T: TryRead> (pub T);

impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<&[T]> {
	type Output = (usize, T);
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default_index = if let Some((index, _item)) = &default {
			Some(*index)
		} else {
			None
		};
		read_input_option_enumerated(self.0, prompt, default_index)
	}
}

impl<T: Display + Clone + PartialEq, const LEN: usize> TryRead for EnumerateInput<&[T; LEN]> {
	type Output = (usize, T);
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default_index = if let Some((index, _item)) = &default {
			Some(*index)
		} else {
			None
		};
		read_input_option_enumerated(self.0, prompt, default_index)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<Vec<T>> {
	type Output = (usize, T);
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default_index = if let Some((index, _item)) = &default {
			Some(*index)
		} else {
			None
		};
		read_input_option_enumerated(&self.0, prompt, default_index)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<VecDeque<T>> {
	type Output = (usize, T);
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default_index = if let Some((index, _item)) = &default {
			Some(*index)
		} else {
			None
		};
		let slice = self.0.iter().cloned().collect::<Vec<_>>();
		read_input_option_enumerated(&slice, prompt, default_index)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for EnumerateInput<LinkedList<T>> {
	type Output = (usize, T);
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default_index = if let Some((index, _item)) = &default {
			Some(*index)
		} else {
			None
		};
		let slice = self.0.iter().cloned().collect::<Vec<_>>();
		read_input_option_enumerated(&slice, prompt, default_index)
	}
}
