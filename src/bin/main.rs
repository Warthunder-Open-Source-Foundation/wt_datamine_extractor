use std::fs;
use std::time::Instant;
use wt_datamine_extractor_lib::lang::extract_csv;
use wt_datamine_extractor_lib::missile::extract_missiles::KnownMissiles;
use wt_datamine_extractor_lib::missile::missile::Missile;
use wt_datamine_extractor_lib::thermal::extract_thermals::KnownThermals;
use wt_datamine_extractor_lib::thermal::thermals::Thermal;

fn main() {
	let start = Instant::now();

	if fs::read_dir("resources/cache").is_ok() {
		extract_csv();

		let known_missiles = KnownMissiles::generate_index().write_index().copy_index_to_folder();
		let known_thermals = KnownThermals::generate_index().write_index().copy_index_to_folder();

		let missiles = Missile::generate_from_index(&known_missiles);
		let thermals = Thermal::generate_from_index(&known_thermals);

		Missile::write_all(missiles);
		Thermal::write_all(thermals);
	} else {
		panic!("Local mined cache is invalid or could not be read");
	}

	println!("{:?}", start.elapsed());
}