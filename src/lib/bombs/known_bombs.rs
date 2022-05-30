use crate::extraction_traits::known::{Index, KnownItem, OwnedIndex};

pub const KNOWN_BOMBS_LOC: &str = "resources/cache/aces.vromfs.bin_u/gamedata/weapons/bombguns/";

#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, const_gen::CompileConst, Default)]
pub struct KnownBombs {
	pub path: Vec<String>,
}

impl KnownItem for KnownBombs {
	fn push_index(&mut self, index: OwnedIndex) {
		self.path = index;
	}

	fn get_index(&self) -> Index {
		&self.path
	}
}