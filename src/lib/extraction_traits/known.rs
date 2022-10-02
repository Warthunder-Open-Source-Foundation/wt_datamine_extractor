use std::fs;

pub type Index<'a> = &'a Vec<String>;
pub type OwnedIndex = Vec<String>;

pub trait KnownItem {

	/// Where the folder sits relative to the project node, within the `/resources/` directory
	const READ_FOLDER: &'static str;

	/// File relative to the project node pointing to the xx.json
	const KNOWN_FILE: &'static str;

	/// Folder containing in-game files copied for git-tracking as defined by `KNOWN_FILE`
	const INDEX_FOLDER: &'static str;

	/// Creates index by reading generic indexed single layer folder
	fn generate_index() -> Self where Self: Default {
		let mut known = Self::default();
		let mut index: Vec<String> = vec![];
		let folder = fs::read_dir(Self::READ_FOLDER).unwrap();
		for i in folder.enumerate() {
			if let Ok(file) = &i.1 {
				index.push(file.file_name().to_str().unwrap().to_owned());
			}
		}
		index.sort();
		known.push_index(index);
		known
	}
	/// Stores index to usual known file
	#[must_use]
	fn write_index(self) -> Self where Self: serde::Serialize + Sized {
		fs::write(Self::KNOWN_FILE, serde_json::to_string_pretty(&self).unwrap()).unwrap();
		self
	}
	/// Copies index from cache folder into designated one
	#[must_use]
	fn copy_index_to_folder(self) -> Self where Self: Sized {
		for i in self.get_index() {
			let file_path = format!("{}{i}", Self::READ_FOLDER);
			if let Ok(file) = fs::read(&file_path) {
				fs::write(format!("{}{i}", Self::INDEX_FOLDER), &file).unwrap();
			}
		}
		self
	}

	/// Pushes items onto index for storing in any way
	fn push_index(&mut self, index: OwnedIndex);
	// Retrieves index out of struct
	fn get_index(&self) -> Index;
}