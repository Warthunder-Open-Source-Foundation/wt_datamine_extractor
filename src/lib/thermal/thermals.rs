use std::fs;

const PATH: &str = "resources/cache/War-Thunder-Datamine-master/aces.vromfs.bin_u/gamedata/units/tankmodels";

pub struct Thermal {
	pub name: String,
	pub vehicle_type: VehicleType,
	pub sights: Vec<Sight>
}

impl Thermal {
	pub fn generate_raw_missile(path: &str) {

	}
}

pub struct Sight {
	pub crew: Crew,
	pub x: String,
	pub y: String,
}

pub enum Crew {
	Global = 0,
	Gunner = 1,
	Commander = 2,
	Driver = 3,
}

pub enum VehicleType {
	Tank = 0,
	Helicopter = 1,
	Aircraft = 2,
}