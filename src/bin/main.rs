use wt_missile_calc_lib::missiles::{generate_raw, PATH};

fn main() {
	generate_raw(PATH);
	println!("Generated new all.json");
}