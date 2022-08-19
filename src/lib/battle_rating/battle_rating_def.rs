use std::fmt::{Display, Formatter};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Default, Hash, PartialEq, Eq)]
pub struct BattleRating {
	pub economic_rank: u8
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GameMode {
	Arcade,
	// Referred to as 'Historical' in files
	Realistic,
	Simulator,
}

impl Default for GameMode {
	fn default() -> Self {
		Self::Arcade
	}
}

pub const BATTLE_RATINGS: [u8; 3] = [0, 3, 7];

impl Display for BattleRating {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		let split = self.to_split();
		write!(f, "{}.{}", split.0, split.1)
	}
}

impl BattleRating {
	pub fn to_split(&self) -> (u8, u8) {
		let left = self.economic_rank / 3 + 1;
		let right = self.economic_rank % 3;

		(left, BATTLE_RATINGS[right as usize])
	}
	pub fn new(num: u8) -> Self {
		Self {
			economic_rank: num
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::battle_rating::battle_rating_def::{BattleRating};

	#[test]
	fn br_0() {
		assert_eq!(BattleRating::new(0).to_string(), "1.0");
	}

	#[test]
	fn br_6_7() {
		assert_eq!(BattleRating::new(17).to_string(), "6.7");
	}

	#[test]
	fn br_9_3() {
		assert_eq!(BattleRating::new(25).to_string(), "9.3");
	}

	#[test]
	fn br_11_3() {
		assert_eq!(BattleRating::new(31).to_string(), "11.3");
	}
}