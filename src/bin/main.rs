use std::fs;
use wt_datamine_extractor_lib::missile::extract_missiles::{extract_known_missiles, KnownMissiles};
use wt_datamine_extractor_lib::missile::missile::{generate_raw_missiles, PATH};
use wt_datamine_extractor_lib::thermal::extract_thermals::KnownThermals;

fn main() {
	// if fs::read_dir("resources/cache").is_ok() {
	// 	extract_known_missiles();
	// }
	// generate_raw_missiles(PATH);
	// println!("Generated new all.json");

	KnownThermals::generate_index().write_index().copy_index_to_folder();
}