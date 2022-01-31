use crate::shell::parent_gun::ParentGun;
use crate::shell::shells::{Shell, ShellType};

#[derive(serde::Serialize, Clone, serde::Deserialize, Debug, PartialEq, Hash, Eq, Default)]
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
	pub fn compress(shells_old: &[Shell]) -> Self {
		let mut shells = shells_old.clone().to_vec();
		shells.sort_by_key(|x|format!("{:?}", x));
		shells.iter().fold(CompressedShells::default(), |mut acc, v| {
			acc.name.push(v.name.clone());
			acc.localized.push(v.localized.clone());
			acc.shell_type.push(v.shell_type.clone());
			acc.caliber.push(v.caliber);
			acc.true_caliber.push(v.true_caliber);
			acc.velocity.push(v.velocity);
			acc.penetration.push(v.penetration.clone());
			acc.explosive.push(v.explosive.clone());
			acc
		})
	}
	pub fn decompress(&self) -> Vec<Shell> {
		let mut shells = Vec::new();
		for i in self.name.iter().enumerate() {
			shells.push( Shell {
				name: i.1.clone(),
				localized: self.localized[i.0].clone(),
				shell_type: self.shell_type[i.0].clone(),
				caliber: self.caliber[i.0],
				true_caliber: self.true_caliber[i.0],
				velocity: self.velocity[i.0],
				penetration: self.penetration[i.0].clone(),
				explosive: self.explosive[i.0].clone(),
			});
		}
		shells
	}
}