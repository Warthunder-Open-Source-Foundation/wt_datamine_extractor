pub fn parameter_to_data(file: &str, parameter: &str) -> Option<String> {
	file.find(&format!("\"{}\"", parameter)).map(|value| {
		let position_value = file.split_at(value + parameter.len() + 3).1;
		let cropped_value = position_value.split_once('\n').unwrap().0;
		let cleaned_value = cropped_value.replace(',', "").trim().to_owned(); // Sub-objects somehow contain a comma
		cleaned_value
	})
}

pub fn is_clrf(file: &str)  -> bool {
	 file.contains("\r\n")
}

pub fn get_sep(file: &str) -> String {
	return if file.contains("\r\n")  {
		"\r\n"
	} else {
		"\n"
	}.to_owned()
}

#[cfg(test)]
mod tests {}