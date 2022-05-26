use std::collections::HashMap;
use std::str::FromStr;

use lazy_static::lazy_static;

lazy_static! {
    static ref EXPLOSIVE_FILE: String = {
        std::fs::read_to_string("explosive/explosive.blkx").unwrap()
		};
	static ref EXPLOSIVE_TYPES: Vec<Explosive> = {
		Explosive::new_from_file()
	};
	static ref EXPLOSIVE_MAP: HashMap<String, f64> = {
		let mut map: HashMap<String, f64> = HashMap::new();
		for item in EXPLOSIVE_TYPES.iter() {
			map.insert(item.e_type.clone(), item.k);
		}
		map
	};
}

#[derive(Debug, const_gen::CompileConst)]
pub struct Explosive {
	pub e_type: String,
	pub k: f64,
}

impl Explosive {
	pub fn new_from_file() -> Vec<Self> {
		let file = EXPLOSIVE_FILE.clone();
		let mut types: Vec<Self> = vec![];

		let split = &file.split('\n').collect::<Vec<&str>>()[2..];
		let mut pos = 0;

		loop {
			let explosive_name = split[pos].replace('"', "").replace("{", "").replace(":", "");
			let equiv = split[pos + 1].split(':').collect::<Vec<&str>>()[1].replace(",", "");
			if let Ok(equiv) = f64::from_str(equiv.trim()) {
				types.push(Explosive {
					e_type: explosive_name.trim().to_owned(),
					k: equiv,
				});
				pos += 4;
			} else {
				break;
			}
			if pos > 1000 {
				panic!("Explosive analysis overran acceptable buffer")
			}
		}
		types
	}
}

pub fn explosive_type_to_tnt(e_type: &str, raw_mass: f64) -> f64 {
	if raw_mass == 0.0 || e_type.is_empty() {
		return 0.0;
	}
	if let Some(k) = EXPLOSIVE_MAP.get(e_type) {
		return (k * f64::from(raw_mass)).round();
	};
	panic!("Cannot resolve {} {}", e_type, raw_mass)
}