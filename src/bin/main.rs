use std::fs;
use wt_missile_calc_lib::extract::extract_known;
use wt_missile_calc_lib::missiles::{generate_raw, PATH};

fn main() {
	if fs::read_dir("resources/cache").is_ok() {
		extract_known();
	}
	generate_raw(PATH);
	println!("Generated new all.json");
}