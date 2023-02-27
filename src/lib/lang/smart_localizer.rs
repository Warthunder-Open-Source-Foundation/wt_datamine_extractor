

pub enum LocalizerLevel {
	// Only returns localization from direct in-game file
	Strict = 0,

	// Returns auto-solution or better
	Best = 1,

	// Returns any
	Any = 2,
}


const NATION_PREFIXES: [&str; 10] = [
	"us_",
	"de_",
	"su_",
	"gb_",
	"jp_",
	"cn_",
	"it_",
	"fr_",
	"sw_",
	"il_",
];

fn uppercase_char(mut input: String, idx: usize) -> Option<String> {
	input.replace_range(idx..(idx + 1), &input.clone().get(idx..(idx + 1))?.to_uppercase());
	Some(input)
}

pub fn auto_localize(mut input: impl ToString) -> Option<String> {
	let mut input: String = input.to_string();

	input = input.chars().filter(|c| c.is_ascii()).collect::<String>();

	// Strip nation prefix
	for NATION_PREFIX in NATION_PREFIXES {
		if input.starts_with(NATION_PREFIX) {
			input = input.replacen(NATION_PREFIX, "", 1);
			break;
		}
	}

	// Uppercase first letter
	input = uppercase_char(input, 0)?;

	// Naively replace underscores to hyphen
	input = input.replace("_", "-");

	// Convert letters to uppercase as long as previous was data
	let mut upper = false;
	for (i, char) in input.clone().char_indices() {
		if char.is_numeric() {
			upper = true;
			continue;
		}
		if upper && char.is_alphabetic() {
			input = uppercase_char(input, i)?;
			continue;
		}
		if char == '-' {
			upper = false;
			continue;
		}
	}

	Some(input)
}

#[cfg(test)]
mod test {
	use crate::lang::smart_localizer::auto_localize;

	// #[test]
	// fn locale() {
	// 	println!("{}", auto_localize("su_r_23r").unwrap());
	// }
}