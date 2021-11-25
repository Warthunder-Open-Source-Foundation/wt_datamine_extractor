use std::fs;

use crate::thermal::extract_thermals::KnownThermals;
use crate::util::parameter_to_data;

const PATH: &str = "resources/cache/War-Thunder-Datamine-master/aces.vromfs.bin_u/gamedata/units/tankmodels";

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Thermal {
	pub name: String,
	pub vehicle_type: VehicleType,
	pub sights: Vec<Sight>,
}

impl Thermal {
	pub fn generate_from_index(index: &KnownThermals) -> Vec<Self> {
		let mut generated: Vec<Self> = vec![];
		for i in &index.path {
			if let Ok(file) = fs::read_to_string(format!("thermal_index/thermals/{}", i)) {
				let vehicle_type = if file.contains("FM") {
					if file.contains("helicopter") {
						VehicleType::Aircraft
					} else {
						VehicleType::Helicopter
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

				let mut name: String = parameter_to_data(&file, "model").unwrap().trim().to_string();
				name  = name[1..name.len() - 1].to_string();

				if sights.len() == 0 {
					panic!(format!("Missing sight on {}", name))
				}
				generated.push(Self {
					name,
					vehicle_type,
					sights,
				})
			}
		}
		generated
	}
}

pub fn write_all(values: &Vec<Thermal>) {
	fs::write("thermal_index/all.json", serde_json::to_string_pretty(&values).unwrap()).unwrap();
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

		let part = &file.split(keyword).collect::<Vec<&str>>()[1].split("\n").collect::<Vec<&str>>()[1];
		let array = part.split("[").collect::<Vec<&str>>()[1].split("]").collect::<Vec<&str>>()[0];
		let x: f64 = array.split(",").collect::<Vec<&str>>()[0].parse().unwrap();
		let y: f64 = array.split(",").collect::<Vec<&str>>()[1].parse().unwrap();
		Sight {
			crew,
			x,
			y,
		}
	}
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Sight {
	pub crew: Crew,
	pub x: f64,
	pub y: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum Crew {
	Global = 0,
	Gunner = 1,
	Commander = 2,
	Driver = 3,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum VehicleType {
	Tank = 0,
	Helicopter = 1,
	Aircraft = 2,
}