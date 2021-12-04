
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Shell {
	/// Metadata
	pub name: String,

	pub shell_type: ShellType,
	pub caliber: u32,
	pub velocity: u32,
	pub penetration: [u32; 9], // X axis represents ranges from 0, 100, 500, 1000, 1500, 2000, 3000, 10000 and 20000
}

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub enum ShellType {
	APFSDS = 0,
	HEATFS = 1,
	HEFS = 2,
}