use std::fs;
use get_size::GetSize;
use crate::extraction_traits::core::ExtractCore;
use crate::extraction_traits::known::KnownItem;

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, const_gen::CompileConst, get_size::GetSize)]
pub struct Atgm {
	pub name: String,

	/// Generic data
	// Weights throughout engine burns
	pub weights: Vec<f64>,
	pub caliber: f64,
	pub time_life: f64,

	/// FM data
	// technically a dub to below, but easier
	pub weight: f64,
	pub thrust_and_burn_time: Vec<(f64,f64)>,
	pub cxk: f64,
	pub start_speed: f64,
	pub max_speed: f64,
	pub max_lateral_accel: f64,

	/// DM data
	pub explosive_mass: f64,
	pub explosive_equiv: f64,
	pub explosive_type: String,
	pub is_tandem: bool,
	pub penetration: f64,
	pub has_proximity_fuse: bool,

	/// Guidance data
	pub seeker_type: SeekerType,
	pub guidance_duration: f64,
	pub guidance_range: f64,
	pub targets_surface: bool,
	pub targets_vehicles: bool,
	pub is_lofting: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, const_gen::CompileConst, get_size::GetSize)]
pub enum SeekerType {
	// Manual Command Line Of Sight
	MCLOS,
	// Semi Automatic Command Line Of Sight
	SACLOS,
	// Line Of Sight Beam Riding
	LOSBR,
	// Semi Active Laser Homing
	SALH,
	// Imaging Infra Red - not actually IIR atm, but it works like one
	IIR,
	// Charge Coupled Device - AKA visual
	CCD,
}

impl ExtractCore for Atgm {
	fn generate_from_index(_: impl KnownItem, write_path: &str) -> Vec<Self> where Self: Sized {
		let raw = fs::read_to_string(write_path).unwrap();
		let mut items: Vec<Self> = vec![];

		items
	}

	fn new_from_file(file: &[u8], name: String) -> Self {
		todo!()
	}

	fn sort(items: &mut Vec<Self>) where Self: Sized {
		items.sort_by_key(|x| x.name.clone());
	}
}