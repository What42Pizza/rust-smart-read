use crate::*;
use std::collections::{LinkedList, VecDeque};





/// Internal utility function
pub fn read_list_input_enumerated<T>(choices: &[InputChoice<T>], prompt: Option<String>, default: Option<usize>) -> BoxResult<(usize, T)> {
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
		println!();
	};
	
	if choices.len() == 1 {
		print_prompt();
		println!();
		println!("Automatically choosing {} since it is the only option", choices[0]);
		return Ok((0, choices[0].clone()));
	}
	
	print_prompt();
	let mut input = read_stdin()?;
	
	loop {
		if input.is_empty() && let Some(default) = default {
			return Ok((default, choices[default].clone()));
		}
		
		// find exact match
		for (i, choice) in choice_strings.iter().enumerate() {
			if choice.eq_ignore_ascii_case(&input) {
				return Ok((i, choices[i].clone()));
			}
		}
		
		println!();
		println!("Invalid option.");
		
		// try fuzzy match
		let possible_choice_index = custom_fuzzy_search(&input, &choice_strings);
		print!("Did you mean \"{}\"? (enter nothing to confirm, or re-enter input) ", choice_strings[possible_choice_index]);
		let new_input = read_stdin()?;
		if new_input.is_empty() {
			return Ok((possible_choice_index, choices[possible_choice_index].clone()));
		}
		input = new_input;
		
	}
}

/// Internal utility function
pub fn read_list_input<T: Display + Clone>(choices: &[T], prompt: Option<String>, default: Option<usize>) -> BoxResult<T> {
	read_list_input_enumerated(choices, prompt, default).map(|(_index, output)| output)
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



/// Custom implementation of fuzzy search, returns the index of the closest match
pub fn custom_fuzzy_search(pattern: &str, items: &[String]) -> usize {
	let (mut best_score, mut best_index) = (custom_fuzzy_match(pattern, &items[0]), 0);
	for (i, item) in items.iter().enumerate().skip(1) {
		let score = custom_fuzzy_match(pattern, item);
		if score > best_score {
			best_score = score;
			best_index = i;
		}
	}
	best_index
}

/// Custom implementation of fuzzy match. Not efficient at all, but gives good results
pub fn custom_fuzzy_match(pattern: &str, item: &str) -> f32 {
	let mut best_score = 0.0f32;
	let offset_start = pattern.len() as isize * -1 + 1;
	let offset_end = item.len() as isize - 1;
	for offset in offset_start..=offset_end {
		let item_slice = &item[offset.max(0) as usize .. (offset + pattern.len() as isize).min(item.len() as isize) as usize];
		let pattern_slice = &pattern[(offset * -1).max(0) as usize .. (item.len() as isize - offset).min(pattern.len() as isize) as usize];
		let mut slice_score = 0.0f32;
		for (item_char, pattern_char) in item_slice.chars().zip(pattern_slice.chars()) {
			if item_char.eq_ignore_ascii_case(&pattern_char) {
				slice_score += 3.;
			} else {
				slice_score -= 1.;
			}
		}
		slice_score *= 1. - offset as f32 / item.len() as f32 * 0.5; // give higher value to earlier matches, best weight is at offset = 0
		best_score = best_score.max(slice_score);
	}
	best_score
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
pub struct InputChoice<T> {
	/// What's shown to the user (minus the choose_name)
	pub display_name: String,
	/// What the user needs to enter to choose this option
	pub choose_name: Option<String>,
	/// What isn't shown to the user
	pub data: T,
}

impl<T> InputChoice<T> {
	pub fn get_display_string(&self, is_default: bool) -> String {
		if let Some(choose_name) = &self.choose_name {
			
		} else {
			
		}
	}
}

impl<T> Display for InputChoice<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if let Some(choose_name) = &self.choose_name {
			write!(f, "{}: {}", choose_name, self.display_name)
		} else {
			write!(f, "{}", self.display_name)
		}
	}
}





impl<T: Display + Clone + PartialEq> TryRead for &[T] {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_list_input(self, prompt, default)
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
		read_list_input(*self, prompt, default)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for Vec<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_list_input(self, prompt, default)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for VecDeque<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_list_input(&self.iter().cloned().collect::<Vec<_>>(), prompt, default)
	}
}

impl<T: Display + Clone + PartialEq> TryRead for LinkedList<T> {
	type Output = T;
	fn try_read_line(&self, prompt: Option<String>, default: Option<Self::Output>) -> BoxResult<Self::Output> {
		let default =
			self.iter().enumerate()
			.find(|v| Some(v.1) == default.as_ref())
			.map(|v| v.0);
		read_list_input(&self.iter().cloned().collect::<Vec<_>>(), prompt, default)
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
		read_list_input_enumerated(self.0, prompt, default_index)
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
		read_list_input_enumerated(self.0, prompt, default_index)
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
		read_list_input_enumerated(&self.0, prompt, default_index)
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
		read_list_input_enumerated(&slice, prompt, default_index)
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
		read_list_input_enumerated(&slice, prompt, default_index)
	}
}
