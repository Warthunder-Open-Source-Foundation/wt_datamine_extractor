use std::fs;
use std::str::FromStr;
use const_gen::CompileConst;
use get_size::GetSize;
use serde::{Deserialize, Serialize};

use crate::battle_rating::battle_rating_def::{BattleRating};
use crate::battle_rating::nation::{Nation, TechTree};
use crate::battle_rating::rank::Rank;
use crate::lang::{Lang, name_to_local};

#[derive(Serialize, Deserialize, Clone, Debug, Default, Hash, PartialEq, Eq, CompileConst, GetSize)]
pub struct VehicleBattleRating {
	pub name: String,
	pub localized: String,
	pub arcade: BattleRating,
	pub realistic: BattleRating,
	pub simulator: BattleRating,
	pub rank: Rank,
	pub tech_tree: TechTree,
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

			let get_line = |line: String| {
				line.split(':').last().unwrap().split(",").next().unwrap().trim().to_owned()
			};
			if line.contains("\"economicRankArcade\":") {
				let split = get_line(line.to_owned()).parse().unwrap();
				current.arcade = BattleRating::new(split);
				continue;
			}
			if line.contains("\"economicRankHistorical\":") {
				let split = get_line(line.to_owned()).parse().unwrap();
				current.realistic = BattleRating::new(split);
				continue;
			}
			if line.contains("\"economicRankSimulation\":") {
				let split = get_line(line.to_owned()).parse().unwrap();
				current.simulator = BattleRating::new(split);
				continue;
			}
			if line.contains("\"rank\":") {
				let split = get_line(line.to_owned()).parse().unwrap();
				current.rank = Rank::new(split);
				continue;
			}
			if line.contains("\"country\":") {
				let split = get_line(line.to_owned());
				current.tech_tree = TechTree {
					nation: Nation::from_str(&split).unwrap()
				}
			}
		}
		items
	}
	pub fn write_all(vec: Vec<Self>, path: &str) {
		let serialized = serde_json::to_string_pretty(&vec).unwrap();
		fs::write(path, serialized).unwrap();
	}
}