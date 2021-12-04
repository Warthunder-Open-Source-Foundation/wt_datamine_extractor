use std::fs;
use std::str::FromStr;

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
	pub caliber: u32,
	// in mm
	pub true_caliber: u32,
	// in mm
	pub velocity: u32,
	// in m/s
	pub penetration: Vec<(u32, u32)>,
	// left is range, right is value
	pub explosive: (String, f64),
}

impl Shell {
	pub fn new_from_file(file: &[u8], parent_gun: &str) -> Vec<Self> {
		let file = String::from_utf8(file.to_vec()).unwrap();
		let mut shells: Vec<Self> = vec![];

		let bullets = file.split("\"bullet\"").clone().collect::<Vec<&str>>();
		for bullet in bullets {
			let name: String = if let Some(file_name) = parameter_to_data(bullet, "bulletName") {
				file_name.trim().replace("\"", "")
			} else {
				continue;
			};

			let shell_type = ShellType::from_str(&parameter_to_data(bullet, "bulletType").unwrap()).unwrap();

			let caliber = (f64::from_str(&parameter_to_data(bullet, "caliber").unwrap()).unwrap() * 1000.0).round() as u32;

			let true_caliber = parameter_to_data(bullet, "damageCaliber").map_or(caliber, |true_caliber| (f64::from_str(&true_caliber).unwrap() * 1000.0).round() as u32);

			let velocity = f64::from_str(&parameter_to_data(bullet, "speed").unwrap_or_else(|| "0".to_owned())).expect(&name).round() as u32;

			let penetration: Vec<(u32, u32)> = shell_to_penetration(bullet, &shell_type);

			let explosive: (String, f64) = match shell_type {
				ShellType::ApFsDs | ShellType::Apds | ShellType::Smoke | ShellType::Practice => {
					(
						"".to_owned(),
						0.0
					)
				}
				ShellType::He | ShellType::HeatFs | ShellType::Apcbc | ShellType::Atgm | ShellType::Hesh | ShellType::Heat | ShellType::SapHei => {
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
					parent_gun: parent_gun.to_owned(),
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

				let shells = Shell::new_from_file(&file, &name);

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
	ApFsDs = 0,
	HeatFs = 1,
	He = 2,
	Apcbc = 3,
	Smoke = 4,
	Apds = 5,
	Atgm = 6,
	Hesh = 7,
	Heat = 8,
	Practice = 9,
	SapHei = 10,
}

impl FromStr for ShellType {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			r#""apds_fs_long_tank""# |
			r#""apds_fs_tank""# |
			r#""apds_fs_tungsten_small_core_tank""# |
			r#""apds_fs_tungsten_l10_l15_tank""# |
			r#""apds_fs_full_body_steel_tank""# => {
				Ok(Self::ApFsDs)
			}
			r#""heat_fs_tank""# => {
				Ok(Self::HeatFs)
			}
			r#""he_frag""# |
			r#""he_frag_tank""# |
			r#""he_frag_dist_fuse""# |
			r#""he_frag_radio_fuse""# |
			r#""he_frag_fs_tank""# |
			r#""he_i_t""# => {
				Ok(Self::He)
			}
			r#""apcbc_tank""# => {
				Ok(Self::Apcbc)
			}
			r#""smoke_tank""# => {
				Ok(Self::Smoke)
			}
			r#""apds_tank""# |
			r#""apds_l15_tank""# |
			r#""apds_autocannon""# => {
				Ok(Self::Apds)
			}
			r#""atgm_tank""# |
			r#""atgm_tandem_tank""# => {
				Ok(Self::Atgm)
			}
			r#""hesh_tank""# => {
				Ok(Self::Hesh)
			}
			r#""heat_tank""# => {
				Ok(Self::Heat)
			}
			r#""practice_tank""# => {
				Ok(Self::Practice)
			}
			r#""sap_hei_tank""# => {
				Ok(Self::SapHei)
			}
			_ => { panic!("Cannot determine shell type {}", s) }
		}
	}
}