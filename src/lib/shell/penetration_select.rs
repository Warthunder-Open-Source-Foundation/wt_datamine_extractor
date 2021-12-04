use std::str::FromStr;
use crate::shell::shells::ShellType;
use crate::util::parameter_to_data;

pub fn shell_to_penetration(shell: &str, shell_type: &ShellType) -> [u32; 9] {
	// X axis represents ranges from 0, 100, 500, 1000, 1500, 2000, 3000, 10000 and 20000
	let mut penetration = [0, 0, 0, 0, 0, 0, 0, 0, 0];
	match shell_type {
		ShellType::APFSDS => {
			penetration[0] = f64::from_str(&parameter_to_data(shell, "ArmorPower0m").unwrap().split(".").collect::<Vec<&str>>()[0].replace("[", "")).unwrap().round() as u32;
			penetration[1] = f64::from_str(&parameter_to_data(shell, "ArmorPower100m").unwrap().split(".").collect::<Vec<&str>>()[0].replace("[", "")).unwrap().round() as u32;
			penetration[2] = f64::from_str(&parameter_to_data(shell, "ArmorPower500m").unwrap().split(".").collect::<Vec<&str>>()[0].replace("[", "")).unwrap().round() as u32;
			penetration[3] = f64::from_str(&parameter_to_data(shell, "ArmorPower1000m").unwrap().split(".").collect::<Vec<&str>>()[0].replace("[", "")).unwrap().round() as u32;
			penetration[4] = f64::from_str(&parameter_to_data(shell, "ArmorPower1500m").unwrap().split(".").collect::<Vec<&str>>()[0].replace("[", "")).unwrap().round() as u32;
			penetration[5] = f64::from_str(&parameter_to_data(shell, "ArmorPower2000m").unwrap().split(".").collect::<Vec<&str>>()[0].replace("[", "")).unwrap().round() as u32;
			penetration[6] = f64::from_str(&parameter_to_data(shell, "ArmorPower3000m").unwrap().split(".").collect::<Vec<&str>>()[0].replace("[", "")).unwrap().round() as u32;
			penetration[7] = f64::from_str(&parameter_to_data(shell, "ArmorPower10000m").unwrap().split(".").collect::<Vec<&str>>()[0].replace("[", "")).unwrap().round() as u32;
			penetration[8] = f64::from_str(&parameter_to_data(shell, "ArmorPower20000m").unwrap().split(".").collect::<Vec<&str>>()[0].replace("[", "")).unwrap().round() as u32;
		}
		ShellType::HEATFS => {
			let pen = &parameter_to_data(shell, "armorPower").unwrap();
			let pen_32 = f64::from_str(pen.trim()).unwrap().round() as u32;
			for i in 0..penetration.len() {
				penetration[i] = pen_32;
			}
		}
		ShellType::HEFS => {
			// They dont have penetration values in the files it seems
		}
	};
	penetration
}