use std::fs;

use get_size::GetSize;

use crate::custom_loadouts::custom_loadouts::WeaponType::{AAM, AGM, Bomb, Cannon, Countermeasures, Empty, GBU, GunPod, Rocket, TargetingPod};
use crate::extraction_traits::core::ExtractCore;
use crate::lang::{Lang, name_to_local};
use crate::util::parameter_to_data;

/*
Steps todo:
DONE 1. Save pylon weapon to use TGP
DONE 2. Validate current setups to properly use TGP if not selected (new error)
3. Make partial fn for interactive web view to display automatic requirement
 */

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, const_gen::CompileConst, get_size::GetSize)]
pub struct CustomLoadout {
	pub aircraft: String,
	pub localized: String,
	pub max_load: f64,
	pub max_imbalance: f64,
	pub max_wing_load: (f64, f64),
	pub pylons: Vec<Pylon>,
	pub misc_pylons: Vec<Pylon>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, const_gen::CompileConst, get_size::GetSize)]
pub struct Pylon {
	pub index: u32,
	pub tier: Option<u32>,
	pub order: Option<u32>,
	pub exempt_from_imbalance: bool,
	pub weapons: Vec<Weapon>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, const_gen::CompileConst, get_size::GetSize)]
pub struct Weapon {
	pub name: String,
	pub localized: String,
	pub icon_type: String,
	pub count: u32,
	pub individual_mass: f64,
	pub total_mass: f64,
	pub weapon_type: WeaponType,
	pub depend_on: Option<Dependent>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, const_gen::CompileConst, get_size::GetSize)]
pub struct Dependent {
	pub name: String,
	pub slot: u32,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, Copy, const_gen::CompileConst, get_size::GetSize)]
pub enum WeaponType {
	Cannon,
	Rocket,
	Bomb,
	GBU,
	AGM,
	AAM,
	Countermeasures,
	GunPod,
	TargetingPod,
	Empty,
}

impl WeaponType {
	fn from_str(input: &str) -> Self {
		return match input {
			r#""rockets""# => Rocket,
			r#""bombs""# => Bomb,
			r#""guided bombs""# => GBU,
			r#""cannon""# => Cannon,
			r#""aam""# => AAM,
			r#""countermeasures""# => Countermeasures,
			r#""atgm""# => AGM,
			r#""additional gun""# => GunPod,
			// Dummies are used as targeting pods
			r#""targetingPod""# | r#""gunner0""# => TargetingPod,
			// Yep, we got both in and out of quotes
			r#""empty""# | "empty" => Empty,
			_ => {
				panic!("Cannot get Weapon from {input}");
			}
		};
	}
}

impl CustomLoadout {
	pub fn select_by_name(loadouts: &[Self], name: &str) -> Option<Self> {
		for (i, loadout) in loadouts.iter().enumerate() {
			if loadout.aircraft.contains(&name.replace('-', "_")) {
				return Some(loadouts[i].clone());
			}
		}
		None
	}
}

impl ExtractCore for CustomLoadout {
	#[allow(clippy::too_many_lines)]
	fn new_from_file(file: &[u8], name: String) -> Self {
		let file = String::from_utf8(file.to_vec()).unwrap();

		let max_load = parameter_to_data(&file, "maxloadMass").unwrap().parse().unwrap();

		let max_imbalance = parameter_to_data(&file, "maxDisbalance").unwrap().parse().unwrap();

		let max_wing_load = (
			parameter_to_data(&file, "maxloadMassLeftConsoles").unwrap().parse().unwrap(),
			parameter_to_data(&file, "maxloadMassRightConsoles").unwrap().parse().unwrap()
		);

		let mut pylons: Vec<Pylon> = vec![];
		let mut misc_pylons: Vec<Pylon> = vec![];

		let mut split_pylons = file.split("\"WeaponSlot\"").collect::<Vec<&str>>();
		split_pylons.remove(0);
		for slot in split_pylons {
			let exempt_from_imbalance = parameter_to_data(slot, "notUseforDisbalanceCalculation").unwrap_or("false".to_owned()).parse().unwrap();

			let index = parameter_to_data(slot, "index").unwrap().parse().unwrap();

			let tier = parameter_to_data(slot, "tier").map(|param| param.parse::<u32>().unwrap());

			let order = parameter_to_data(slot, "order").map(|param| param.parse::<u32>().unwrap());

			let mut weapons: Vec<Weapon> = vec![];

			let mut split_weapons = slot.split("\"WeaponPreset\"").collect::<Vec<&str>>();
			split_weapons.remove(0);

			for preset in split_weapons {
				let weapon_type = WeaponType::from_str(&parameter_to_data(preset, "trigger").unwrap_or("empty".to_owned()));

				if weapon_type == WeaponType::Empty {
					weapons.push(Weapon {
						count: 0,
						individual_mass: 0.0,
						name: "Empty".to_owned(),
						total_mass: 0.0,
						weapon_type,
						localized: "Empty".to_owned(),
						icon_type: "".to_owned(),
						depend_on: None,
					});
					continue;
				}

				let blk_path = parameter_to_data(preset, "blk").unwrap();

				let mut mass: f64 = 0.0;
				let mut count: u32 = 0;
				let mut projectile_name = "".to_owned();
				get_container_weight(&blk_path, &mut mass, &mut count, &mut projectile_name);
				let weight = mass * f64::from(count);

				let name = parameter_to_data(preset, "name").unwrap().replace('\"', "");
				let direct_local = name_to_local(&name, &Lang::Weapon);

				let localized = match () {
					_ if !projectile_name.is_empty() => {
						name_to_local(&projectile_name.replace('\"', ""), &Lang::Weapon)
					}
					_ if name != direct_local => {
						parameter_to_data(preset, "reqModification").unwrap_or(name.clone()).replace('\"', "")
					}
					_ => {
						name.clone().replace('\"', "")
					}
				};

				let icon_type = match weapon_type {
					WeaponType::Cannon => {
						"cannon".to_owned()
					}
					WeaponType::Countermeasures => {
						"heli_false_thermal_targets".to_owned()
					}
					_ => {
						parameter_to_data(preset, "iconType").unwrap_or_else(|| {
							println!("{}", name);
							"".to_owned()
						}).replace('\"', "")
					}
				};

				let split_on_depend = preset.split("DependentWeaponPreset").collect::<Vec<&str>>();
				let depend_on = if split_on_depend.len() > 1 {
					let target = split_on_depend[1];
					Some(Dependent {
						name: parameter_to_data(target, "preset").unwrap().replace('\"', ""),
						slot: parameter_to_data(target, "slot").unwrap().parse().unwrap(),
					})
				} else {
					None
				};

				weapons.push(Weapon {
					localized,
					icon_type,
					count,
					individual_mass: mass,
					name,
					total_mass: weight,
					weapon_type,
					depend_on,
				});
			}

			let pylon = Pylon { index, tier, order, exempt_from_imbalance, weapons };

			if tier.is_none() {
				misc_pylons.push(pylon);
			} else {
				pylons.push(pylon);
			}
		}


		Self {
			localized: name_to_local(&name, &Lang::Unit),
			aircraft: name,
			pylons,
			misc_pylons,
			max_load,
			max_imbalance,
			max_wing_load,
		}
	}

	fn sort(items: &mut Vec<Self>) where Self: Sized {
		items.sort_by_key(|d| d.aircraft.clone());
	}
}

pub fn get_container_weight(base_container: &str, mass: &mut f64, count: &mut u32, projectile_name: &mut String) {
	let container = fs::read_to_string(wt_blk_to_actual(base_container)).unwrap();

	if let Some(mass_str) = parameter_to_data(&container, "mass") {
		if *count == 0 {
			*count = 1;
		}
		*mass = mass_str.parse::<f64>().unwrap();
		*projectile_name = base_container.split('.').next().unwrap().split('/').last().unwrap().to_owned();
	} else {
		// Dummies are marked as containers yet they dont contain anything
		if base_container.contains("dummy_weapon") {
			return;
		}
		let param_bullets = parameter_to_data(&container, "bullets").expect(&container).parse::<u32>().unwrap();
		if *count == 0 {
			*count = param_bullets;
		} else {
			*count *= param_bullets;
		}
		let blk_path = parameter_to_data(&container, "blk").unwrap();

		get_container_weight(&blk_path, mass, count, projectile_name);
	};
}

pub fn wt_blk_to_actual(raw: &str) -> String {
	let clean_path = raw.replace('\"', "");
	let mut split = clean_path.split('/').collect::<Vec<&str>>();
	split.remove(0);
	format!("custom_loadouts/{}", split.join("/").to_ascii_lowercase() + "x")
}
