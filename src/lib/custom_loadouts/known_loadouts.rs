use std::fs;

use crate::extraction_traits::known::{Index, KnownItem, OwnedIndex};

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, Default)]
pub struct KnownLoadouts {
	pub path: Vec<String>,
}

impl KnownItem for KnownLoadouts {
	const READ_FOLDER: &'static str = "resources/cache/aces.vromfs.bin_u/gamedata/flightmodels/";
	const KNOWN_FILE: &'static str = "loadouts/known.json";
	const INDEX_FOLDER: &'static str = "loadouts/index/";

	fn generate_index() -> Self {
		let mut index: Vec<String> = vec![];
		let folder = fs::read_dir(Self::READ_FOLDER).unwrap();
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