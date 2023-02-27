use get_size::GetSize;


#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, const_gen::CompileConst, get_size::GetSize)]
pub struct Visband {
	/// 0 VALUES MEAN IT IS FULLY RESISTANT TO SAID BAND!!!
	pub range_band0: usize, // rear aspect engine
	pub range_band1: usize, // all aspect
	pub range_band2: usize, // flares
	pub range_band3: usize, // infrared countermeasures (IRCM)
	pub range_band4: usize, // sun, rockets, ATGMs etc.
	pub range_band6: usize, // DIRCM
	pub range_band7: usize, // Afterburner plume
	pub range_max: usize, // Absolute cap
}

impl Visband {
	pub const fn rear_aspect(&self) -> usize {
		self.range_band0
	}
	pub const fn all_aspect(&self) -> usize {
		self.range_band1
	}
	pub const fn flares(&self) -> usize {
		self.range_band2
	}
	pub const fn ircm(&self) -> usize {
		self.range_band3
	}
	pub const fn sun_and_misc(&self) -> usize {
		self.range_band4
	}
	pub const fn dircm(&self) -> usize {
		self.range_band6
	}
	pub const fn afterburner_plume(&self) -> usize {
		self.range_band7
	}
	pub const fn absolute(&self) -> usize {
		self.range_max
	}
}