use std::fs;

use crate::extract::KnownMissiles;

pub const PATH: &str = "./index/missiles";

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Missile {
	// metadata that is global or does not exist on files that are generated

	/// associated file-name of the missiles
	pub name: String,

	/// type of seeker that has to be extracted from the file and represented as enum
	pub seekertype: SeekerType,


	// main data raw values taken from the file

	/// mass in kg
	pub mass: f64,

	/// mass in kg after engine burn
	pub mass_end: f64,

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

	/// amount of raw explosive material in kg
	pub exp_mass: f64,

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

	/// time to switch missile into seek mode in s
	pub warmuptime: f64,

	/// time the missile remains in seek mode in s
	pub worktime: f64,

	/// if the seeker is on a gimbal or not
	pub cageable: bool,


	// Calculated (dynamically created and not in files)

	/// total potential energy in m/s²
	pub deltav: f64,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum SeekerType {
	Ir = 0,
	Radar = 1,
}

impl Missile {
	pub fn new_from_file(file: &[u8], name: String) -> Self {
		let file = String::from_utf8(file.to_vec()).unwrap();

		let name = name.split("/").collect::<Vec<&str>>()[3].split(".").collect::<Vec<&str>>()[0].to_string();

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

		let caliber = parameter_to_data(&file, "caliber").unwrap().parse().unwrap();

		let force0 = parameter_to_data(&file, "force").map_or_else(|| parameter_to_data(&file, "force0").unwrap().parse().unwrap(), |value| value.parse().unwrap());

		let force1 = parameter_to_data(&file, "force1").map_or(0.0, |value| value.parse().unwrap());

		let timefire0 = parameter_to_data(&file, "timeFire").map_or_else(|| parameter_to_data(&file, "timeFire0").unwrap().parse().unwrap(), |value| value.parse().unwrap());

		let timefire1 = parameter_to_data(&file, "timeFire1").map_or(0.0, |value| value.parse().unwrap());

		let cxk = parameter_to_data(&file, "CxK").unwrap().parse().unwrap();

		let dragcx = parameter_to_data(&file, "dragCx").unwrap().parse().unwrap();

		let timelife = parameter_to_data(&file, "timeLife").unwrap().parse().unwrap();

		let endspeed = parameter_to_data(&file, "endSpeed").unwrap().parse().unwrap();

		let exp_mass = parameter_to_data(&file, "explosiveMass").unwrap().parse().unwrap();

		let pfuse = parameter_to_data(&file, "hasProximityFuse").unwrap().parse().unwrap();

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

		let warmuptime = parameter_to_data(&file, "warmUpTime").unwrap().parse().unwrap();

		let worktime = parameter_to_data(&file, "workTime").unwrap().parse().unwrap();

		let cageable = parameter_to_data(&file, "uncageBeforeLaunch").unwrap().parse().unwrap();

		Self {
			name,
			seekertype,
			mass,
			mass_end,
			caliber,
			force0,
			force1,
			timefire0,
			timefire1,
			cxk,
			dragcx,
			timelife,
			endspeed,
			exp_mass,
			pfuse,
			loadfactormax,
			reqaccelmax,
			bands,
			fov,
			gate,
			lockanglemax,
			anglemax,
			minangletosun,
			warmuptime,
			worktime,
			cageable,
			deltav: ((force0 / mass * timefire0) + (force1 / mass * timefire1)).round(),
		}
	}
	pub fn new_from_generated(path: Option<&str>, regen: Option<&str>) -> Option<Vec<Self>> {
		if let Some(value) = regen {
			generate_raw(value);
			println!("Regenerating missile-index");
		}

		if let Some(path) = path {
			// Path provided

			if let Ok(from_reader) = fs::read_to_string(path) {
				// Attempt to get from reader

				if let Ok(serialized) = serde_json::from_str::<Vec<Self>>(&from_reader) {
					// Serialized result
					return Some(serialized);
				} else {
					println!("Cannot parse missile-index from {}, using fallback", path);
				}
			} else {
				println!("Cannot read missile-index from {}, using fallback", path);
			}
		}
		// If the given path does not work in some way a fallback will be used
		return get_fallback();


		fn get_fallback() -> Option<Vec<Missile>> {
			static FALLBACK: &str = "index/all.json";
			if let Ok(from_reader) = fs::read_to_string(FALLBACK) {
				// Attempt to get from reader

				if let Ok(serialized) = serde_json::from_str::<Vec<Missile>>(&from_reader) {
					// Serialized result
					return Some(serialized);
				}
				println!("Fallback {} cannot be parsed", FALLBACK);
				return None;
			}
			println!("Fallback {} cannot be found", FALLBACK);
			None
		}
	}
	pub fn select_by_name(missiles: &Vec<Self>, name: &str) -> Option<Self> {
		for (i, missile) in missiles.iter().enumerate() {
			if missile.name.contains(&name.replace("-", "_")) {
				return Some(missiles[i].clone());
			}
		}
		None
	}

	pub fn new_from_empty() -> Self {
		Self {
			name: "".to_string(),
			seekertype: SeekerType::Ir,
			mass: 0.0,
			mass_end: 0.0,
			caliber: 0.0,
			force0: 0.0,
			force1: 0.0,
			timefire0: 0.0,
			timefire1: 0.0,
			cxk: 0.0,
			dragcx: 0.0,
			timelife: 0.0,
			endspeed: 0.0,
			exp_mass: 0.0,
			pfuse: false,
			loadfactormax: 0.0,
			reqaccelmax: 0.0,
			bands: [0.0, 0.0, 0.0, 0.0],
			fov: 0.0,
			gate: 0.0,
			lockanglemax: 0.0,
			anglemax: 0.0,
			minangletosun: 0.0,
			warmuptime: 0.0,
			worktime: 0.0,
			cageable: false,
			deltav: 0.0
		}
	}
}

fn parameter_to_data(file: &str, parameter: &str) -> Option<String> {
	file.find(parameter).map(|value| {
		let position_value = file.split_at(value + parameter.len() + 3).1;
		let cropped_value = position_value.split_once("\n").unwrap().0;
		let cleaned_value = cropped_value.replace(",", ""); // Sub-objects somehow contain a comma
		cleaned_value.trim().to_owned()
	})
}

pub fn generate_raw(path: &str) {
	let dir_ir = fs::read_dir(format!("{}", path)).unwrap();

	let mut files: Vec<String> = vec![];
	let mut known: KnownMissiles = KnownMissiles::new_from_index(vec![]);
	for (_, entry) in dir_ir.enumerate() {
		let file_name = entry.unwrap().file_name().into_string().unwrap();
		if file_name.contains("blkx") {
			files.push(format!("{}/{}", path, file_name));
			known.path.push(file_name);
		}
	}

	let mut missiles: Vec<Missile> = vec![];
	for file in files {
		let data = fs::read(&file).unwrap();
		missiles.push(Missile::new_from_file(&data, file));
	}

	let known_json = serde_json::to_string_pretty(&known).unwrap();
	fs::write("index/known.json", known_json).unwrap();

	let missiles_json = serde_json::to_string_pretty(&missiles).unwrap();
	fs::write("../wt_missile_calc/index/all.json", missiles_json).unwrap();
	//println!("{:#?}", missiles);
}
