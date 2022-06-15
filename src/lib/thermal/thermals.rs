use get_size::GetSize;

use crate::extraction_traits::core::ExtractCore;
use crate::lang::{Lang, name_to_local};

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, get_size::GetSize)]
pub struct Thermal {
	pub name: String,
	pub localized: String,
	pub vehicle_type: VehicleType,
	pub sights: Vec<Sight>,
}

impl ExtractCore for Thermal {
	fn new_from_file(file: &[u8], name: String) -> Self {
		let file = String::from_utf8(file.to_owned()).unwrap();
		let vehicle_type = if file.contains("fmFile") {
			if file.contains("helicopter") {
				VehicleType::Helicopter
			} else {
				VehicleType::Aircraft
			}
		} else {
			VehicleType::Tank
		};

		let mut sights: Vec<Sight> = vec![];

		if file.contains("gunnerThermal") {
			sights.push(Sight::from_file(&file, "gunnerThermal"));
		}

		if file.contains("commanderViewThermal") {
			sights.push(Sight::from_file(&file, "commanderViewThermal"));
		}

		if file.contains("sightTPodThermal") {
			sights.push(Sight::from_file(&file, "sightTPodThermal"));
		}

		if file.contains("sightThermal") {
			sights.push(Sight::from_file(&file, "sightThermal"));
		}

		if sights.is_empty() {
			assert!(!sights.is_empty(), "Missing sight on {}", name);
		}
		Self {
			localized: name_to_local(&name, &Lang::Unit).clone(),
			name,
			vehicle_type,
			sights,
		}
	}

	fn sort(items: &mut Vec<Self>) {
		items.sort_by_key(|x| x.name.clone());
	}
}

impl Sight {
	pub fn from_file(file: &str, keyword: &str) -> Self {
		let crew = if keyword == "gunnerThermal" {
			Crew::Gunner
		} else if keyword == "commanderViewThermal" {
			Crew::Commander
		} else {
			Crew::Global
		};

		let part = &file.split(keyword).collect::<Vec<&str>>()[1].split('\n').collect::<Vec<&str>>()[1];
		let array = part.split('[').collect::<Vec<&str>>()[1].split(']').collect::<Vec<&str>>()[0];
		let x: f64 = array.split(',').collect::<Vec<&str>>()[0].parse().unwrap();
		let y: f64 = array.split(',').collect::<Vec<&str>>()[1].parse().unwrap();
		Sight {
			crew,
			x,
			y,
		}
	}
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, get_size::GetSize)]
pub struct Sight {
	pub crew: Crew,
	pub x: f64,
	pub y: f64,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, get_size::GetSize, Eq)]
pub enum Crew {
	Global = 0,
	Gunner = 1,
	Commander = 2,
	Driver = 3,
}

#[derive(Clone, Copy, serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, get_size::GetSize, Eq)]
pub enum VehicleType {
	Tank = 0,
	Helicopter = 1,
	Aircraft = 2,
}