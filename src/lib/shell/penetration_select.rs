use std::str::FromStr;
use crate::shell::demarre::{DemarreMod, penetration_from_demarre};

use crate::util::parameter_to_data;

pub fn shell_to_penetration(shell: &str) -> Vec<(u32, u32)> {
	// X axis represents ranges from 0, 100, 500, 1000, 1500, 2000, 3000, 10000 and 20000
	let mut penetration: Vec<(u32, u32)> = vec![];
	if shell.contains("cumulativeDamage") {
		let pen = &parameter_to_data(shell, "armorPower").unwrap();
		let pen_32 = f64::from_str(pen.trim()).unwrap().round() as u32;
		penetration.push((0, pen_32));
	} else if shell.contains("ArmorPower0m") {
		for range in 0..5000 / 100 {
			if let Some(param) = &parameter_to_data(shell, &format!("ArmorPower{}m", range * 100)) {
				let param_64 = f64::from_str(&param.split('.').collect::<Vec<&str>>()[0].replace("[", "")).unwrap();
				penetration.push((range * 100, param_64.round() as u32));
			}
		}
	} else if shell.contains("demarre") {
		let speed = f64::from_str(&parameter_to_data(shell, "speed").unwrap()).unwrap();

		let caliber = f64::from_str(
			&if let Some(calib) = parameter_to_data(shell, "damageCaliber") {
				calib
			} else {
				parameter_to_data(shell, "caliber").unwrap()
			}
		).unwrap();

		let mass = f64::from_str(&parameter_to_data(shell, "mass").unwrap()).unwrap();

		penetration.push((0, penetration_from_demarre(speed, caliber, mass, DemarreMod::from_file(shell))))
	}
	penetration
}