use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst)]
pub struct KnownBombs {
	pub path: Vec<String>,
}

impl KnownBombs {
	pub fn generate_index() -> Self {
		let mut index: Vec<String> = vec![];
		let folder = fs::read_dir("resources/cache/aces.vromfs.bin_u/gamedata/weapons/bombguns").unwrap();
		for i in folder.enumerate() {
			if let Ok(file) = &i.1 {
				index.push(file.file_name().to_str().unwrap().to_owned())
			}
		}
		index.sort();
		Self {
			path: index,
		}
	}

	pub fn write_index(self) -> Self {
		fs::write("bombs/known.json", serde_json::to_string_pretty(&self).unwrap()).unwrap();
		self
	}

	pub fn copy_index_to_folder(self) -> Self {
		for i in &self.path {
			let file_path = format!("resources/cache/aces.vromfs.bin_u/gamedata/weapons/bombguns/{}", i);
			if let Ok(file) = fs::read(&file_path) {
				fs::write(format!("bombs/index/{}", i), &file).unwrap();
			}
		}
		self
	}
}