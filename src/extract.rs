use std::fs;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct KnownMissiles {
	pub path: Vec<String>,
}

impl KnownMissiles {
	pub fn new_from_index(known: Vec<String>) -> Self {
		Self {
			path: known,
		}
	}
}

pub fn extract_known() {
	let known_raw = fs::read_to_string("index/known.json").unwrap();
	let known: KnownMissiles = serde_json::from_str(&known_raw).unwrap();
	for (_, known) in known.path.iter().enumerate() {
		let path = format!("resources/cache/aces.vromfs.bin_u/gamedata/weapons/rocketguns/{}", known);
		if let Ok(contents) = fs::read(path) {
			fs::write(format!("index/missiles/{}", known), contents).unwrap();
		}else {
			println!("Cannot find {}", known);
		}
	}
}