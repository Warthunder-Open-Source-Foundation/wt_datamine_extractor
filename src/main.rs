use std::fs;

use crate::extract::{extract_known, KnownMissiles};
use crate::html_gen::generate_html;
use crate::missiles::Missile;

mod missiles;
mod extract;
mod html_gen;

const PATH: &str = "./index/missiles";


fn main() {
// 	extract_known();
//
// 	generate_raw();

	generate_html(vec![])
}

fn generate_raw() {
	let dir_ir = fs::read_dir(format!("{}", PATH)).unwrap();

	let mut files: Vec<String> = vec![];
	let mut known: KnownMissiles = KnownMissiles::new_from_index(vec![]);
	for (_, entry) in dir_ir.enumerate() {
		let file_name = entry.unwrap().file_name().into_string().unwrap();
		if file_name.contains("blkx") {
			files.push(format!("{}/{}", PATH, file_name));
			known.path.push(file_name);
		}
	}

	let mut missiles: Vec<Missile> = vec![];
	for file in files {
		let data = fs::read(&file).unwrap();
		missiles.push(Missile::new_from_file(&data, file));
	}

	let known_json = serde_json::to_string_pretty(&known).unwrap();
	fs::write("./index/known.json", known_json).unwrap();

	let missiles_json = serde_json::to_string_pretty(&missiles).unwrap();
	fs::write("./all.json", missiles_json).unwrap();
	//println!("{:#?}", missiles);
}
