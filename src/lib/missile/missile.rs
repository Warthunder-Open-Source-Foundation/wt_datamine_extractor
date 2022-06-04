use std::fs;

use get_size::GetSize;

use crate::explosive::explosive::explosive_type_to_tnt;
use crate::lang::{Lang, name_to_local};
use crate::missile::known_missiles::KnownMissiles;
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

	/// Range band distances for different spectrums (when infrared else its 0)
	/// 0 = rear aspect engine
	/// 1 = all aspect of target
	/// 2 = infrared decoys
	/// 3 = infrared countermeasures and sun
	pub bands: [f64; 4],

	/// size of the uncaged part of the seeker in degrees
	pub fov: f64,

	/// size of the locked seeker center in degrees
	pub gate: f64,

	/// distance from bore before launch in degrees
	pub lockanglemax: f64,

	/// distance from bore after launch in degrees
	pub anglemax: f64,

	/// distance from sun to be distracted in degrees
	pub minangletosun: f64,

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

	// Calculated (dynamically created and not in files)

	/// total potential energy in m/s²
	pub deltav: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Copy, Clone, const_gen::CompileConst, get_size::GetSize)]
pub enum SeekerType {
	Ir = 0,
	Radar = 1,
}

impl Missile {
	pub fn new_from_file(file: &[u8], name: String) -> Self {
		let file = String::from_utf8(file.to_vec()).unwrap();

		let seekertype = {
			if file.contains("irSeeker") {
				SeekerType::Ir
			} else if file.contains("radarSeeker") {
				SeekerType::Radar
			} else {
				panic!("Cant identify seeker type")
			}
		};

		let mass = parameter_to_data(&file, "mass").unwrap().parse().unwrap();

		let mass_end = parameter_to_data(&file, "massEnd").unwrap().parse().unwrap();

		let mass_end1 = parameter_to_data(&file, "massEnd1").unwrap_or("0.0".to_owned()).parse().unwrap();

		let caliber = parameter_to_data(&file, "caliber").unwrap().parse().unwrap();

		let force0 = parameter_to_data(&file, "force").map_or_else(|| parameter_to_data(&file, "force0").unwrap().parse().unwrap(), |value| value.parse().unwrap());

		let force1 = parameter_to_data(&file, "force1").map_or(0.0, |value| value.parse().unwrap());

		let timefire0 = parameter_to_data(&file, "timeFire").map_or_else(|| parameter_to_data(&file, "timeFire0").unwrap().parse().unwrap(), |value| value.parse().unwrap());

		let timefire1 = parameter_to_data(&file, "timeFire1").map_or(0.0, |value| value.parse().unwrap());

		let cxk = parameter_to_data(&file, "CxK").unwrap().parse().unwrap();

		let dragcx = parameter_to_data(&file, "dragCx").unwrap().parse().unwrap();

		let timelife = parameter_to_data(&file, "timeLife").unwrap().parse().unwrap();

		let endspeed = parameter_to_data(&file, "endSpeed").unwrap().parse().unwrap();

		let exp_mass = explosive_type_to_tnt(&parameter_to_data(&file, "explosiveType").unwrap().replace('\"', ""), (parameter_to_data(&file, "explosiveMass").unwrap().parse::<f64>().unwrap() * 1000.0).round());

		let pfuse = parameter_to_data(&file, "hasProximityFuse").map_or(false, |value| value.parse().unwrap());

		let loadfactormax = parameter_to_data(&file, "loadFactorMax").unwrap().parse().unwrap();

		let mut reqaccelmax = 0.0;
		if let Some(value) = parameter_to_data(&file, "reqAccelMax") {
			reqaccelmax = value.parse().unwrap();
		}

		let mut bands: [f64; 4] = [0.0, 0.0, 0.0, 0.0];
		if seekertype == SeekerType::Ir {
			if let Some(value) = parameter_to_data(&file, "rangeBand0") {
				bands[0] = value.parse().unwrap();
			}
			if let Some(value) = parameter_to_data(&file, "rangeBand1") {
				bands[1] = value.parse().unwrap();
			}
			if let Some(value) = parameter_to_data(&file, "rangeBand2") {
				bands[2] = value.parse().unwrap();
			}
			if let Some(value) = parameter_to_data(&file, "rangeBand3") {
				bands[3] = value.parse().unwrap();
			}
		}

		let fov = parameter_to_data(&file, "fov").map_or(0.0, |value| value.parse().unwrap());

		let gate = parameter_to_data(&file, "gateWidth").map_or(0.0, |value| value.parse().unwrap());

		let lockanglemax = parameter_to_data(&file, "lockAngleMax").unwrap().parse().unwrap();

		let anglemax = parameter_to_data(&file, "angleMax").unwrap().parse().unwrap();

		let minangletosun = if seekertype == SeekerType::Ir {
			parameter_to_data(&file, "minAngleToSun").unwrap().parse().unwrap()
		} else {
			0.0
		};

		let timeout = parameter_to_data(&file, "timeOut").unwrap().parse().unwrap();

		let warmuptime = parameter_to_data(&file, "warmUpTime").unwrap().parse().unwrap();

		let worktime = parameter_to_data(&file, "workTime").unwrap().parse().unwrap();

		let cageable = parameter_to_data(&file, "uncageBeforeLaunch").unwrap().parse::<bool>().unwrap();

		let rate_max = parameter_to_data(&file, "rateMax").unwrap().parse::<f64>().unwrap();

		let inertial_navigation = parameter_to_data(&file, "inertialNavigation").unwrap_or_else(|| "false".to_owned()).parse::<bool>().unwrap();

		let use_target_vel = parameter_to_data(&file, "useTargetVel").unwrap_or_else(|| "false".to_owned()).parse::<bool>().unwrap();

		let allow_radar_slave = file.contains("designationSourceTypeMask");

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
			deltav: ((force0 / mass * timefire0) + (force1 / mass * timefire1)).round(),
		}
	}

	pub fn write_all(mut values: Vec<Self>) -> Vec<Self> {
		values.sort_by_key(|d| d.name.clone());
		fs::write("missile_index/all.json", serde_json::to_string_pretty(&values).unwrap()).unwrap();
		values
	}

	pub fn generate_from_index(index: &KnownMissiles) -> Vec<Self> {
		let mut generated: Vec<Self> = vec![];
		for i in &index.path {
			if let Ok(file) = fs::read(format!("missile_index/missiles/{}", i)) {
				let name = i.split('.').collect::<Vec<&str>>()[0].to_owned();

				let missile = Missile::new_from_file(&file, name);

				generated.push(missile);
			}
		}
		generated.sort_by_key(|x| x.name.clone());
		generated
	}

	pub fn select_by_name(missiles: &[Self], name: &str) -> Option<Self> {
		for (i, missile) in missiles.iter().enumerate() {
			if missile.name.contains(&name.replace('-', "_")) {
				return Some(missiles[i].clone());
			}
		}
		None
	}
}
