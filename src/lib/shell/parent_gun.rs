use std::fs;
use get_size::GetSize;

#[derive(serde::Serialize, Clone, serde::Deserialize, Debug, PartialEq, Hash, Eq, const_gen::CompileConst, get_size::GetSize)]
pub struct ParentGun {
	pub name: String,
	pub localized: String,
}

impl ParentGun {
	pub fn from_paths(paths: Vec<&str>) -> Vec<Self> {
		let mut files: Vec<Self> = vec![];
		for path in paths {
			files.push(
				serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap()
			);
		}
		files
	}
}