use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::fs;
use get_size::GetSize;

use crate::explosive::explosive::explosive_type_to_tnt;
use crate::extraction_traits::core::ExtractCore;
use crate::extraction_traits::known::KnownItem;
use crate::lang::{Lang, name_to_local};
use crate::missile::visbands::Visband;
use crate::util::parameter_to_data;

#[allow(clippy::struct_excessive_bools)]
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, const_gen::CompileConst, get_size::GetSize)]
pub struct Missile {
	// metadata that is global or does not exist on files that are generated

	/// associated file-name of the missile
	pub name: String,

	/// english localized name
	pub localized: String,

	/// type of seeker that has to be extracted from the file and represented as enum
	pub seekertype: SeekerType,


	// main data raw values taken from the file

	/// mass in kg
	pub mass: f64,

	/// mass in kg after 1st stage engine burn
	pub mass_end: f64,

	/// mass in kg after 2nd stage engine burn
	pub mass_end1: f64,

	/// diameter in meter
	pub caliber: f64,

	/// first set of thrust in N
	pub force0: f64,

	/// second set of thrust in N
	pub force1: f64,

	/// first set of thrust in s
	pub timefire0: f64,

	/// second set of thrust in s
	pub timefire1: f64,

	/// drag constant coefficient
	pub cxk: f64,

	/// drag constant
	pub dragcx: f64,

	/// absolute time of life after spawning in s
	pub timelife: f64,

	/// hard cap for velocity in m/s
	pub endspeed: f64,

	/// amount of raw explosive material in g
	pub exp_mass: u32,

	/// proximity fuse
	pub pfuse: bool,

	/// maximum G load at launch (assuming G = 9.81m/s²)
	pub loadfactormax: f64,

	/// maximum G during flight (assuming G = 9.81m/s²)
	pub reqaccelmax: f64,

	/// seeker visibility ranges
	pub bands: Option<Visband>,

	/// size of the uncaged part of the seeker in degrees
	pub fov: Option<f64>,

	/// size of the locked seeker center in degrees
	pub gate: Option<f64>,

	/// distance from bore before launch in degrees
	pub lockanglemax: f64,

	/// distance from bore after launch in degrees
	pub anglemax: f64,

	/// distance from sun to be distracted in degrees
	pub minangletosun: Option<f64>,

	/// time missile does not manuever after launch
	pub timeout: f64,

	/// time to switch missile into seek mode in s
	pub warmuptime: f64,

	/// time the missile remains in seek mode in s
	pub worktime: f64,

	/// if the seeker is on a gimbal or not
	pub cageable: bool,

	/// angular velocity the seeker moves at
	pub rate_max: f64,

	/// if the missile applies proportional navigation after losing lock (dead behaviour)
	pub inertial_navigation: bool,

	/// if the missile influences the inertial navigation with target velocity
	pub use_target_vel: bool,

	/// permits the missile to slave after the radar
	pub allow_radar_slave: bool,

	/// permits the missile to link back data to other aircraft
	pub has_data_link: bool,

	/// permits the missile to regain datalink after loss of connection
	pub reconnect_data_link: bool,

	/// permits missile to accurately track without continous targeting data, prevents self destruction when main seeker looses sight
	pub has_inertial_navigation: bool,

	// Calculated (dynamically created and not in files)

	/// total potential energy in m/s²
	pub deltav: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Copy, Clone, const_gen::CompileConst, get_size::GetSize)]
pub enum SeekerType {
	Ir = 0,
	Sarh = 1,
	Arh = 2,
}

impl Display for SeekerType {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			SeekerType::Ir => { write!(f, "{}", "IR") }
			SeekerType::Sarh => { write!(f, "{}", "SARH") }
			SeekerType::Arh => { write!(f, "{}", "ARH") }
		}
	}
}

impl Missile {
	pub fn select_by_name(missiles: &[Self], name: &str) -> Option<Self> {
		for (i, missile) in missiles.iter().enumerate() {
			if missile.name.contains(&name.replace('-', "_")) {
				return Some(missiles[i].clone());
			}
		}
		None
	}
	// Compares ignoring the name only considering localized differences
	pub fn eq_by_values(mut self, mut other: Self) -> bool {
		self.name.clear();
		other.name.clear();
		self == other
	}
}

impl ExtractCore for Missile {
	fn new_from_file(file: &[u8], name: String) -> Self {
		let file = String::from_utf8(file.to_vec()).unwrap();

		let blk = wt_blk::WTBlk::new(&file).unwrap();

		let seekertype = {
			if file.contains("irSeeker") {
				SeekerType::Ir
			} else if file.contains("radarSeeker") {
				if file.contains("\"active\": true,") {
					SeekerType::Arh
				} else {
					SeekerType::Sarh
				}
			} else {
				panic!("Cant identify seeker type")
			}
		};

		let mass = blk.float("/rocket/mass").unwrap();

		let mass_end = blk.float("/rocket/massEnd").unwrap();

		let mass_end1 = blk.float("/rocket/massEnd1").unwrap_or(0.0);

		let caliber = blk.float("/rocket/caliber").unwrap();

		// Add force0 if this starts to crash
		let force0 = blk.float("/rocket/force").unwrap();

		let force1 = blk.float("/rocket/force1").unwrap_or(0.0);

		let timefire0 = blk.float("/rocket/timeFire").unwrap();

		let timefire1 = blk.float("/rocket/timeFire1").unwrap_or(0.0);

		let cxk = blk.float("/rocket/CxK").unwrap();

		let dragcx = blk.float("/rocket/dragCx").unwrap();

		let timelife = blk.float("/rocket/timeLife").unwrap();

		let endspeed = blk.float("/rocket/endSpeed").unwrap();

		let exp_mass = explosive_type_to_tnt(
			&blk.str("/rocket/explosiveType").unwrap().replace('\"', ""),
			(blk.float("/rocket/explosiveMass").unwrap() * 1000.0).round(),
		);

		let pfuse = blk.bool("/rocket/hasProximityFuse").unwrap_or(false);

		let loadfactormax = blk.float("/rocket/loadFactorMax").unwrap();

		let reqaccelmax = blk.float("/rocket/guidance/guidanceAutopilot/reqAccelMax").unwrap_or(0.0);

		let bands = if seekertype == SeekerType::Ir {
			Some(Visband {
				range_band0: blk.float("/rocket/guidance/irSeeker/rangeBand0").unwrap() as usize,
				range_band1: blk.float("/rocket/guidance/irSeeker/rangeBand1").unwrap_or(0.0) as usize,
				range_band2: blk.float("/rocket/guidance/irSeeker/rangeBand2").unwrap() as usize,
				range_band3: blk.float("/rocket/guidance/irSeeker/rangeBand3").unwrap() as usize,
				range_band6: blk.float("/rocket/guidance/irSeeker/rangeBand6").unwrap_or(0.0) as usize,
				range_band7: blk.float("/rocket/guidance/irSeeker/rangeBand7").unwrap() as usize,
				range_max: blk.float("/rocket/guidance/irSeeker/rangeMax").unwrap() as usize,
			})
		} else {
			None
		};

		let fov = if seekertype == SeekerType::Ir {
			Some(blk.float("/rocket/guidance/irSeeker/fov").unwrap())
		} else {
			None
		};

		let gate = blk.float("/rocket/guidance/irSeeker/gateWidth").ok();

		let seeker_name = match seekertype {
			SeekerType::Ir => {
				"irSeeker"
			}
			SeekerType::Sarh => {
				"radarSeeker"
			}
			SeekerType::Arh => {
				"radarSeeker"
			}
		};
		let lockanglemax =  blk.float(&format!("/rocket/guidance/{seeker_name}/lockAngleMax")).unwrap();

		let anglemax =blk.float(&format!("/rocket/guidance/{seeker_name}/angleMax")).unwrap();

		let minangletosun = if seekertype == SeekerType::Ir {
			Some(blk.float(&format!("/rocket/guidance/irSeeker/minAngleToSun")).unwrap())
		} else {
			None
		};

		let timeout = blk.float(&format!("/rocket/guidance/guidanceAutopilot/timeOut")).unwrap_or(0.0);

		let warmuptime = blk.float("/rocket/guidance/warmUpTime").unwrap();

		let worktime = blk.float("/rocket/guidance/workTime").unwrap();

		let cageable = blk.bool("/rocket/guidance/uncageBeforeLaunch").unwrap();

		let rate_max = blk.float(&format!("/rocket/guidance/{seeker_name}/rateMax")).unwrap();

		let inertial_navigation = blk.bool("/rocket/guidance/inertialNavigation").unwrap_or(false);

		let use_target_vel = blk.bool("/rocket/guidance/useTargetVel").unwrap_or(false);

		let allow_radar_slave = blk.float(&format!("/rocket/guidance/{seeker_name}/designationSourceTypeMask")).is_ok();

		let has_data_link = blk.bool(&format!("/rocket/guidance/inertialGuidance/datalink")).unwrap_or(false);

		let reconnect_data_link = blk.bool(&format!("/rocket/guidance/inertialGuidance/reconnectDatalink")).unwrap_or(false);

		let has_inertial_navigation = blk.pointer("/rocket/guidance/inertialGuidance").is_ok();

		Self {
			// localized first as the borrow consumes name otherwise
			localized: name_to_local(&name, &Lang::Weapon),
			name,
			seekertype,
			mass,
			mass_end,
			mass_end1,
			caliber,
			force0,
			force1,
			timefire0,
			timefire1,
			cxk,
			dragcx,
			timelife,
			endspeed,
			// Temporary workaround. TODO refactor to f64
			exp_mass: exp_mass.round() as u32,
			pfuse,
			loadfactormax,
			reqaccelmax,
			bands,
			fov,
			gate,
			lockanglemax,
			anglemax,
			minangletosun,
			timeout,
			warmuptime,
			worktime,
			cageable,
			rate_max,
			inertial_navigation,
			use_target_vel,
			allow_radar_slave,
			has_data_link,
			reconnect_data_link,
			has_inertial_navigation,
			deltav: ((force0 / mass * timefire0) + (force1 / mass * timefire1)).round(),
		}
	}

	fn sort(items: &mut Vec<Self>) where Self: Sized {
		items.sort_by_key(|x| x.name.clone());
	}

	fn generate_from_index(index: impl KnownItem, write_path: &str) -> Vec<Self> {
		let mut generated: Vec<Self> = vec![];
		for i in index.get_index() {
			if let Ok(file) = fs::read(format!("{write_path}{i}")) {
				let name = i.split('.').collect::<Vec<&str>>()[0].to_owned();

				let missile = Self::new_from_file(&file, name);

				generated.push(missile);
			}
		}

		let mut dedup: Vec<Self> = vec![];

		for i in generated {
			let mut is_contained = false;
			for j in &dedup {
				if j.clone().eq_by_values(i.clone()) {
					is_contained = true;
				}
			}

			if !is_contained {
				dedup.push(i);
			}
		}

		Self::sort(&mut dedup);
		dedup
	}
}