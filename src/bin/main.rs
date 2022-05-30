use std::fs;
use std::time::Instant;

use fs_extra::dir::CopyOptions;
use get_size::GetSize;

use wt_datamine_extractor_lib::bombs::bombs::Bomb;
use wt_datamine_extractor_lib::bombs::known_bombs::{KNOWN_BOMBS_LOC, KnownBombs};
use wt_datamine_extractor_lib::custom_loadouts::custom_loadouts::CustomLoadout;
use wt_datamine_extractor_lib::custom_loadouts::known_loadouts::{KNOWN_LOADOUTS_LOC, KnownLoadouts};
use wt_datamine_extractor_lib::extraction_traits::known::KnownItem;
use wt_datamine_extractor_lib::lang::copy_lang;
use wt_datamine_extractor_lib::missile::known_missiles::KnownMissiles;
use wt_datamine_extractor_lib::missile::missile::Missile;
use wt_datamine_extractor_lib::shell::compress::CompressedShells;
use wt_datamine_extractor_lib::shell::known_shells::{KNOWN_SHELLS_LOC, KnownShells};
use wt_datamine_extractor_lib::shell::shells::Shell;
use wt_datamine_extractor_lib::thermal::known_thermals::KnownThermals;
use wt_datamine_extractor_lib::thermal::thermals::Thermal;
use wt_datamine_extractor_lib::missile::known_missiles::KNOWN_MISSILES_LOC;

fn main() {
	let start = Instant::now();

	if fs::read_dir("resources/cache").is_ok() {
		fs::write("meta_index/version.txt", &fs::read_to_string("resources/cache/aces.vromfs.bin_u/version").unwrap()).unwrap();
		fs::write("explosive/explosive.blkx", &fs::read_to_string("resources/cache/aces.vromfs.bin_u/gamedata/damage_model/explosive.blkx").unwrap()).unwrap();

		copy_lang();
		copy_loadouts();

		let known_missiles = KnownMissiles::generate_index(KNOWN_MISSILES_LOC).write_index("missile_index/known.json").copy_index_to_folder(KNOWN_MISSILES_LOC, "missile_index/missiles/");
		let known_thermals = KnownThermals::generate_index().write_index().copy_index_to_folder();
		let known_shells = KnownShells::generate_index(KNOWN_SHELLS_LOC).write_index("shell_index/known.json").copy_index_to_folder(KNOWN_SHELLS_LOC, "shell_index/shells/");
		let known_loadouts = KnownLoadouts::generate_index(KNOWN_LOADOUTS_LOC).write_index("custom_loadouts/known.json").copy_index_to_folder(KNOWN_LOADOUTS_LOC, "custom_loadouts/aircraft/");

		let known_bombs = KnownBombs::generate_index(KNOWN_BOMBS_LOC).write_index("bombs/known.json").copy_index_to_folder(KNOWN_BOMBS_LOC, "bombs/index/");

		let missiles = Missile::generate_from_index(&known_missiles);
		let thermals = Thermal::generate_from_index(&known_thermals);
		let shells = Shell::generate_from_index(&known_shells);
		let loadouts = CustomLoadout::generate_from_index(&known_loadouts);
		let bombs = Bomb::generate_from_index(&known_bombs);

		let compressed_shells = CompressedShells::compress(&shells);

		println!("Missiles: {}kb\nThermals: {}kb\nShells(compressed): {}kb({}kb)\nLoadouts: {}kb\nBombs: {}kb",
				 missiles.get_heap_size() / 1024,
				 thermals.get_heap_size() / 1024,
				 shells.get_heap_size() / 1024,
				 compressed_shells.get_heap_size() / 1024,
				 loadouts.get_heap_size() / 1024,
				 bombs.get_heap_size() / 1024,
		);


		fs::write("shell_index/compressed.json", serde_json::to_string(&compressed_shells).unwrap()).unwrap();

		Missile::write_all(missiles);
		Thermal::write_all(thermals);
		Shell::write_all(shells);
		CustomLoadout::write_all(loadouts);
		Bomb::write_all(bombs);
	} else {
		panic!("Local mined cache is invalid or could not be read");
	}

	println!("Process took {:?}", start.elapsed());
}

pub fn copy_loadouts() {
	let options = CopyOptions {
		overwrite: true,
		skip_exist: false,
		buffer_size: 10_000,
		copy_inside: true,
		content_only: false,
		depth: 0,
	};

	fs_extra::dir::copy("resources/cache/aces.vromfs.bin_u/gamedata/weapons", "./custom_loadouts", &options).unwrap();
}