use std::collections::HashMap;

use any_ascii::any_ascii;
use fs_extra::dir::CopyOptions;
use lazy_static::lazy_static;
use wt_csv::wtcsv::core::wtcsv::WTCSV;

const EDGE_CASES: [(&str, &str); 10] = [
	("space_rocket_launcher", "Space rocket launcher"),
	("us_fim-92b", "Fim-92B"),
	("su_9m336", "9K333 Werba (hidden)"),
	("us_m1a1_abrams_yt_cup_2019", "M1A1 YT cup"),
	("germ_leopard_2a5_yt_cup_2019", "Leopard 2A5 YT cup"),
	("uk_challenger_ii_yt_cup_2019", "Challenger 2 YT cup"),
	("ussr_t_80u_yt_cup_2019", "T-80U YT cup"),
	("ussr_t_72b3_2017_race", "T-72B3 race"),
	("cn_ztz_96a_race", "ZTZ96A race"),
	("ussr_t_80u_race", "T-80U race"),
];

lazy_static! {
    static ref CSV_UNIT: HashMap<String, String> = {
		let wtcsv = WTCSV::new_from_path("lang/units.csv", "units").unwrap();

		let mut map = HashMap::new();

		for record  in wtcsv.records {
			map.insert(record.items[0].clone(), record.items[1].clone());
		}

		for item in EDGE_CASES {
			map.insert(item.0.to_owned(), item.1.to_owned());
		}

		map
		};

	static ref CSV_WEAPON: HashMap<String, String> = {
		let wtcsv = WTCSV::new_from_path("lang/weaponry.csv", "weaponry").unwrap();

		let mut map = HashMap::new();

		for record  in wtcsv.records {
			map.insert(record.items[0].clone(), record.items[1].clone());
		}

		for item in EDGE_CASES {
			map.insert(item.0.to_owned(), item.1.to_owned());
		}

		map
	};
}

pub enum Lang {
	Unit,
	Weapon,
}

pub fn copy_lang() {
	let options = CopyOptions {
		overwrite: true,
		skip_exist: false,
		buffer_size: 10_000,
		copy_inside: false,
		content_only: true,
		depth: 0,
	};

	fs_extra::dir::copy("./resources/cache/lang.vromfs.bin_u/lang/", "./lang/", &options).unwrap();
}

pub fn unit_to_local(target: &str, lang: &Lang) -> String {
	let to_scan = vec![
		target.to_owned(),
		format!("weapons/{}/short", target),
		format!("weapons/{}", target),
		format!("{}_shop", target),
	];

	match lang {
		Lang::Weapon => {
			for i in to_scan {
				if let Some(value) = CSV_WEAPON.get(&i) {
					return any_ascii(value);
				}
			}
		}
		Lang::Unit => {
			for i in to_scan {
				if let Some(value) = CSV_UNIT.get(&i) {
					return any_ascii(value);
				}
			}
		}
	}
	target.to_owned()
}

#[cfg(test)]
mod tests {
	use std::collections::HashSet;
	use std::fs;
	use crate::lang::{CSV_UNIT, CSV_WEAPON};

	use crate::missile::missile::Missile;
	use crate::thermal::thermals::Thermal;

	#[test]
	fn test_static_csv() {
		lazy_static::initialize(&CSV_WEAPON);
		lazy_static::initialize(&CSV_UNIT);
	}

	#[test]
	fn test_duplicate_locale_missiles() {
		let missiles: Vec<Missile> = serde_json::from_str(&fs::read_to_string("missile_index/all.json").unwrap()).unwrap();

		let mut set = HashSet::new();

		for missile in missiles.clone() {
			if !set.contains(&missile.localized) {
				set.insert(missile.localized);
			} else {
				panic!("Duplicate missile name: {} - {}", &missile.localized, &missile.name);
			}
		}
		assert_eq!(missiles.len(), set.len());
	}

	#[test]
	fn test_duplicate_locale_thermals() {
		let reference: Vec<Thermal> = serde_json::from_str(&fs::read_to_string("thermal_index/all.json").unwrap()).unwrap();

		let mut set = HashSet::new();

		for reference in reference.clone() {
			if !set.contains(&reference.localized) {
				set.insert(reference.localized);
			} else {
				panic!("Duplicate thermal name: {} - {}", &reference.localized, &reference.name);
			}
		}
		assert_eq!(reference.len(), set.len());
	}
}