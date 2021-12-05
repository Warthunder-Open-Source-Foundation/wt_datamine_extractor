use std::collections::HashMap;
use std::fs;
use any_ascii::any_ascii;
use lazy_static::lazy_static;

const EDGE_CASES: [(&str, &str); 10] = [
	("space_rocket_launcher", "Space rocket launcher"),
	("us_fim-92b","Fim-92B"),
	("su_9m336","9K333 Werba (hidden)"),
	("us_m1a1_abrams_yt_cup_2019","M1A1 YT cup"),
	("germ_leopard_2a5_yt_cup_2019","Leopard 2A5 YT cup"),
	("uk_challenger_ii_yt_cup_2019","Challenger 2 YT cup"),
	("ussr_t_80u_yt_cup_2019","T-80U YT cup"),
	("ussr_t_72b3_2017_race","T-72B3 race"),
	("cn_ztz_96a_race","ZTZ96A race"),
	("ussr_t_80u_race","T-80U race"),
];

lazy_static! {
    static ref CSV_UNIT: HashMap<String, String> = {
        let mut raw_csv = csv::ReaderBuilder::new().delimiter(b';').from_path("lang/units.csv").unwrap();

		let parsed = raw_csv.records().map(|x| {
			let u = x.unwrap();
			(u.get(0).unwrap().to_owned(), u.get(1).unwrap().to_owned())
		}).collect::<Vec<(String,String)>>();

		let mut map: HashMap<String, String> = HashMap::new();
		for item in parsed {
			map.insert(item.0, item.1);
		}
		for item in EDGE_CASES {
			map.insert(item.0.to_owned(), item.1.to_owned());
		}
		map
		};

	static ref CSV_WEAPON: HashMap<String, String> = {
        let mut raw_csv = csv::ReaderBuilder::new().delimiter(b';').from_path("lang/weaponry.csv").unwrap();

		let parsed = raw_csv.records().map(|x| {
			let u = x.unwrap();
			(u.get(0).unwrap().to_owned(), u.get(1).unwrap().to_owned())
		}).collect::<Vec<(String,String)>>();

		let mut map: HashMap<String, String> = HashMap::new();
		for item in parsed {
			map.insert(item.0, item.1);
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

pub fn extract_csv() {
	let units = fs::read("resources/cache/lang.vromfs.bin_u/lang/units.csv").unwrap();
	let weaponry = fs::read("resources/cache/lang.vromfs.bin_u/lang/units_weaponry.csv").unwrap();

	fs::write("lang/units.csv", units).unwrap();
	fs::write("lang/weaponry.csv", weaponry).unwrap();
}

pub fn unit_to_local(target: &str, lang: Lang) -> String {

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
		},
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

	use crate::missile::missile::Missile;
	use crate::thermal::thermals::Thermal;

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