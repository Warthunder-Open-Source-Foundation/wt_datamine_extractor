use std::fs;

use crate::extraction_traits::known::{Index, KnownItem, OwnedIndex};

pub const KNOWN_THERMALS_LOC_TANK: &str = "resources/cache/aces.vromfs.bin_u/gamedata/units/tankmodels/";
pub const KNOWN_THERMALS_LOC_AIR: &str = "resources/cache/aces.vromfs.bin_u/gamedata/flightmodels/";

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, Default)]
pub struct KnownThermals {
	pub path: Vec<String>,
}

impl KnownItem for KnownThermals {
	// Path dropped, as it varies
	fn generate_index(_: &str) -> Self where Self: Default {
		let mut index: Vec<String> = vec![];
		let folder_tanks = fs::read_dir(KNOWN_THERMALS_LOC_TANK).unwrap();
		for i in folder_tanks.enumerate() {
			if let Ok(file) = &i.1 {
				if let Ok(contents) = fs::read_to_string(file.path()) {
					if contents.contains("gunnerThermal") || contents.contains("commanderViewThermal") {
						index.push(file.file_name().into_string().unwrap());
					}
				}
			}
		}

		let folder_planes = fs::read_dir(KNOWN_THERMALS_LOC_AIR).unwrap();
		for i in folder_planes.enumerate() {
			if let Ok(file) = &i.1 {
				if let Ok(contents) = fs::read_to_string(file.path()) {
					if contents.contains("sightTPodThermal") || contents.contains("sightThermal") {
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

	// Source path dropped, as it varies
	fn copy_index_to_folder(self, _: &str, destination_path: &str) -> Self where Self: Sized {
		for i in &self.path {
			let file_path_plane = format!("{KNOWN_THERMALS_LOC_TANK}{}", i);
			let file_path_tank = format!("{KNOWN_THERMALS_LOC_AIR}{}", i);
			if let Ok(file) = fs::read(&file_path_plane) {
				fs::write(format!("{destination_path}{}", i), &file).unwrap();
			} else if let Ok(file) = fs::read(&file_path_tank) {
				fs::write(format!("{destination_path}{}", i), &file).unwrap();
			}
		}
		self
	}

	fn push_index(&mut self, index: OwnedIndex) {
		self.path = index;
	}

	fn get_index(&self) -> Index {
		&self.path
	}
}