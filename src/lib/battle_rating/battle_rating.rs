use std::fs;

use serde::{Deserialize, Serialize};

use crate::battle_rating::battle_rating_def::BattleRating;
use crate::lang::{Lang, name_to_local};

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, PartialEq, Eq)]
pub struct VehicleBattleRating {
	pub name: String,
	pub localized: String,
	pub arcade: BattleRating,
	pub realistic: BattleRating,
	pub simulator: BattleRating,
	pub rank: u8,
}

impl VehicleBattleRating {
	pub fn generate_from_index(path: &str) -> Vec<Self> {
		let raw = fs::read_to_string(path).unwrap();
		let mut items: Vec<VehicleBattleRating> = vec![];
		let mut current = VehicleBattleRating::default();
		for line in raw.split('\n').collect::<Vec<&str>>() {
			// Indentation based detection of a new vehicle
			if line.starts_with(r#"  ""#) && line.contains(r#"": {"#) {
				current = VehicleBattleRating::default();

				current.name = line.split_at(3).1.split('"').next().unwrap().to_owned();
				current.localized = name_to_local(&current.name, &Lang::Unit);
			}
			// Store latest and reset
			if line.starts_with("  },") {
				items.push(current.clone());
				continue;
			}

			let get_line = |line: &str| {
				line.split(':').last().unwrap().split(",").next().unwrap().trim().parse().unwrap()
			};
			if line.contains("\"economicRankArcade\":") {
				let split = get_line(line);
				current.arcade = BattleRating::new(split);
				continue;
			}
			if line.contains("\"economicRankHistorical\":") {
				let split = get_line(line);
				current.realistic = BattleRating::new(split);
				continue;
			}
			if line.contains("\"economicRankSimulation\":") {
				let split = get_line(line);
				current.simulator = BattleRating::new(split);
				continue;
			}
			if line.contains("\"rank\":") {
				let split = get_line(line);
				current.rank = split;
				continue;
			}
		}
		items
	}
	pub fn write_all(vec: Vec<Self>, path: &str) {
		let serialized = serde_json::to_string_pretty(&vec).unwrap();
		fs::write(path, serialized).unwrap();
	}
}