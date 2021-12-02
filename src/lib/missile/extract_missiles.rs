use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct KnownMissiles {
	pub path: Vec<String>,
}

impl KnownMissiles {
	pub fn generate_index() -> Self {
		let mut index: Vec<String> = vec![];
		let folder = fs::read_dir("resources/cache/aces.vromfs.bin_u/gamedata/weapons/rocketguns").unwrap();
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

	pub fn write_index(self) -> Self {
		fs::write("missile_index/known.json", serde_json::to_string_pretty(&self).unwrap()).unwrap();
		self
	}

	pub fn copy_index_to_folder(self) -> Self {
		for i in &self.path {
			let file_path = format!("resources/cache/aces.vromfs.bin_u/gamedata/weapons/rocketguns/{}", i);
			if let Ok(file) = fs::read(&file_path) {
				fs::write(format!("missile_index/missiles/{}", i), &file).unwrap();
			}
		}
		self
	}
}