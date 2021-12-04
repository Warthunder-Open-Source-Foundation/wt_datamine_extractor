use std::{fs};
use std::str::FromStr;

use crate::lang::unit_to_local;
use crate::shell::known_shells::KnownShells;
use crate::shell::penetration_select::shell_to_penetration;
use crate::util::parameter_to_data;

#[derive(serde::Serialize, Clone, serde::Deserialize, Debug, PartialEq)]
pub struct Shell {
	/// Metadata
	pub name: String,
	pub localized: String,
	pub parent_gun: String,

	pub shell_type: ShellType,
	pub caliber: u32, // in mm
	pub true_caliber: u32, // in mm
	pub velocity: u32, // in m/s
	pub penetration: [u32; 9], // X axis represents ranges from 0, 100, 500, 1000, 1500, 2000, 3000, 10000 and 20000
	pub explosive: (String, f64),
}

impl Shell {
	pub fn new_from_file(file: &[u8], parent_gun: String) -> Vec<Self> {
		let file = String::from_utf8(file.to_vec()).unwrap();
		let mut shells: Vec<Self> = vec![];

		let bullets = file.split("\"bullet\"").to_owned().collect::<Vec<&str>>();
		for bullet in bullets {
			let name: String = if let Some(file_name) = parameter_to_data(bullet, "bulletName") {
				file_name.trim().replace("\"", "")
			} else {
				continue
			};

			let shell_type = ShellType::from_str(&parameter_to_data(bullet, "bulletType").unwrap()).unwrap();

			let caliber = (f64::from_str(&parameter_to_data(bullet, "caliber").unwrap()).unwrap() * 1000.0).round() as u32;
			let true_caliber = if let Some(true_caliber) = parameter_to_data(bullet, "damageCaliber") {
				(f64::from_str(&true_caliber).unwrap() * 1000.0).round() as u32
			} else {
				caliber
			};

			let velocity = f64::from_str(&parameter_to_data(bullet, "speed").unwrap()).unwrap().round() as u32;

			let mut penetration: [u32; 9] = shell_to_penetration(bullet, &shell_type);

			let explosive: (String, f64) = match shell_type {
				ShellType::APFSDS => {
					(
					"".to_owned(),
						0.0
					)
				}
				ShellType::HEFS | ShellType::HEATFS => {
					(
					parameter_to_data(bullet, "explosiveType").unwrap().trim().replace("\\", "").replace("\"", ""),
					f64::from_str(&parameter_to_data(bullet, "explosiveMass").unwrap()).unwrap()
					)
				}
			};

			shells.push(
				Self {
					name,
					localized: "".to_string(),
					parent_gun: parent_gun.clone(),
					shell_type,
					caliber,
					true_caliber,
					velocity,
					penetration,
					explosive,
				}
			);
		}
		shells
	}

	pub fn write_all(mut values: Vec<Self>) -> Vec<Self> {
		values.sort_by_key(|d| d.name.clone());
		fs::write("shell_index/all.json", serde_json::to_string_pretty(&values).unwrap()).unwrap();
		values
	}

	pub fn generate_from_index(index: &KnownShells) -> Vec<Self> {
		let mut generated: Vec<Self> = vec![];
		for i in &index.path {
			if let Ok(file) = fs::read(format!("shell_index/shells/{}", i)) {
				let name = i.split('.').collect::<Vec<&str>>()[0].to_owned();

				let shells = Shell::new_from_file(&file, name);

				for shell in shells {
					generated.push(shell);
				}
			}
		}
		generated
	}

	pub fn select_by_name(shells: &[Self], name: &str) -> Option<Self> {
		for (i, missile) in shells.iter().enumerate() {
			if missile.name.contains(&name.replace("-", "_")) {
				return Some(shells[i].clone());
			}
		}
		None
	}
}

#[derive(serde::Serialize, Clone, serde::Deserialize, Debug, PartialEq)]
pub enum ShellType {
	APFSDS = 0,
	HEATFS = 1,
	HEFS = 2,
}

impl FromStr for ShellType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			r#""apds_fs_long_tank""# => {
				Ok(Self::APFSDS)
			}
			r#""heat_fs_tank""# => {
				Ok(Self::HEATFS)
			}
			r#""he_frag""# | r#""he_frag_dist_fuse""# => {
				Ok(Self::HEFS)
			}
			_ => { panic!("Cannot determine shell type {}", s) }
		}
	}
}