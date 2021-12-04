use std::fs;
use std::time::Instant;
use wt_datamine_extractor_lib::lang::extract_csv;
use wt_datamine_extractor_lib::missile::extract_missiles::KnownMissiles;
use wt_datamine_extractor_lib::missile::missile::Missile;
use wt_datamine_extractor_lib::shell::known_shells::KnownShells;
use wt_datamine_extractor_lib::shell::shells::Shell;
use wt_datamine_extractor_lib::thermal::extract_thermals::KnownThermals;
use wt_datamine_extractor_lib::thermal::thermals::Thermal;

fn main() {
	let start = Instant::now();

	// if fs::read_dir("resources/cache").is_ok() {
	// 	extract_csv();
	//
	// 	let known_missiles = KnownMissiles::generate_index().write_index().copy_index_to_folder();
	// 	let known_thermals = KnownThermals::generate_index().write_index().copy_index_to_folder();
	//
	// 	let missiles = Missile::generate_from_index(&known_missiles);
	// 	let thermals = Thermal::generate_from_index(&known_thermals);
	//
	// 	Missile::write_all(missiles);
	// 	Thermal::write_all(thermals);
	// } else {
	// 	panic!("Local mined cache is invalid or could not be read");
	// }

	let known_shells = KnownShells::generate_index().write_index().copy_index_to_folder();

	let dm_53 = Shell::new_from_file(&fs::read("shell_index/shells/120mm_rheinmetall_l55_user_cannon.blkx").unwrap(), "120mm_rheinmetall_l55_user_cannon.blkx".to_owned());

	eprintln!("dm_53 = {:#?}", dm_53);
	println!("{:?}", start.elapsed());
}