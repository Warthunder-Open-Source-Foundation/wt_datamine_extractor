use std::fs;
use crate::extraction_traits::known::{Index, KnownItem, OwnedIndex};

pub const KNOWN_ATGM_LOC: &str = "resources/cache/aces.vromfs.bin_u/gamedata/weapons/rocketguns/";

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, Default)]
pub struct KnownAtgms {
	pub path: Vec<String>,
}

impl KnownItem for KnownAtgms {
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
}