use std::fs;

use crate::extraction_traits::known::{Index, KnownItem, OwnedIndex};

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, Default)]
pub struct KnownMissiles {
	pub path: Vec<String>,
}

impl KnownItem for KnownMissiles {
	const READ_FOLDER: &'static str = "resources/cache/aces.vromfs.bin_u/gamedata/weapons/rocketguns/";
	const KNOWN_FILE: &'static str = "missile_index/known.json";
	const INDEX_FOLDER: &'static str = "missile_index/missiles/";

	fn generate_index() -> Self where Self: Default {
		let mut index: Vec<String> = vec![];
		let folder = fs::read_dir(Self::READ_FOLDER).unwrap();
		for i in folder.enumerate() {
			if let Ok(file) = &i.1 {
				if let Ok(contents) = fs::read_to_string(file.path()) {
					// Radar missiles 							IR missiles								 That arent F&F ATGMs
					if (contents.contains("radarSeeker") || contents.contains("irSeeker")) && contents.contains("\"bulletType\": \"aam\"")
					{
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