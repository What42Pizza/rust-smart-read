use crate::*;
use std::{collections::{LinkedList, VecDeque}, ops::Deref};



// NOTE: the returned usize has to be less than the length of input_options
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
		all_choose_strings.push(option.get_name());
		choose_name_mappings.push(i);
		choose_name_hidden_flags.push(false);
		for alt_name in &option.names[1..] {
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
				print!("Did you mean to type \"{}\", for option \"{}\"? (enter nothing to confirm, or re-enter input) ", all_choose_strings[possible_choose_string_index], possible_option.get_name());
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



// NOTE: the returned usize is always less than the length of self
impl<'a, Data> TryRead for &'a [InputOption<Data>] {
	type Output = (usize, &'a InputOption<Data>);
	type Default = usize;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let chosen_index = read_list(self, prompt, default)?;
		Ok((chosen_index, &self[chosen_index]))
	}
}

// having this does allow for some additional scenarios to compile
// NOTE: the returned usize is always less than the length of self
impl<'a, Data, const LEN: usize> TryRead for &'a [InputOption<Data>; LEN] {
	type Output = (usize, &'a InputOption<Data>);
	type Default = usize;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let chosen_index = read_list(self, prompt, default)?;
		Ok((chosen_index, &self[chosen_index]))
	}
}

// NOTE: the returned usize is always less than the length of self
impl<Data, const LEN: usize> TryRead for [InputOption<Data>; LEN] {
	type Output = (usize, InputOption<Data>);
	type Default = usize;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let chosen_index = read_list(&self, prompt, default)?;
		#[allow(clippy::expect_used)] // REASON: the output of read_list() is always less than the length of the given slice
		Ok((chosen_index, self.into_iter().nth(chosen_index).expect("chosen index is out of bounds")))
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
	let (mut best_score, mut best_index) = (custom_fuzzy_match(pattern, items[0]), 0);
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
/// let mut option_number = 0;
/// let choosable_colors =
/// 	colors.iter().enumerate()
/// 	.filter_map(|(i, color)| {
/// 		let first_char = color.chars().next()?;
/// 		if first_char.is_lowercase() {return None;}
/// 		option_number += 1;
/// 		Some(InputOption::new(option_number, &[*color], i))
/// 	})
/// 	.collect::<Vec<_>>();
/// 
/// // prompt
/// println!("List of colors: {colors:?}");
/// let (_option_index, InputOption {extra_data: index_to_remove, ..}) = prompt!("Choose a color to remove: "; choosable_colors);
/// colors.remove(*index_to_remove);
/// println!("New list of colors: {colors:?}");
/// ```
pub struct InputOption<Data> {
	/// This is what's displayed before the colon
	pub bulletin_string: Option<String>,
	/// The first value is shown as the option's name, and all following values are alternative strings that can be used to select this option
	pub names: Vec<String>,
	/// Extra data for storing whatever you want
	pub extra_data: Data,
}

impl<Data> InputOption<Data> {
	/// Basic initializer
	pub fn new<T: ToString>(bulletin: impl ToString, names: &[T], data: Data) -> Self {
		let names = names.into_iter().map(ToString::to_string).collect::<Vec<_>>();
		Self {
			bulletin_string: Some(bulletin.to_string()),
			names,
			extra_data: data,
		}
	}
	/// Initializer without bulletin string
	pub fn new_without_bulletin<T: ToString>(names: &[T], data: Data) -> Self {
		let names = names.into_iter().map(ToString::to_string).collect::<Vec<_>>();
		Self {
			bulletin_string: None,
			names,
			extra_data: data,
		}
	}
	/// Internal function
	pub fn get_display_string(&self, is_default: Option<bool>) -> String {
		let name = self.get_name();
		match (self.bulletin_string.as_deref(), is_default) {
			(Some(bulletin_string), Some(true )) => format!("[{bulletin_string}]: {name}",),
			(Some(bulletin_string), Some(false)) => format!(" {bulletin_string}:  {name}",),
			(None                       , Some(true )) => format!("[{name}]",),
			(None                       , Some(false)) => format!(" {name} ",),
			(Some(bulletin_string), None       ) => format!("{bulletin_string}: {name}",),
			(None                       , None       ) => name.to_string(),
		}
	}
	/// Gets the name of the option from the start of `self.names`
	/// 
	/// It is assumed that there is at least one value in `names`, but if not, it returns `"[unnamed]"`
	pub fn get_name(&self) -> &str {
		self.names.first().map(Deref::deref).unwrap_or("[unnamed]")
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
					names: vec!(option.to_string()),
					extra_data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (options.deref()).try_read_line(prompt, default)?.0;
		Ok((chosen_index, &self[chosen_index]))
	}
}

// for some reason this one doesn't seem needed
//impl<'a, T: Display, const LEN: usize> TryRead for &'a [T; LEN] {
//	type Output = (usize, &'a T);
//	type Default = usize;
//	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
//		let options = self.iter().enumerate()
//			.map(|(i, option)| {
//				InputOption {
//					bulletin_string: Some((i + 1).to_string()),
//					main_name: option.to_string(),
//					alt_names: vec!(),
//					data: (),
//				}
//			})
//			.collect::<Vec<_>>();
//		let chosen_index = (options.deref()).try_read_line(prompt, default)?.0;
//		Ok((chosen_index, &self[chosen_index]))
//	}
//}

impl<T: Display, const LEN: usize> TryRead for [T; LEN] {
	type Output = (usize, T);
	type Default = usize;
	fn try_read_line(self, prompt: Option<String>, default: Option<Self::Default>) -> BoxResult<Self::Output> {
		let options = self.iter().enumerate()
			.map(|(i, option)| {
				InputOption {
					bulletin_string: Some((i + 1).to_string()),
					names: vec!(option.to_string()),
					extra_data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (options.deref()).try_read_line(prompt, default)?.0;
		#[allow(clippy::expect_used)] // REASON: Vec<InputOption<_>>.try_Read_line().0 is always less than the length of the given vec
		Ok((chosen_index, self.into_iter().nth(chosen_index).expect("chosen index is out of bounds")))
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
					names: vec!(option.to_string()),
					extra_data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (options.deref()).try_read_line(prompt, default)?.0;
		Ok((chosen_index, self.swap_remove(chosen_index)))
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
					names: vec!(option.to_string()),
					extra_data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (options.deref()).try_read_line(prompt, default)?.0;
		#[allow(clippy::expect_used)] // REASON: Vec<InputOption<_>>.try_Read_line().0 is always less than the length of the given vec
		Ok((chosen_index, self.swap_remove_back(chosen_index).expect("chosen index is out of bounds")))
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
					names: vec!(option.to_string()),
					extra_data: (),
				}
			})
			.collect::<Vec<_>>();
		let chosen_index = (options.deref()).try_read_line(prompt, default)?.0;
		#[allow(clippy::expect_used)] // REASON: Vec<InputOption<_>>.try_Read_line().0 is always less than the length of the given vec
		Ok((chosen_index, self.into_iter().nth(chosen_index).expect("chosen index is out of bounds")))
	}
}
