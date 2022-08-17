use std::str::FromStr;
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
		let crew = match keyword {
			"gunnerThermal" => Crew::Gunner,
			"commanderViewThermal" => Crew::Commander,
			_ => Crew::Global
		};

		// Splits file at the start of the wanted thermal sight
		let keyword_split = &file.split(keyword).collect::<Vec<&str>>();
		// Get the start of the sight
		let start_sight = keyword_split.get(1).unwrap();

		let start_res = start_sight.split("[").collect::<Vec<&str>>()[1].to_string();
		let end_res = start_res.split("]").collect::<Vec<&str>>().first().unwrap().to_string();

		let x_y_split = end_res.split(",").collect::<Vec<&str>>();

		let split_n_clean= |idx: usize| f64::from_str(x_y_split[idx].trim()).unwrap();

		let x = split_n_clean(0);
		let y = split_n_clean(1);

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