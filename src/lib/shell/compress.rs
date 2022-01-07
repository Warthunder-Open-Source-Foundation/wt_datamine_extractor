use crate::shell::parent_gun::ParentGun;
use crate::shell::shells::{Shell, ShellType};

#[derive(serde::Serialize, Clone, serde::Deserialize, Debug, PartialEq, Hash, Eq)]
pub struct CompressedShells {
	/// Metadata
	pub name: Vec<String>,
	pub localized: Vec<String>,
	pub parent_guns: Vec<Vec<ParentGun>>,
	pub hash: Vec<u64>,

	pub shell_type: Vec<ShellType>,

	// in mm
	pub caliber: Vec<u32>,
	pub true_caliber: Vec<u32>,

	// in m/s
	pub velocity: Vec<u32>,

	// in mm
	pub penetration: Vec<Vec<(u32, u32)>>,

	// 1st is type, 2nd is raw mass, 3rd is TNT equivalent mass
	pub explosive: Vec<(String, u32, u32)>,
}

impl CompressedShells {
	pub fn new() ->  Self {
		Self {
			name: vec![],
			localized: vec![],
			parent_guns: vec![],
			hash: vec![],
			shell_type: vec![],
			caliber: vec![],
			true_caliber: vec![],
			velocity: vec![],
			penetration: vec![],
			explosive: vec![]
		}
	}
	pub fn compress(shells: &Vec<Shell>) -> Self {
		let aggregated = shells.into_iter().fold(CompressedShells::new(), |mut acc, v| {
			acc.name.push(v.name.clone());
			acc.localized.push(v.localized.clone());
			acc.parent_guns.push(v.parent_guns.clone());
			acc.hash.push(v.hash);
			acc.shell_type.push(v.shell_type.clone());
			acc.caliber.push(v.caliber);
			acc.true_caliber.push(v.true_caliber);
			acc.velocity.push(v.velocity);
			acc.penetration.push(v.penetration.clone());
			acc.explosive.push(v.explosive.clone());
			acc
		});
		aggregated
	}
	pub fn decompress(&self) -> Vec<Shell> {
		let mut shells = Vec::new();
		for i in self.name.iter().enumerate() {
			shells.push( Shell {
				name: i.1.clone(),
				localized: self.localized[i.0].clone(),
				parent_guns: self.parent_guns[i.0].clone(),
				hash: self.hash[i.0].clone(),
				shell_type: self.shell_type[i.0].clone(),
				caliber: self.caliber[i.0].clone(),
				true_caliber: self.true_caliber[i.0].clone(),
				velocity: self.velocity[i.0].clone(),
				penetration: self.penetration[i.0].clone(),
				explosive: self.explosive[i.0].clone(),
			})
		}
		shells
	}
}