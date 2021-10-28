use std::fs;

use crate::missiles::{Missile};

mod missiles;

const PATH: &str = "./missiles/";

fn main() {
	let dir_ir = fs::read_dir(format!("{}IR", PATH)).unwrap();
	let dir_rd = fs::read_dir(format!("{}Radar", PATH)).unwrap();

	let mut files: Vec<String> = vec![];
	for (_, entry) in dir_ir.enumerate() {

		let file_name = entry.unwrap().file_name().into_string().unwrap();
		if file_name.contains("blkx") {
			files.push(format!("{}IR/{}", PATH, file_name));
		}
	}
	for (_, entry) in dir_rd.enumerate() {
		let file_name = entry.unwrap().file_name().into_string().unwrap();
		if file_name.contains("blkx") {
			files.push(format!("{}Radar/{}", PATH, file_name));
		}
	}

	let mut missiles: Vec<Missile> = vec![];
	for file in files {
		let data = fs::read(&file).unwrap();
		missiles.push(Missile::new_from_file(&data, file));
	}
	let write = serde_json::to_string_pretty(&missiles).unwrap();
	fs::write("./all.json", write).unwrap();
	//println!("{:#?}", missiles);
}
