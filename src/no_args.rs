use crate::{read_string, BoxResult, ReadData, ReadLine};



impl ReadLine for () {
	type Output = String;
	fn try_read_line(&self, mut read_data: ReadData<Self::Output>) -> BoxResult<Self::Output> {
		match (read_data.prompt, &read_data.default) {
			(Some(prompt), Some(default)) => print!("{prompt}(default: {default}) "),
			(None, Some(default)) => print!("(default: {default}) "),
			(Some(prompt), None) => print!("{prompt}"),
			(None, None) => {},
		}
		let output = read_string(&mut read_data.input)?;
		Ok(if output.is_empty() && let Some(default) = read_data.default {
			default
		} else {
			output
		})
	}
}
