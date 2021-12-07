use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct KnownShells {
	pub path: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct BlackList {
	pub path: Vec<String>,
}

impl KnownShells {
	pub fn generate_index() -> Self {
		let mut index: Vec<String> = vec![];
		let folder = fs::read_dir("resources/cache/aces.vromfs.bin_u/gamedata/weapons/groundmodels_weapons").unwrap();
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

	pub fn write_index(self) -> Self {
		fs::write("shell_index/known.json", serde_json::to_string_pretty(&self).unwrap()).unwrap();
		self
	}

	pub fn copy_index_to_folder(self) -> Self {
		for i in &self.path {
			let path = format!("resources/cache/aces.vromfs.bin_u/gamedata/weapons/groundmodels_weapons/{}", i);
			if let Ok(file) = fs::read(&path) {
				fs::write(format!("shell_index/shells/{}", i), &file).unwrap();
			}
		}
		self
	}

	pub fn from_file() -> Self {
		return serde_json::from_str(&fs::read_to_string("shell_index/known.json").unwrap()).unwrap()
	}
}