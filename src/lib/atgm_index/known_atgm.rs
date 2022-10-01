use std::fs;
use crate::extraction_traits::known::{Index, KnownItem, OwnedIndex};

pub const KNOWN_AIR_ATGM_LOC: &str = "resources/cache/aces.vromfs.bin_u/gamedata/weapons/rocketguns/";

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, Default)]
pub struct KnownAirAtgms {
	pub path: Vec<String>,
}

impl KnownItem for KnownAirAtgms {
	fn push_index(&mut self, mut index: OwnedIndex) {
		self.path.append(&mut index);
	}

	fn get_index(&self) -> Index {
		&self.path
	}

	fn copy_index_to_folder(self, format_path: &str, destination_path: &str) -> Self where Self: Sized {
		for i in self.get_index() {
			let file_path = format!("{format_path}{i}");
			if let Ok(file) = fs::read(&file_path) {
				fs::write(format!("{destination_path}{i}"), &file).unwrap();
			}
		}
		self
	}
	fn generate_index(path: &str) -> Self where Self: Default {
		let mut known = Self::default();
		let mut index: Vec<String> = vec![];
		let folder = fs::read_dir(path).unwrap();
		for i in folder.enumerate() {
			if let Ok(file) = &i.1 {
				if file.file_type().unwrap().is_file() {
					let content = fs::read_to_string(file.path()).unwrap();
					if content.contains("guidance") && !content.contains("\"bulletType\": \"aam\"") {
						index.push(file.file_name().to_str().unwrap().to_owned());
					}
				}
			}
		}
		index.sort();
		known.push_index(index);
		known
	}
}