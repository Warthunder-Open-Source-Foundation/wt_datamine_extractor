use std::str::FromStr;
use crate::util::parameter_to_data;

struct Reference {
	// in mm
	caliber: f64,
	// in kg
	mass: f64,
	// in m/s
	velocity: f64,
	// in CENTIMETERS not mm!!!
	penetration: f64,
}

const REFERENCE: Reference = {
	Reference {
		caliber: 100.0,
		mass: 1.0,
		velocity: 1900.0,
		penetration: 100.0,
	}
};

pub struct DemarreMod {
	pub penetration_k: f64,
	pub speed_pow: f64,
	pub mass_pow: f64,
	pub caliber_pow: f64,
}

impl DemarreMod {
	pub fn from_default() -> Self {
		// These are the reference keys used by demarre
		Self {
			penetration_k: 1.0,
			speed_pow: 1.4283,
			mass_pow: 0.7143,
			caliber_pow: 1.0714,
		}
	}
	pub fn from_file(file: &str) -> Self {
		Self {
			penetration_k: f64::from_str(&parameter_to_data(file, "demarrePenetrationK").unwrap()).unwrap(),
			speed_pow: f64::from_str(&parameter_to_data(file, "demarreSpeedPow").unwrap()).unwrap(),
			mass_pow: f64::from_str(&parameter_to_data(file, "demarreMassPow").unwrap()).unwrap(),
			caliber_pow: f64::from_str(&parameter_to_data(file, "demarreCaliberPow").unwrap()).unwrap(),
		}
	}
}

pub fn penetration_from_demarre(velocity: f64, caliber: f64, mass: f64, modifiers: &DemarreMod) -> u32 {
	let caliber = caliber * 1000.0;
	// Source: http://www.tankarchives.ca/2014/10/penetration-equations.html
	(REFERENCE.penetration * modifiers.penetration_k * (velocity / REFERENCE.velocity).powf(modifiers.speed_pow) *
		(caliber / REFERENCE.caliber).powf(modifiers.caliber_pow) *
		(mass / caliber.powi(3)).powf(modifiers.mass_pow) /
		(REFERENCE.mass / REFERENCE.caliber.powi(3)).powf(modifiers.mass_pow)).round() as u32
}