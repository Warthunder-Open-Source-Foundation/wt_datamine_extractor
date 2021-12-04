use std::str::FromStr;
use crate::shell::shells::{ShellType};
use crate::util::parameter_to_data;

pub fn shell_to_penetration(shell: &str, shell_type: &ShellType) -> Vec<(u32, u32)> {
	// X axis represents ranges from 0, 100, 500, 1000, 1500, 2000, 3000, 10000 and 20000
	let mut penetration: Vec<(u32, u32)> = vec![];
	#[allow(clippy::match_same_arms)]
	match shell_type {
		ShellType::ApFsDs => {
			for range in 0..5000/100 {
				if let Some(param) = &parameter_to_data(shell, &format!("ArmorPower{}m", range * 100)) {
					let param_64 = f64::from_str(&param.split('.').collect::<Vec<&str>>()[0].replace("[", "")).unwrap();
					penetration.push((range * 100, param_64.round() as u32));
				}
			}
		}
		ShellType::HeatFs | ShellType::Atgm | ShellType::Hesh | ShellType::Heat => {
			let pen = &parameter_to_data(shell, "armorPower").unwrap();
			let pen_32 = f64::from_str(pen.trim()).unwrap().round() as u32;
			penetration.push((0, pen_32));
		}
		ShellType::He | ShellType::Smoke => {
			// They dont have penetration values in the files it seems
		}
		ShellType::Apcbc | ShellType::Apds | ShellType::Practice | ShellType::SapHei => {
			// TODO It uses demarre that i have yet to calculate
		}
	};
	penetration
}