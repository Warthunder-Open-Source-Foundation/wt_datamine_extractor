use std::fs;
use get_size::GetSize;

use crate::bombs::known_bombs::KnownBombs;
use crate::explosive::explosive::explosive_type_to_tnt;

use crate::util::parameter_to_data;



#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, const_gen::CompileConst, get_size::GetSize)]
pub struct Bomb {
	pub name: String,
	pub weight: f64,
	pub explosive_mass: f64,
	pub explosive_type: String,
	pub explosive_equiv: f64,
	pub can_kill_npc_tank: bool,
	pub can_kill_npc_ship: bool,
}

impl Bomb {
	pub fn new_from_file(file: &[u8], name: String) -> Self {
		let file = String::from_utf8(file.to_owned()).unwrap();
		let weight = parameter_to_data(&file, "mass").unwrap().parse().unwrap();

		let explosive_mass: f64 = parameter_to_data(&file, "explosiveMass").unwrap_or("0.0".to_owned()).parse().unwrap();

		let explosive_type = parameter_to_data(&file, "explosiveType").unwrap_or("tnt".to_owned()).parse::<String>().unwrap().replace('\"', "");

		let explosive_equiv = explosive_type_to_tnt(&explosive_type, explosive_mass);

		let can_kill_npc_tank = parameter_to_data(&file, "antiTankBomb").unwrap_or("false".to_owned()).parse().unwrap();

		let can_kill_npc_ship = parameter_to_data(&file, "antiShipBomb").unwrap_or("false".to_owned()).parse().unwrap();

		Bomb {
			name,
			weight,
			explosive_mass,
			explosive_type,
			explosive_equiv,
			can_kill_npc_tank,
			can_kill_npc_ship
		}
	}

	pub fn write_all(mut values: Vec<Self>) -> Vec<Self> {
		values.sort_by_key(|d| d.name.clone());
		fs::write("bombs/all.json", serde_json::to_string_pretty(&values).unwrap()).unwrap();
		values
	}

	pub fn generate_from_index(index: &KnownBombs) -> Vec<Self> {
		let mut generated: Vec<Self> = vec![];
		for i in &index.path {
			if let Ok(file) = fs::read(format!("bombs/index/{}", i)) {
				let name = i.split('.').collect::<Vec<&str>>()[0].to_owned();

				let bomb = Bomb::new_from_file(&file, name);

				generated.push(bomb);
			}
		}
		generated.sort_by_key(|x| x.name.clone());
		generated
	}
}
