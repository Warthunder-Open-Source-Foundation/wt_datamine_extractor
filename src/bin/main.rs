use std::fs;
use std::time::Instant;

use get_size::GetSize;
use wt_datamine_extractor_lib::atgm_index::atgm::Atgm;
use wt_datamine_extractor_lib::atgm_index::known_atgm::{KNOWN_ATGM_LOC, KnownAtgms};

use wt_datamine_extractor_lib::battle_rating::battle_rating::VehicleBattleRating;
use wt_datamine_extractor_lib::bombs::bombs::Bomb;
use wt_datamine_extractor_lib::bombs::known_bombs::{KNOWN_BOMBS_LOC, KnownBombs};
use wt_datamine_extractor_lib::extraction_traits::core::ExtractCore;
use wt_datamine_extractor_lib::extraction_traits::csv_serialize::CsvSerialize;
use wt_datamine_extractor_lib::extraction_traits::dummy::DummyCore;
use wt_datamine_extractor_lib::extraction_traits::known::KnownItem;
use wt_datamine_extractor_lib::lang::copy_lang;
use wt_datamine_extractor_lib::missile::known_missiles::KNOWN_MISSILES_LOC;
use wt_datamine_extractor_lib::missile::known_missiles::KnownMissiles;
use wt_datamine_extractor_lib::missile::missile::Missile;
use wt_datamine_extractor_lib::shell::compress::CompressedShells;
use wt_datamine_extractor_lib::shell::known_shells::{KNOWN_SHELLS_LOC, KnownShells};
use wt_datamine_extractor_lib::shell::shells::Shell;
use wt_datamine_extractor_lib::thermal::known_thermals::KnownThermals;
use wt_datamine_extractor_lib::thermal::thermals::Thermal;

fn main() {
	let start = Instant::now();

	if fs::read_dir("resources/cache").is_ok() {
		fs::write("meta_index/version.txt", &fs::read_to_string("resources/cache/aces.vromfs.bin_u/version").unwrap()).unwrap();
		fs::write("explosive/explosive.blkx", &fs::read_to_string("resources/cache/aces.vromfs.bin_u/gamedata/damage_model/explosive.blkx").unwrap()).unwrap();
		fs::write("battle_rating/wpcost.blkx", &fs::read_to_string("resources/cache/char.vromfs.bin_u/config/wpcost.blkx").unwrap()).unwrap();

		copy_lang();

		let known_missiles = KnownMissiles::generate_index(KNOWN_MISSILES_LOC).write_index("missile_index/known.json").copy_index_to_folder(KNOWN_MISSILES_LOC, "missile_index/missiles/");
		let known_thermals = KnownThermals::generate_index("").write_index("thermal_index/known.json").copy_index_to_folder("", "thermal_index/thermals/");
		let known_shells = KnownShells::generate_index(KNOWN_SHELLS_LOC).write_index("shell_index/known.json").copy_index_to_folder(KNOWN_SHELLS_LOC, "shell_index/shells/");
		let known_bombs = KnownBombs::generate_index(KNOWN_BOMBS_LOC).write_index("bombs/known.json").copy_index_to_folder(KNOWN_BOMBS_LOC, "bombs/index/");
		// let known_atgms = KnownAtgms::generate_index(KNOWN_ATGM_LOC).write_index("atgm/known.json").copy_index_to_folder(KNOWN_ATGM_LOC, "atgm/index/");

		let missiles = Missile::generate_from_index(known_missiles, "missile_index/missiles/");
		let thermals = Thermal::generate_from_index(known_thermals, "thermal_index/thermals/");
		let shells = Shell::generate_from_index(&known_shells);
		let bombs = Bomb::generate_from_index(known_bombs, "bombs/index/");
		let battle_ratings = VehicleBattleRating::generate_from_index(DummyCore, "battle_rating/wpcost.blkx");
		// let atgms = Atgm::generate_from_index(DummyCore, "atgm/index/");

		let compressed_shells = CompressedShells::compress(&shells);

		println!("Missiles: {}kb\nThermals: {}kb\nShells(compressed): {}kb({}kb)\nBombs: {}kb\nBattle Ratings: {}kb",
				 missiles.get_heap_size() / 1024,
				 thermals.get_heap_size() / 1024,
				 shells.get_heap_size() / 1024,
				 compressed_shells.get_heap_size() / 1024,
				 bombs.get_heap_size() / 1024,
				battle_ratings.get_heap_size() / 1024
		);


		fs::write("shell_index/compressed.json", serde_json::to_string(&compressed_shells).unwrap()).unwrap();

		Missile::write_all(missiles, "missile_index/all.json");
		Thermal::write_all(thermals, "thermal_index/all.json");
		Shell::write_all(shells);
		Bomb::write_all(bombs, "bombs/all.json");
		VehicleBattleRating::to_csv(battle_ratings.clone(), "battle_rating/all.csv").unwrap();
		VehicleBattleRating::write_all(battle_ratings, "battle_rating/all.json");
		// Atgm::write_all(atgms, "atgm/all.json");
	} else {
		panic!("Local mined cache is invalid or could not be read");
	}

	println!("Process took {:?}", start.elapsed());
}

// fn copy_loadouts() {
// 	let options = CopyOptions {
// 		overwrite: true,
// 		skip_exist: false,
// 		buffer_size: 10_000,
// 		copy_inside: true,
// 		content_only: false,
// 		depth: 0,
// 	};
//
// 	fs_extra::dir::copy("resources/cache/aces.vromfs.bin_u/gamedata/weapons", "./custom_loadouts", &options).unwrap();
// }