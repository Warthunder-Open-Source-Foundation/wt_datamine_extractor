use std::fs;

use crate::extract::KnownMissiles;

pub const PATH: &str = "./index/missiles";

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone)]
pub struct Missile {
	// Metadata
	pub name: String,
	pub seekertype: SeekerType,

	// Main data
	pub mass: f64,
	pub mass_end: f64,
	pub caliber: f64,
	pub force0: f64,
	pub force1: f64,
	pub timefire0: f64,
	pub timefire1: f64,
	pub cxk: f64,
	pub dragcx: f64,
	pub timelife: f64,
	pub endspeed: f64,
	pub tnt: f64,
	pub pfuse: bool,
	pub loadfactormax: f64,
	pub reqaccelmax: f64,
	pub bands: [f64; 4],
	pub fov: f64,
	pub gate: f64,
	pub lockanglemax: f64,
	pub anglemax: f64,
	pub minangletosun: f64,
	pub warmuptime: f64,
	pub worktime: f64,
	pub cageable: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone)]
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

		let caliber = parameter_to_data(&file, "caliber").unwrap().parse().unwrap();

		let force0 = parameter_to_data(&file, "force").map_or_else(|| parameter_to_data(&file, "force0").unwrap().parse().unwrap(), |value| value.parse().unwrap());

		let force1 = parameter_to_data(&file, "force1").map_or(0.0, |value| value.parse().unwrap());

		let timefire0 = parameter_to_data(&file, "timeFire").map_or_else(|| parameter_to_data(&file, "timeFire0").unwrap().parse().unwrap(), |value| value.parse().unwrap());

		let timefire1 = parameter_to_data(&file, "timeFire1").map_or(0.0, |value| value.parse().unwrap());

		let cxk = parameter_to_data(&file, "CxK").unwrap().parse().unwrap();

		let dragcx = parameter_to_data(&file, "dragCx").unwrap().parse().unwrap();

		let timelife = parameter_to_data(&file, "timeLife").unwrap().parse().unwrap();

		let endspeed = parameter_to_data(&file, "endSpeed").unwrap().parse().unwrap();

		let tnt = parameter_to_data(&file, "explosiveMass").unwrap().parse().unwrap();

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

		let fov = parameter_to_data(&file, "timeFire1").map_or(0.0, |value| value.parse().unwrap());

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
			tnt,
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
		}
	}
	pub fn new_from_generated(path: Option<&str>, regen: Option<&str>) -> Vec<Self> {
		if let Some(value) =  regen {
			generate_raw(value);
		}
		return if let Some(value) = path {
			serde_json::from_str(&fs::read_to_string(value).unwrap()).unwrap()
		} else {
			serde_json::from_str(&fs::read_to_string("./resources/all.json").unwrap()).unwrap()
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
	fs::write("index/all.json", missiles_json).unwrap();
	//println!("{:#?}", missiles);
}
