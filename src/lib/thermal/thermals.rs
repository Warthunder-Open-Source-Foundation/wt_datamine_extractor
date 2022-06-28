use get_size::GetSize;

use crate::extraction_traits::core::ExtractCore;
use crate::lang::{Lang, name_to_local};
use crate::util::get_sep;

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
		let start_sight = keyword_split.last().unwrap();

		let start_of_resolution = start_sight.split("resolution").collect::<Vec<&str>>();

		let split_by_newline = start_of_resolution.last().unwrap().split(&get_sep(file)).collect::<Vec<&str>>();

		let raw_x = split_by_newline[1];
		let raw_y = split_by_newline[2];

		let split_x = raw_x.split(',').collect::<Vec<&str>>();
		let x: f64 = split_x.first().unwrap().trim().parse().unwrap();

		let split_y = raw_y.split(',').collect::<Vec<&str>>();
		let y: f64 = split_y.first().unwrap().trim().parse().unwrap();
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