use std::fs;
use wt_datamine_extractor_lib::missile::extract_missiles::extract_known_missiles;
use wt_datamine_extractor_lib::missile::missile::{generate_raw_missiles, PATH};

fn main() {
	if fs::read_dir("resources/cache").is_ok() {
		extract_known_missiles();
	}
	generate_raw_missiles(PATH);
	println!("Generated new all.json");
}