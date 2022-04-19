use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst)]
pub struct KnownLoadouts {
	pub path: Vec<String>,
}

impl KnownLoadouts {
	pub fn generate_index() -> Self {
		let mut index: Vec<String> = vec![];
		let folder = fs::read_dir("resources/cache/aces.vromfs.bin_u/gamedata/flightmodels").unwrap();
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

	pub fn write_index(self) -> Self {
		fs::write("custom_loadouts/known.json", serde_json::to_string_pretty(&self).unwrap()).unwrap();
		self
	}

	pub fn copy_index_to_folder(self) -> Self {
		for i in &self.path {
			let file_path = format!("resources/cache/aces.vromfs.bin_u/gamedata/flightmodels/{}", i);
			if let Ok(file) = fs::read(&file_path) {
				fs::write(format!("custom_loadouts/aircraft/{}", i), &file).unwrap();
			}
		}
		self
	}
}