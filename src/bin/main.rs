
use std::time::Instant;



use wt_datamine_extractor_lib::shell::known_shells::KnownShells;
use wt_datamine_extractor_lib::shell::shells::{Shell};



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

	let shells = Shell::generate_from_index(&known_shells);
	Shell::write_all(shells);

	println!("{:?}", start.elapsed());
}