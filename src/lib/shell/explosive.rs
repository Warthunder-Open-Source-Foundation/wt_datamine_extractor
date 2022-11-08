
pub enum ExplosiveType {
	Inert,
	Energetic(Explosive)
}

pub struct Explosive {
	name_type: String,
	raw_mass: u32,
	equiv_mass: u32,
}