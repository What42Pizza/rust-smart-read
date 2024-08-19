use crate::*;
use std::{collections::{LinkedList, VecDeque}, mem::MaybeUninit};



fn read_list<Data>(input_options: &[InputOption<Data>], prompt: Option<String>, default: Option<usize>) -> BoxResult<usize> {
	if input_options.is_empty() {return Err(Box::new(ListConstraintError::EmptyList));}
	
	// get prompt data
	let prompt = prompt.unwrap_or(String::from("Enter one of the following:"));
	let display_strings =
		input_options.iter().enumerate()
		.map(|(i, option)| {
			option.get_display_string(default.map(|default| i == default))
		})
		.collect::<Vec<_>>();
	
	// lists for string to match against, which option that string goes with, and whether that string is an alt_name
	let (mut all_choose_strings, mut choose_name_mappings, mut choose_name_hidden_flags) = (vec!(), vec!(), vec!());
	for (i, option) in input_options.iter().enumerate() {
		if let Some(bulletin_string) = option.bulletin_string.as_deref() {
			all_choose_strings.push(bulletin_string);
			choose_name_mappings.push(i);
			choose_name_hidden_flags.push(false);
		}
		all_choose_strings.push(&*option.main_name);
		choose_name_mappings.push(i);
		choose_name_hidden_flags.push(false);
		for alt_name in &option.alt_names {
			all_choose_strings.push(alt_name);
			choose_name_mappings.push(i);
			choose_name_hidden_flags.push(true);
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
			if choose_name_hidden_flags[possible_choose_string_index] {
				print!("Did you mean to type \"{}\", for option \"{}\"? (enter nothing to confirm, or re-enter input) ", all_choose_strings[possible_choose_string_index], possible_option.main_name);
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



impl<'a, Data> TryRead for &'a [InputOption<Data>] {
	type Output = (usize, &'a InputOption<Data>);
	type Default = usize;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let chosen_index = read_list(self, prompt, default)?;
		Ok((chosen_index, &self[chosen_index]))
	}
}

impl<Data, const LEN: usize> TryRead for [InputOption<Data>; LEN] {
	type Output = (usize, InputOption<Data>);
	type Default = usize;
	fn try_read_line(mut self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let chosen_index = read_list(&self, prompt, default)?;
		#[allow(invalid_value)]
		let chosen_item = std::mem::replace(&mut self[chosen_index], unsafe {MaybeUninit::zeroed().assume_init()}); // consume self and return chosen item
		Ok((chosen_index, chosen_item))
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
pub fn custom_fuzzy_search(pattern: &str, items: &[&str]) -> Option<usize> {
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
/// let mut option_number = 1;
/// let choosable_colors =
/// 	colors.iter().enumerate()
/// 	.filter_map(|(i, color_name)| {
/// 		let first_char = color_name.chars().next()?;
/// 		if first_char.is_lowercase() {return None;}
/// 		Some(OptionWithData::new(option_number, color_name, vec!(), i))
/// 	})
/// 	.collect::<Vec<_>>();
/// 
/// // prompt
/// let OptionWithData {data: index_to_remove, ..} = prompt!("Choose a color to remove: "; choosable_colors);
/// colors.remove(index_to_remove);
/// ```
pub struct InputOption<Data> {
	/// This is what's displayed before the colon
	pub bulletin_string: Option<String>,
	/// This is what's shown as the option name
	pub main_name: String,
	/// These are alternate valid strings that the user could enter to choose this option
	pub alt_names: Vec<String>,
	/// Extra data for storing whatever you want
	pub data: Data,
}

impl<Data> InputOption<Data> {
	/// Basic initializer
	pub fn new(bulletin: impl Into<String>, display: impl Into<String>, choose: Vec<impl Into<String>>, data: Data) -> Self {
		Self {
			bulletin_string: Some(bulletin.into()),
			main_name: display.into(),
			alt_names: choose.into_iter().map(|v| v.into()).collect(),
			data,
		}
	}
	/// Initializer without bulletin string
	pub fn new_without_bulletin(display: impl Into<String>, choose: Vec<impl Into<String>>, data: Data) -> Self {
		Self {
			bulletin_string: None,
			main_name: display.into(),
			alt_names: choose.into_iter().map(|v| v.into()).collect(),
			data,
		}
	}
	/// Internal function
	pub fn get_display_string(&self, is_default: Option<bool>) -> String {
		match (self.bulletin_string.as_deref(), is_default) {
			(Some(bulletin_string), Some(true )) => format!("[{bulletin_string}]: {}", self.main_name),
			(Some(bulletin_string), Some(false)) => format!(" {bulletin_string}:  {}", self.main_name),
			(None                       , Some(true )) => format!("[{}]", self.main_name),
			(None                       , Some(false)) => format!(" {} ", self.main_name),
			(Some(bulletin_string), None       ) => format!("{bulletin_string}: {}", self.main_name),
			(None                       , None       ) => format!("{}", self.main_name),
		}
	}
}





impl<'a, T: Display> TryRead for &'a [T] {
	type Output = (usize, &'a T);
	type Default = usize;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.iter().enumerate()
			.map(|(i, option)| {
				InputOption {
					bulletin_string: Some((i + 1).to_string()),
					main_name: option.to_string(),
					alt_names: vec!(),
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (&*options).try_read_line(prompt, default)?.0;
		Ok((chosen_index, &self[chosen_index]))
	}
}

impl<T: Display, const LEN: usize> TryRead for [T; LEN] {
	type Output = (usize, T);
	type Default = usize;
	fn try_read_line(mut self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.iter().enumerate()
			.map(|(i, option)| {
				InputOption {
					bulletin_string: Some((i + 1).to_string()),
					main_name: option.to_string(),
					alt_names: vec!(),
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (&*options).try_read_line(prompt, default)?.0;
		let chosen_item = std::mem::replace(&mut self[chosen_index], unsafe {MaybeUninit::zeroed().assume_init()}); // consume self and return chosen item
		Ok((chosen_index, chosen_item))
	}
}

impl<T: Display> TryRead for Vec<T> {
	type Output = (usize, T);
	type Default = usize;
	fn try_read_line(mut self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.iter().enumerate()
			.map(|(i, option)| {
				InputOption {
					bulletin_string: Some((i + 1).to_string()),
					main_name: option.to_string(),
					alt_names: vec!(),
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (&*options).try_read_line(prompt, default)?.0;
		let chosen_item = std::mem::replace(&mut self[chosen_index], unsafe {MaybeUninit::zeroed().assume_init()}); // consume self and return chosen item
		Ok((chosen_index, chosen_item))
	}
}

impl<T: Display> TryRead for VecDeque<T> {
	type Output = (usize, T);
	type Default = usize;
	fn try_read_line(mut self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.iter().enumerate()
			.map(|(i, option)| {
				InputOption {
					bulletin_string: Some((i + 1).to_string()),
					main_name: option.to_string(),
					alt_names: vec!(),
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (&*options).try_read_line(prompt, default)?.0;
		let chosen_item = std::mem::replace(&mut self[chosen_index], unsafe {MaybeUninit::zeroed().assume_init()}); // consume self and return chosen item
		Ok((chosen_index, chosen_item))
	}
}

impl<T: Display> TryRead for LinkedList<T> {
	type Output = (usize, T);
	type Default = usize;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.iter().enumerate()
			.map(|(i, option)| {
				InputOption {
					bulletin_string: Some((i + 1).to_string()),
					main_name: option.to_string(),
					alt_names: vec!(),
					data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (&*options).try_read_line(prompt, default)?.0;
		Ok((chosen_index, self.into_iter().nth(chosen_index).unwrap()))
	}
}
