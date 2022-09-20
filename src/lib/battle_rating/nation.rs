use std::str::FromStr;
use const_gen::CompileConst;
use get_size::GetSize;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

#[derive(Display, Debug, Serialize, Deserialize, Clone, Hash, PartialEq, Eq, CompileConst, GetSize)]
pub enum Nation {
	USA,
	Germany,
	USSR,
	#[strum(serialize = "Great Britain")]
	#[serde(rename(serialize = "Great Britain"))]
	GreatBritain,
	Japan,
	China,
	Italy,
	France,
	Sweden,
	Israel,
}

impl Default for Nation {
	fn default() -> Self {
		Self::USA
	}
}

impl FromStr for Nation {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let s = s.replace("\"", "");
		match s.as_str() {
			"country_usa" => Ok(Self::USA),
			"country_germany" => Ok(Self::Germany),
			"country_ussr" => Ok(Self::USSR),
			"country_britain" => Ok(Self::GreatBritain),
			"country_japan" => Ok(Self::Japan),
			"country_china" => Ok(Self::China),
			"country_italy" => Ok(Self::Italy),
			"country_france" => Ok(Self::France),
			"country_sweden" => Ok(Self::Sweden),
			"country_israel" => Ok(Self::Israel),
			_ => {Err(())}
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, Default, Hash, PartialEq, Eq, const_gen::CompileConst, get_size::GetSize)]
pub struct TechTree {
	pub nation: Nation,
}