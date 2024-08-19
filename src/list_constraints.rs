use crate::*;
use std::collections::{LinkedList, VecDeque};



fn read_list<'a, Data: 'a>(input_options: &'a [InputOption<Data>], prompt: Option<String>, default: Option<usize>) -> BoxResult<usize> {
	if input_options.is_empty() {return Err(Box::new(ListConstraintError::EmptyList));}
	
	// get prompt data
	let prompt = prompt.unwrap_or(String::from("Enter one of the following:"));
	let display_strings =
		input_options.iter().enumerate()
		.map(|(i, option)| {
			option.get_display_string(default.map(|default| i == default))
		})
		.collect::<Vec<_>>();
	let (mut all_choose_strings, mut choose_name_mappings) = (vec!(), vec!());
	for (i, option) in input_options.iter().enumerate() {
		if option.choose_names.is_empty() {
			all_choose_strings.push(&option.display_name);
			choose_name_mappings.push(i);
		} else {
			for choose_name in &option.choose_names {
				all_choose_strings.push(choose_name);
				choose_name_mappings.push(i);
			}
		}
	}
	
	// misc work
	let print_prompt = || {
		println!("{prompt}");
		for option in display_strings.iter() {
			println!("{option}");
		}
		println!();
	};
	
	if input_options.len() == 1 {
		print_prompt();
		println!();
		println!("Automatically choosing the first option because it is the only option");
		return Ok(0);
	}
	
	print_prompt();
	let mut input = read_stdin()?;
	
	// read input
	loop {
		if input.is_empty() && let Some(default) = default {
			return Ok(default);
		}
		
		// find exact match
		for (i, option) in all_choose_strings.iter().enumerate() {
			if option.eq_ignore_ascii_case(&input) {
				let chosen_index = choose_name_mappings[i];
				return Ok(chosen_index);
			}
		}
		
		println!();
		println!("Invalid option.");
		
		// try fuzzy match
		if let Some(possible_choose_string_index) = custom_fuzzy_search(&input, &all_choose_strings) {
			let possible_option_index = choose_name_mappings[possible_choose_string_index];
			let possible_option = &input_options[possible_option_index];
			let is_hidden_name = possible_option.choose_names.len() > 1 && all_choose_strings[possible_choose_string_index] != &possible_option.choose_names[0];
			if is_hidden_name {
				print!("Did you mean to type \"{}\", for option \"{}\"? (enter nothing to confirm, or re-enter input) ", all_choose_strings[possible_choose_string_index], possible_option.display_name);
			} else {
				print!("Did you mean \"{}\"? (enter nothing to confirm, or re-enter input) ", all_choose_strings[possible_choose_string_index]);
			}
			let new_input = read_stdin()?;
			if new_input.is_empty() {
				let chosen_index = possible_option_index;
				return Ok(chosen_index);
			}
			input = new_input;
		} else {
			print!("Invalid option, please re-enter input: ");
			input = read_stdin()?;
		}
		
	}
}



impl<'a, Data: 'a> TryRead<'a> for &[InputOption<Data>] {
	type Output = (usize, &'a InputOption<Data>);
	type Default = usize;
	fn try_read_line(&'a self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let index = read_list(*self, prompt, default)?;
		Ok((index, &self[index]))
	}
}

impl<'a, Data: 'a, const LEN: usize> TryRead<'a> for [InputOption<Data>; LEN] {
	type Output = (usize, &'a InputOption<Data>);
	type Default = usize;
	fn try_read_line(&'a self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let index = read_list(self, prompt, default)?;
		Ok((index, &self[index]))
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
pub fn custom_fuzzy_search(pattern: &str, items: &[&String]) -> Option<usize> {
	let (mut best_score, mut best_index) = (custom_fuzzy_match(pattern, &items[0]), 0);
	for (i, item) in items.iter().enumerate().skip(1) {
		let score = custom_fuzzy_match(pattern, item);
		if score > best_score {
			best_score = score;
			best_index = i;
		}
	}
	if best_score > 0.0 {
		Some(best_index)
	} else {
		None
	}
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
	pub choose_names: Vec<String>,
	/// What isn't shown to the user
	pub data: Data,
}

impl<Data> InputOption<Data> {
	/// Basic initializer
	pub fn new(display: impl Into<String>, choose: Vec<impl Into<String>>, data: Data) -> Self {
		Self {
			display_name: display.into(),
			choose_names: choose.into_iter().map(|v| v.into()).collect(),
			data,
		}
	}
	/// Internal function
	pub fn get_display_string(&self, is_default: Option<bool>) -> String {
		match (self.choose_names.first(), is_default) {
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
					choose_names: vec!(),
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (&*options).try_read_line(prompt, default)?.0;
		Ok((chosen_index, &self[chosen_index]))
	}
}

impl<'a, T: Display + 'a, const LEN: usize> TryRead<'a> for [T; LEN] {
	type Output = (usize, &'a T);
	type Default = usize;
	fn try_read_line(&'a self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.into_iter()
			.map(|option| {
				InputOption {
					display_name: option.to_string(),
					choose_names: vec!(),
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (&*options).try_read_line(prompt, default)?.0;
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
					choose_names: vec!(),
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (&*options).try_read_line(prompt, default)?.0;
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
					choose_names: vec!(),
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (&*options).try_read_line(prompt, default)?.0;
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
					choose_names: vec!(),
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (&*options).try_read_line(prompt, default)?.0;
		Ok((chosen_index, self.iter().nth(chosen_index).unwrap()))
	}
}
