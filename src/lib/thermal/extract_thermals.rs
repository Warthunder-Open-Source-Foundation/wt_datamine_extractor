use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst)]
pub struct KnownThermals {
	pub path: Vec<String>,
}

impl KnownThermals {
	pub fn generate_index() -> Self {
		let mut index: Vec<String> = vec![];
		let folder_tanks = fs::read_dir("resources/cache/aces.vromfs.bin_u/gamedata/units/tankmodels").unwrap();
		for i in folder_tanks.enumerate() {
			if let Ok(file) = &i.1 {
				if let Ok(contents) = fs::read_to_string(file.path()) {
					if contents.contains("gunnerThermal") || contents.contains("commanderViewThermal") {
						index.push(file.file_name().into_string().unwrap());
					}
				}
			}
		}

		let folder_planes = fs::read_dir("resources/cache/aces.vromfs.bin_u/gamedata/flightmodels").unwrap();
		for i in folder_planes.enumerate() {
			if let Ok(file) = &i.1 {
				if let Ok(contents) = fs::read_to_string(file.path()) {
					if contents.contains("sightTPodThermal") || contents.contains("sightThermal"){
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
		fs::write("thermal_index/known.json", serde_json::to_string_pretty(&self).unwrap()).unwrap();
		self
	}

	pub fn copy_index_to_folder(self) -> Self {
		for i in &self.path {
			let file_path_plane = format!("resources/cache/aces.vromfs.bin_u/gamedata/flightmodels/{}", i);
			let file_path_tank = format!("resources/cache/aces.vromfs.bin_u/gamedata/units/tankmodels/{}", i);
			if let Ok(file) = fs::read(&file_path_plane) {
				fs::write(format!("thermal_index/thermals/{}", i), &file).unwrap();
			} else if let Ok(file) = fs::read(&file_path_tank) {
				fs::write(format!("thermal_index/thermals/{}", i), &file).unwrap();
			}
		}
		self
	}
}