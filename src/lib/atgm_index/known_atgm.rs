use std::fs;
use crate::extraction_traits::known::{Index, KnownItem, OwnedIndex};

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, Default)]
pub struct KnownAirAtgms {
	pub path: Vec<String>,
}

impl KnownItem for KnownAirAtgms {
	const READ_FOLDER: &'static str = "resources/cache/aces.vromfs.bin_u/gamedata/weapons/rocketguns/";
	const KNOWN_FILE: &'static str = "atgm/air_known.json";
	const INDEX_FOLDER: &'static str = "atgm/air_index/";


	fn push_index(&mut self, mut index: OwnedIndex) {
		self.path.append(&mut index);
	}

	fn get_index(&self) -> Index {
		&self.path
	}

	fn copy_index_to_folder(self) -> Self where Self: Sized {
		for i in self.get_index() {
			let file_path = format!("{}{i}", Self::READ_FOLDER);
			if let Ok(file) = fs::read(&file_path) {
				fs::write(format!("{}{i}", Self::KNOWN_FILE), &file).unwrap();
			}
		}
		self
	}
	fn generate_index() -> Self where Self: Default {
		let mut known = Self::default();
		let mut index: Vec<String> = vec![];
		let folder = fs::read_dir(Self::READ_FOLDER).unwrap();
		for i in folder.enumerate() {
			if let Ok(file) = &i.1 {
				if file.file_type().unwrap().is_file() {
					let content = fs::read_to_string(file.path()).unwrap();
					if
						content.contains("guidance") &&
						!content.contains("\"bulletType\": \"aam\"") &&
						content.contains("\"operated\": true")
					{
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