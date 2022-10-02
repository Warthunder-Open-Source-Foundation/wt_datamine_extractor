use std::fmt::{Display, Formatter};
use std::str::FromStr;
use const_gen::CompileConst;
use get_size::GetSize;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, Default, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Rank(u8);

impl Rank {
	pub fn new(rank: u8) -> Self {
		Self(rank)
	}
}

impl Display for Rank {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "{}", roman::to(self.0.into()).expect("This should never overflow, as the into call safely wraps"))
	}
}

impl FromStr for Rank {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if let Some(rank) = roman::from(s) {
			let parsed = u8::try_from(rank).or_else(|_|Err(()))?;
			Ok(Self::new(parsed))
		} else {
			Err(())
		}
	}
}

impl GetSize for Rank {

}