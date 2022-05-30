use std::fs;

use crate::extraction_traits::known::{Index, KnownItem, OwnedIndex};

pub const KNOWN_LOADOUTS_LOC: &str = "resources/cache/aces.vromfs.bin_u/gamedata/flightmodels/";

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, Default)]
pub struct KnownLoadouts {
	pub path: Vec<String>,
}

impl KnownItem for KnownLoadouts {
	fn generate_index(path: &str) -> Self {
		let mut index: Vec<String> = vec![];
		let folder = fs::read_dir(path).unwrap();
		for i in folder.enumerate() {
			if let Ok(file) = &i.1 {
				if let Ok(contents) = fs::read_to_string(file.path()) {
					if contents.contains("WeaponSlots") {
						index.push(file.file_name().into_string().unwrap());
					}
				}
			}
		}
		index.sort();
		Self {
			path: index,
		}
	}


	fn push_index(&mut self, index: OwnedIndex) {
		self.path = index;
	}

	fn get_index(&self) -> Index {
		&self.path
	}
}