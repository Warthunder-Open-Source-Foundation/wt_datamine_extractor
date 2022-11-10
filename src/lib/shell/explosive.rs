use const_gen::CompileConst;
use get_size::GetSize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, CompileConst, GetSize)]
pub enum ExplosiveType {
	Inert,
	Energetic(Explosive)
}

#[derive(Debug, Serialize, Deserialize, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, CompileConst, GetSize)]
pub struct Explosive {
	pub name_type: String,
	pub raw_mass: u32,
	pub equiv_mass: u32,
}