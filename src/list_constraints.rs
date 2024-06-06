use crate::*;
use std::collections::{LinkedList, VecDeque};



impl<'a, Data: 'a> TryRead<'a> for Vec<InputOption<Data>> {
	type Output = (usize, &'a InputOption<Data>);
	type Default = usize;
	fn try_read_line(&'a self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		if self.is_empty() {return Err(Box::new(ListConstraintError::EmptyList));}
		
		let prompt = prompt.unwrap_or(String::from("Enter one of the following:"));
		let option_strings =
			self.iter().enumerate()
			.map(|(i, option)| {
				option.get_display_string(default.map(|default| i == default))
			})
			.collect::<Vec<_>>();
		
		let print_prompt = || {
			println!("{prompt}");
			for option in option_strings.iter() {
				println!("{option}");
			}
			println!();
		};
		
		if self.len() == 1 {
			print_prompt();
			println!();
			println!("Automatically choosing the only option because it is the only option");
			return Ok((0, &self[0]));
		}
		
		print_prompt();
		let mut input = read_stdin()?;
		
		loop {
			if input.is_empty() && let Some(default) = default {
				return Ok((default, &self[default]));
			}
			
			// find exact match
			for (i, option) in option_strings.iter().enumerate() {
				if option.eq_ignore_ascii_case(&input) {
					return Ok((i, &self[i]));
				}
			}
			
			println!();
			println!("Invalid option.");
			
			// try fuzzy match
			let possible_option_index = custom_fuzzy_search(&input, &option_strings);
			print!("Did you mean \"{}\"? (enter nothing to confirm, or re-enter input) ", option_strings[possible_option_index]);
			let new_input = read_stdin()?;
			if new_input.is_empty() {
				return Ok((possible_option_index, &self[possible_option_index]));
			}
			input = new_input;
			
		}
	}
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
pub struct InputOption<Data> {
	/// What's shown to the user (minus the choose_name)
	pub display_name: String,
	/// What the user needs to enter to choose this option
	pub choose_name: Option<String>,
	/// What isn't shown to the user
	pub data: Data,
}

impl<Data> InputOption<Data> {
	/// Internal function
	pub fn get_display_string(&self, is_default: Option<bool>) -> String {
		match (self.choose_name.as_ref(), is_default) {
			(Some(choose_name), Some(true )) => format!("[{choose_name}]: {}", self.display_name),
			(Some(choose_name), Some(false)) => format!(" {choose_name}:  {}", self.display_name),
			(None                      , Some(true )) => format!("[{}]", self.display_name),
			(None                      , Some(false)) => format!(" {} ", self.display_name),
			(Some(choose_name), None       ) => format!("{choose_name}: {}", self.display_name),
			(None                      , None       ) => format!("{}", self.display_name),
		}
	}
}





impl<'a, T: Display + 'a> TryRead<'a> for &'a [T] {
	type Output = (usize, &'a T);
	type Default = usize;
	fn try_read_line(&'a self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.into_iter()
			.map(|option| {
				InputOption {
					display_name: option.to_string(),
					choose_name: None,
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = options.try_read_line(prompt, default)?.0;
		Ok((chosen_index, &self[chosen_index]))
	}
}

impl<'a, T: Display + 'a, const LEN: usize> TryRead<'a> for &[T; LEN] {
	type Output = (usize, &'a T);
	type Default = usize;
	fn try_read_line(&'a self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.into_iter()
			.map(|option| {
				InputOption {
					display_name: option.to_string(),
					choose_name: None,
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = options.try_read_line(prompt, default)?.0;
		Ok((chosen_index, &self[chosen_index]))
	}
}

impl<'a, T: Display + 'a> TryRead<'a> for Vec<T> {
	type Output = (usize, &'a T);
	type Default = usize;
	fn try_read_line(&'a self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.into_iter()
			.map(|option| {
				InputOption {
					display_name: option.to_string(),
					choose_name: None,
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = options.try_read_line(prompt, default)?.0;
		Ok((chosen_index, &self[chosen_index]))
	}
}

impl<'a, T: Display + 'a> TryRead<'a> for VecDeque<T> {
	type Output = (usize, &'a T);
	type Default = usize;
	fn try_read_line(&'a self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.into_iter()
			.map(|option| {
				InputOption {
					display_name: option.to_string(),
					choose_name: None,
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = options.try_read_line(prompt, default)?.0;
		Ok((chosen_index, &self[chosen_index]))
	}
}

impl<'a, T: Display + 'a> TryRead<'a> for LinkedList<T> {
	type Output = (usize, &'a T);
	type Default = usize;
	fn try_read_line(&'a self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.into_iter()
			.map(|option| {
				InputOption {
					display_name: option.to_string(),
					choose_name: None,
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = options.try_read_line(prompt, default)?.0;
		Ok((chosen_index, self.iter().nth(chosen_index).unwrap()))
	}
}
