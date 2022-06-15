use std::fs;

use serde::Serialize;

use crate::extraction_traits::known::KnownItem;

pub trait ExtractCore {
	fn generate_from_index(index: impl KnownItem, write_path: &str) -> Vec<Self> where Self: Sized {
		let mut generated: Vec<Self> = vec![];
		for i in index.get_index() {
			if let Ok(file) = fs::read(format!("{write_path}{i}")) {
				let name = i.split('.').collect::<Vec<&str>>()[0].to_owned();

				let missile = Self::new_from_file(&file, name);

				generated.push(missile);
			}
		}
		Self::sort(&mut generated);
		generated
	}
	fn write_all(items: Vec<Self>, path: &str) where Self: Sized, Self: Serialize {
		fs::write(path, serde_json::to_string_pretty(&items).unwrap()).unwrap();
	}
	fn new_from_file(file: &[u8], name: String) -> Self;
	// Least amount of boilerplate this way, Ord trait is a b*tch to manually implement
	fn sort(items: &mut Vec<Self>) where Self: Sized;
}