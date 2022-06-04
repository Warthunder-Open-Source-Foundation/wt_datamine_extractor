use std::fs;

pub type Index<'a> = &'a Vec<String>;
pub type OwnedIndex = Vec<String>;

pub trait KnownItem {
	/// Creates index by reading generic indexed single layer folder
	fn generate_index(path: &str) -> Self where Self: Default {
		let mut known = Self::default();
		let mut index: Vec<String> = vec![];
		let folder = fs::read_dir(path).unwrap();
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
	fn write_index(self, path: &str) -> Self where Self: serde::Serialize + Sized {
		fs::write(path, serde_json::to_string_pretty(&self).unwrap()).unwrap();
		self
	}
	/// Copies index from cache folder into designated one
	#[must_use]
	fn copy_index_to_folder(self, format_path: &str, destination_path: &str) -> Self where Self: Sized {
		for i in self.get_index() {
			let file_path = format!("{format_path}{i}");
			if let Ok(file) = fs::read(&file_path) {
				fs::write(format!("{destination_path}{i}"), &file).unwrap();
			}
		}
		self
	}

	/// Pushes items onto index for storing in any way
	fn push_index(&mut self, index: OwnedIndex);
	// Retrieves index out of struct
	fn get_index(&self) -> Index;
}