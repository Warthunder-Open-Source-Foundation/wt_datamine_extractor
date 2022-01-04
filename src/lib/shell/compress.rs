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
	pub fn compress(shells: Vec<Shell>) -> Self {
		let mut compressed: Self = CompressedShells {
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
		};

		for shell in shells.clone() {
			compressed.name.push(shell.name);
			compressed.localized.push(shell.localized);
			compressed.parent_guns.push(shell.parent_guns);
			compressed.hash.push(shell.hash);
			compressed.shell_type.push(shell.shell_type);
			compressed.caliber.push(shell.caliber);
			compressed.true_caliber.push(shell.true_caliber);
			compressed.velocity.push(shell.velocity);
			compressed.penetration.push(shell.penetration);
			compressed.explosive.push(shell.explosive);
		}
		compressed
	}
}