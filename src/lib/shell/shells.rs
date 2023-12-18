use std::collections::HashSet;
use std::fs;
use std::ops::Not;
use std::str::FromStr;

use get_size::GetSize;
use strum_macros::EnumIter;

use crate::explosive::explosive::explosive_type_to_tnt;
use crate::lang::{Lang, name_to_local};
use crate::shell::error::ShellError;
use crate::shell::explosive::{Explosive, ExplosiveType};
use crate::shell::known_shells::KnownShells;
use crate::shell::penetration_select::shell_to_penetration;
use crate::util::parameter_to_data;

#[derive(serde::Serialize, Clone, serde::Deserialize, Debug, PartialEq, Hash, Eq, const_gen::CompileConst, get_size::GetSize)]
pub struct Shell {
	/// Metadata
	pub name: String,
	pub localized: String,

	pub shell_type: ShellType,

	// in mm
	pub caliber: u32,
	pub true_caliber: u32,

	// in m/s
	pub velocity: u32,

	// in mm
	pub penetration: Vec<(u32, u32)>,

	// 1st is type, 2nd is raw mass, 3rd is TNT equivalent mass
	pub explosive: ExplosiveType,
}

impl Shell {
	pub fn new_from_file(file: &[u8]) -> Vec<Self> {
		let file = String::from_utf8(file.to_vec()).unwrap();
		let mut shells: Vec<Self> = vec![];

		let bullets = file.split("\"bullet\"").clone().collect::<Vec<&str>>();
		for bullet in bullets {
			let name: String = if let Some(file_name) = parameter_to_data(bullet, "bulletName") {
				file_name.trim().replace('\"', "")
			} else {
				continue;
			};

			let caliber = (f64::from_str(&parameter_to_data(bullet, "caliber").unwrap()).unwrap() * 1000.0).round() as u32;

			let true_caliber = parameter_to_data(bullet, "damageCaliber").map_or(caliber, |true_caliber| (f64::from_str(&true_caliber).unwrap() * 1000.0).round() as u32);

			let velocity = f64::from_str(&parameter_to_data(bullet, "speed").unwrap_or_else(|| "0".to_owned())).expect(&name).round() as u32;

			// Shells can sometimes fail to resolve and therefore require manual checking
			// Dumb edge case
			// TODO https://github.com/Warthunder-Open-Source-Foundation/wt_datamine_extractor/issues/65
			let mut pre_type = parameter_to_data(bullet, "bulletType").unwrap();
			if name == "152mm_mim146" {
				pre_type = "\"atgm_tank\"".to_owned();
			}

			let mut shell_type = match ShellType::from_str(&pre_type) {
				Ok(s) => { s }
				Err(e) => {
					match e {
						ShellError::UnknownType(u) => { panic!("Unknown shell type {u}") }
						ShellError::NonDeterministic(_) => {
							if bullet.contains("explosiveType") && bullet.contains("explosiveMass") {
								ShellType::ApHe
							} else {
								ShellType::ApSolid
							}
						}
					}
				}
			};

			let explosive = if !shell_type.is_inert() {
				get_shell_type(&bullet, &name, shell_type)
			} else {
				ExplosiveType::Inert
			};

			let penetration: Vec<(u32, u32)> = shell_to_penetration(bullet);

			shells.push(
				Self {
					localized: name_to_local(&name, &Lang::Weapon),
					name,
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
		values.sort_by_key(|x| format!("{:?}", x));
		fs::write("shell_index/all.json", serde_json::to_string_pretty(&values).unwrap()).unwrap();
		values
	}

	pub fn generate_from_index(index: &KnownShells) -> Vec<Self> {
		let mut generated: Vec<Self> = vec![];
		for i in &index.path {
			if let Ok(file) = fs::read(format!("shell_index/shells/{}", i)) {
				let shells = Shell::new_from_file(&file);

				for shell in shells {
					generated.push(shell);
				}
			}
		}

		// Eliminates duplicates, runs before the following block to prevent unnecessary iteration of full dupes
		let mut set: HashSet<Shell> = HashSet::new();
		for shell in &generated {
			set.insert(shell.clone());
		}
		generated = set.into_iter().collect();

		generated
	}

	pub fn select_by_name(shells: &[Self], name: &str) -> Option<Self> {
		for (i, missile) in shells.iter().enumerate() {
			if missile.name.contains(&name.replace('-', "_")) {
				return Some(shells[i].clone());
			}
		}
		None
	}
}

pub fn get_shell_type(bullet: &str, name: &str, shell_type: ShellType) -> ExplosiveType {
	let mut explosive_type = parameter_to_data(bullet, "explosiveType").map_or_else(|| "".to_owned(), |value| value.trim().replace('\\', "").replace('\"', ""));
	let raw_mass: f64 = parameter_to_data(bullet, "explosiveMass").as_ref().map_or(0.0, |mass| (f64::from_str(mass).unwrap() * 1000.0).round());

	/// Begin edge-case-catching
	match name {
		"125mm_hj_73" | "125mm_hj_73e" => {
			explosive_type = "a_ix_1".to_owned();
		}
		"sonicWave" => {
			return ExplosiveType::Inert;
		}
		"114mm_m8" => {
			explosive_type = "tnt".to_owned();
		}
		"space_rocket" => {
			return ExplosiveType::Inert;
		}
		"40mm_m822" => {
			explosive_type = "octol".to_owned();
		}
		_ => {}
	}
	/// End edge-case-catching

	if explosive_type == "" {
		dbg!(bullet);
		panic!("No Explosive type! {}, {:?}", name, shell_type)
	}
	ExplosiveType::Energetic(
		Explosive {
			name_type: explosive_type.clone(),
			raw_mass: raw_mass as u32,
			equiv_mass: explosive_type_to_tnt(&explosive_type, raw_mass) as u32,
		}
	)
}

#[derive(serde::Serialize, Copy, Clone, serde::Deserialize, Debug, PartialEq, EnumIter, Hash, Eq, const_gen::CompileConst, get_size::GetSize)]
pub enum ShellType {
	ApFsDs = 0,
	HeatFs = 1,
	He = 2,
	ApHe = 3,
	Smoke = 4,
	Apds = 5,
	Atgm = 6,
	Hesh = 7,
	Heat = 8,
	Practice = 9,
	SapHei = 10,
	ApCr = 11,
	ApSolid = 12,
	Sam = 13,
	Rocket = 14,
	AtgmHe = 15,
	Football = 16,
	Shrapnel = 17,
	Aam = 18,
	SonicWave = 19,
	Ahead = 20,
	Napalm = 21,
}

impl ToString for ShellType {
	fn to_string(&self) -> String {
		format!("{:?}", self)
	}
}

impl FromStr for ShellType {
	type Err = ShellError;

	#[allow(clippy::too_many_lines)]
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			r#""apds_fs_long_tank""# |
			r#""apds_fs_tank""# |
			r#""apds_fs_tungsten_small_core_tank""# |
			r#""apds_fs_tungsten_l10_l15_tank""# |
			r#""apds_fs_full_body_steel_tank""# |
			r#""apds_fs_long_l30_tank""# |
			r#""apds_early_tank""# |
			r#""atgm_ke_tank""# |
			r#""apds_fs_tungsten_caliber_fins_tank""# => {
				Ok(Self::ApFsDs)
			}
			r#""heat_fs_tank""# |
			r#""heat_mp_vt_tank""# => {
				Ok(Self::HeatFs)
			}
			r#""he_frag""# |
			r#""he_frag_tank""# |
			r#""he_frag_dist_fuse""# |
			r#""he_frag_radio_fuse""# |
			r#""he_frag_fs_tank""# |
			r#""he_i_t""# |
			r#""he_frag_i""# |
			r#""frag_i_t""# |
			r#""he_dp""# |
			r#""he_frag_t""# |
			r#""he_frag_vog""# |
			r#""he_i""# |
			r#""he_frag_t_ball""# |
			r#""he_i_t_mine""# |
			r#""he_grenade_tank""#|
			r#""he_tf""# |
			r#""grenade_mp_underbarrel_launcher""# => {
				Ok(Self::He)
			}
			r#""aphe_tank""# |
			r#""aphebc_tank""# |
			r#""sapcbc_tank""# |
			r#""sapbc_flat_nose_tank""# |
			r#""ac_shell_tank""# |
			r#""sapi""# |
			r#""aphe""# => {
				Ok(Self::ApHe)
			}
			r#""smoke_tank""# |
			r#""smoke_grenade_tank""# => {
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
			r#""heat_tank""# |
			r#""heat_grenade_tank""# |
			// This from april fools, its basically an RPG but not?!
			r#""heat_fs_rocket""# => {
				Ok(Self::Heat)
			}
			r#""practice_tank""# |
			r#""tp_tank""# |
			r#""tphv_tank""# => {
				Ok(Self::Practice)
			}
			r#""sap_hei_tank""# |
			r#""sap_tank""# => {
				Ok(Self::SapHei)
			}
			r#""apcr_tank""# |
			r#""apcr""# |
			r#""apcr_t""# |
			r#""apcr_i_ball_bs41""# => {
				Ok(Self::ApCr)
			}
			r#""apcbc_solid_medium_caliber_tank""# |
			r#""apbc_tank""# |
			r#""ap_i_t_ball""# |
			r#""he_i_ball""# |
			r#""apcr_i_ball""# |
			r#""ap_i_ball""# |
			r#""ap_ball""# |
			r#""ap_t_ball""# |
			r#""t_ball""# |
			r#""ap_i_t_ball_M20""# |
			r#""i_ball_M1""# |
			r#""ap_ball_M2""# |
			r#""ap_i_ball_M8""# |
			r#""ap_t""# |
			r#""ap_i""# |
			r#""ap_tank""# |
			r#""apc_solid_medium_caliber_tank""# |
			r#""apc_t""# |
			r#""he_ball""# |
			r#""cannon_ball""# => {
				Ok(Self::ApSolid)
			}
			r#""sam_tank""# |
			r#""atgm_vt_fuze_tank""# => {
				Ok(Self::Sam)
			}
			r#""rocket_tank""# => {
				Ok(Self::Rocket)
			}
			r#""atgm_he_tank""# => {
				Ok(Self::AtgmHe)
			}
			r#""football_kick""# |
			r#""football_jump""# => {
				Ok(Self::Football)
			}
			r#""shrapnel_tank""# => {
				Ok(Self::Shrapnel)
			}
			r#""aam""# => {
				Ok(Self::Aam)
			}
			r#""sonicWave""# => {
				Ok(Self::SonicWave)
			}
			r#""ahead_tank""# => {
				Ok(Self::Ahead)
			}
			r#""napalm_tank""# => {
				Ok(Self::Napalm)
			}
			// This is an edge-case, apcbc can both resolve to APHE or solid AP
			r#""apbc_usa_tank""# |
			r#""apc_tank""# |
			r#""apcbc_tank""# |
			// Seems HE frag has such a funny definition that the HE part sometimes is forgotten *oops*
			r#""he_frag_i_t""# |
			// Same with API-T
			r#""ap_i_t""# => {
				Err(ShellError::NonDeterministic(s.to_owned()))
			}
			_ => {
				Err(ShellError::UnknownType(s.to_owned()))
			}
		}
	}
}

impl ShellType {
	// Catches extra edge cases
	pub fn is_inert(&self) -> bool {
		match self {
			Self::ApCr |
			Self::ApSolid |
			Self::ApFsDs |
			Self::Apds |
			Self::Practice |
			Self::Football |
			Self::SonicWave |
			Self::Napalm |
			Self::Smoke // Technically smoke does have some explosive, but it is so incredibly insignificant that we will not count it
			=> true,
			_ => false
		}
	}
}