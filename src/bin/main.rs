use std::fs;
use wt_datamine_extractor_lib::extract_missiles::extract_known_missiles;
use wt_datamine_extractor_lib::missiles::{generate_raw, PATH};

fn main() {
	if fs::read_dir("resources/cache").is_ok() {
		extract_known_missiles();
	}
	generate_raw(PATH);
	println!("Generated new all.json");
}