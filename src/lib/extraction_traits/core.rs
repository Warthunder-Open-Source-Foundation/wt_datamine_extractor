use std::fs;
use crate::extraction_traits::known::KnownItem;

pub trait ExtractCore {
	fn generate_from_index(index: impl KnownItem, write_path: &str) -> Vec<Self> where Self: Sized, Self: Ord {
		let mut generated: Vec<Self> = vec![];
		for i in index.get_index() {
			if let Ok(file) = fs::read(format!("{write_path}{i}")) {
				let name = i.split('.').collect::<Vec<&str>>()[0].to_owned();

				let missile = Self::new_from_file(&file, name);

				generated.push(missile);
			}
		}
		generated.sort();
		generated
	}
	fn write_all(&self);
	fn new_from_file(file: &[u8], name: String) -> Self;
}