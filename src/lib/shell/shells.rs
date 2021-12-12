use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use strum_macros::EnumIter;

use crate::explosive::explosive::explosive_type_to_tnt;
use crate::lang::{Lang, unit_to_local};
use crate::shell::known_shells::KnownShells;
use crate::shell::parent_gun::ParentGun;
use crate::shell::penetration_select::shell_to_penetration;
use crate::util::parameter_to_data;

#[derive(serde::Serialize, Clone, serde::Deserialize, Debug, PartialEq, Hash, Eq)]
pub struct Shell {
	/// Metadata
	pub name: String,
	pub localized: String,
	pub parent_guns: Vec<ParentGun>,
	pub hash: u64,

	pub shell_type: ShellType,

	// in mm
	pub caliber: u32,
	pub true_caliber: u32,

	// in m/s
	pub velocity: u32,

	// in mm
	pub penetration: Vec<(u32, u32)>,

	// 1st is type, 2nd is raw mass, 3rd is TNT equivalent mass
	pub explosive: (String, u32, u32),
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

			let caliber = (f64::from_str(&parameter_to_data(bullet, "caliber").unwrap()).unwrap() * 1000.0).round() as u32;

			let true_caliber = parameter_to_data(bullet, "damageCaliber").map_or(caliber, |true_caliber| (f64::from_str(&true_caliber).unwrap() * 1000.0).round() as u32);

			let velocity = f64::from_str(&parameter_to_data(bullet, "speed").unwrap_or_else(|| "0".to_owned())).expect(&name).round() as u32;

			let explosive: (String, u32, u32) = {
				let explosive_type = parameter_to_data(bullet, "explosiveType").map_or_else(|| "".to_owned(), |value| value.trim().replace("\\", "").replace("\"", ""));
				let raw_mass = parameter_to_data(bullet, "explosiveMass").as_ref().map_or(0, |mass| (f64::from_str(mass).unwrap() * 1000.0).round() as u32);
				(
					explosive_type.clone(),
					raw_mass,
					explosive_type_to_tnt(&explosive_type, raw_mass)
				)
			};

			// Shells can sometimes fail to resolve and therefore require manual checking
			let shell_type = if let Ok(result) = ShellType::from_str(&parameter_to_data(bullet, "bulletType").unwrap()) {
				result
			} else if explosive.0.is_empty() {
				ShellType::ApSolid
			} else {
				ShellType::ApHe
			};

			let penetration: Vec<(u32, u32)> = shell_to_penetration(bullet);

			let parent_guns = [ParentGun { name: parent_gun.to_owned(), localized: unit_to_local(parent_gun, &Lang::Weapon) }].to_vec();

			let mut hasher = DefaultHasher::new();
			bullet.hash(&mut hasher);

			shells.push(
				Self {
					parent_guns,
					localized: unit_to_local(&name, &Lang::Weapon),
					name,
					shell_type,
					caliber,
					true_caliber,
					velocity,
					penetration,
					explosive,
					hash: hasher.finish(), // Remains 0 until write-instancing
				}
			);
		}
		shells
	}

	pub fn write_all(mut values: Vec<Self>) -> Vec<Self> {
		values.sort_by_key(|d| d.hash);
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

		// Eliminates duplicates, runs before the following block to prevent unnecessary iteration of full dupes
		let mut set: HashSet<Shell> = HashSet::new();
		for shell in &generated {
			set.insert(shell.clone());
		}
		generated = set.into_iter().collect();


		// We will separate all parent guns from the shell
		// Then we will get the shell before inserting it, weather or not if that fails, we will choose to push the gun on the identical shell
		// Alternatively we will keep the entry and insert it anyways

		let mut map: HashMap<Shell, Vec<ParentGun>> = HashMap::new();

		// This needs checking for every shell
		for shell in &generated {

			// The seperated parent shell is needed to compare the new shells
			let parents = &shell.parent_guns;

			// New shells for mutation
			let mut new_shell = shell.clone();

			// Remove parent to equalize shells
			new_shell.parent_guns.clear();

			// Attempt to get shell
			if let Some(hit) = map.get(&new_shell) {

				// When a shell is found, it will be added to the new parents
				let mut new_parents: Vec<ParentGun> = hit.clone();
				for parent in parents {
					new_parents.push(parent.clone());
				}
				new_parents.sort_by_key(|x| x.name.clone());
				map.insert(new_shell.clone(), new_parents.clone());
			} else {
				map.insert(new_shell.clone(), parents.clone());
			}
		}

		let gen_map = map.into_iter().collect::<Vec<(Shell, Vec<ParentGun>)>>();

		let mut new_generated = vec![];

		for item in gen_map {
			let mut new_shell = item.0.clone();
			new_shell.parent_guns = item.1;
			new_generated.push(new_shell);
		}

		new_generated
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

#[derive(serde::Serialize, Clone, serde::Deserialize, Debug, PartialEq, EnumIter, Hash, Eq)]
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
}

impl ToString for ShellType {
	fn to_string(&self) -> String {
		format!("{:?}", self)
	}
}

impl FromStr for ShellType {
	type Err = String;

	#[allow(clippy::too_many_lines)]
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
			r#""he_i_t""# |
			r#""he_frag_i""# |
			r#""frag_i_t""# |
			r#""he_dp""# |
			r#""he_frag_t""# |
			r#""he_frag_vog""# |
			r#""he_i""# |
			r#""he_i_t_mine""# |
			r#""he_grenade_tank""# => {
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
			r#""heat_tank""# | r#""heat_grenade_tank""# => {
				Ok(Self::Heat)
			}
			r#""practice_tank""# => {
				Ok(Self::Practice)
			}
			r#""sap_hei_tank""# => {
				Ok(Self::SapHei)
			}
			r#""apcr_tank""# |
			r#""apcr_t""# => {
				Ok(Self::ApCr)
			}
			r#""apcbc_solid_medium_caliber_tank""# |
			r#""apbc_tank""# |
			r#""ap_i_t_ball""# |
			r#""he_i_ball""# |
			r#""apcr_i_ball""# |
			r#""ap_i_ball""# |
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
			// This is an edge-case, apcbc can both resolve to APHE or solid AP
			r#""apbc_usa_tank""# |
			r#""apc_tank""# |
			r#""apcbc_tank""# |
			// Seems HE frag has such a funny definition that the HE part sometimes is forgotten *oops*
			r#""he_frag_i_t""# |
			// Same with API-T
			r#""ap_i_t""# => {
				Err("Failed to resolve shell type from direct parameter".to_owned())
			}
			_ => { panic!("Cannot determine shell type {}", s) }
		}
	}
}