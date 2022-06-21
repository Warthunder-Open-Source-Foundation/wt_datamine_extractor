use get_size::GetSize;

use crate::explosive::explosive::explosive_type_to_tnt;
use crate::extraction_traits::core::ExtractCore;
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

impl ExtractCore for Bomb {
	fn new_from_file(file: &[u8], name: String) -> Self {
		let file = String::from_utf8(file.to_owned()).unwrap();
		let weight = parameter_to_data(&file, "mass").unwrap().parse().unwrap();

		let explosive_mass: f64 = parameter_to_data(&file, "explosiveMass").unwrap_or("0.0".to_owned()).parse().unwrap();

		let explosive_type = parameter_to_data(&file, "explosiveType").unwrap_or("tnt".to_owned()).parse::<String>().unwrap().replace('\"', "");

		let explosive_equiv = explosive_type_to_tnt(&explosive_type, explosive_mass);

		let can_kill_npc_tank = parameter_to_data(&file, "antiTankBomb").unwrap_or("false".to_owned()).parse().unwrap();

		let can_kill_npc_ship = parameter_to_data(&file, "antiShipBomb").unwrap_or("false".to_owned()).parse().unwrap();

		Self {
			name,
			weight,
			explosive_mass,
			explosive_type,
			explosive_equiv,
			can_kill_npc_tank,
			can_kill_npc_ship,
		}
	}

	fn sort(items: &mut Vec<Self>) where Self: Sized {
		items.sort_by_key(|x| x.name.clone());
	}
}
