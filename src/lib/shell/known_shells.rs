use std::fs;
use crate::extraction_traits::known::{Index, KnownItem, OwnedIndex};

pub const KNOWN_SHELLS_LOC: &str = "resources/cache/aces.vromfs.bin_u/gamedata/weapons/groundmodels_weapons/";

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Default)]
pub struct KnownShells {
	pub path: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Default)]
pub struct BlackList {
	pub path: Vec<String>,
}

impl KnownShells {
	pub fn from_file() -> Self {
		serde_json::from_str(&fs::read_to_string("shell_index/known.json").unwrap()).unwrap()
	}
}

impl KnownItem for KnownShells {
	fn generate_index(path: &str) -> Self where Self: Default {
			let mut index: Vec<String> = vec![];
			let folder = fs::read_dir(path).unwrap();
			let blacklist: BlackList = serde_json::from_str(&fs::read_to_string("shell_index/blacklist.json").unwrap()).unwrap();
			let blackset = blacklist.path.join(" ");
			for i in folder.enumerate() {
				if let Ok(file) = &i.1 {
					if let Ok(_contents) = fs::read_to_string(file.path()) {
						// for shell_type in SHELL_TYPES {
						// if contents.contains(shell_type) {
						let file_name = file.file_name().into_string().unwrap();
						if !blackset.contains(&file_name) {
							index.push(file_name);
						}
						// }
						// }
					}
				}
			}
			index.sort();
			Self {
				path: index
			}
	}

	fn push_index(&mut self, index: OwnedIndex) {
		self.path = index;
	}

	fn get_index(&self) -> Index {
		&self.path
	}
}