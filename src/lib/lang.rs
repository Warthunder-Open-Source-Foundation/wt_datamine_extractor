use std::fs;
use any_ascii::any_ascii;

pub fn extract_csv() {
	let units = fs::read("resources/cache/lang.vromfs.bin_u/lang/units.csv").unwrap();
	let weaponry = fs::read("resources/cache/lang.vromfs.bin_u/lang/units_weaponry.csv").unwrap();

	fs::write("lang/units.csv", units).unwrap();
	fs::write("lang/weaponry.csv", weaponry).unwrap();
}

pub fn unit_to_local(target: &str, path: &str) -> String {
	let mut raw_csv = csv::ReaderBuilder::new()
		.delimiter(b';')
		.from_path(path).unwrap();

	let parsed = raw_csv.records().map(|x| {
		let u = x.unwrap();
		(u.get(0).unwrap().to_owned(), u.get(1).unwrap().to_owned())
	}).collect::<Vec<(String,String)>>();


	let to_scan = vec![
		format!("weapons/{}/short", target),
		format!("weapons/{}", target),
		format!("{}_shop", target),
	];

	if let Some(value) = edge_case_localize(target) {
		return value.to_owned()
	}

	for i in to_scan {
		for item in &parsed {
			if item.0.to_lowercase() == i.to_lowercase() {
				let possibly_non_ascii = &item.1;
				return any_ascii(possibly_non_ascii)
			}
		}
	}
	target.to_owned()
}

// Duplicates / special items go here
fn edge_case_localize(raw: &str) -> Option<&str> {
	match raw {
		"space_rocket_launcher" => {
			Some("Space rocket launcher")
		}
		"us_fim-92b" => {
			Some("Fim-92B")
		}
		"su_9m336" => {
			Some("9K333 Werba (hidden)")
		}
		"us_m1a1_abrams_yt_cup_2019" => {
			Some("M1A1 YT cup")
		}
		"germ_leopard_2a5_yt_cup_2019" => {
			Some("Leopard 2A5 YT cup")
		}
		"uk_challenger_ii_yt_cup_2019" => {
			Some("Challenger 2 YT cup")
		}
		"ussr_t_80u_yt_cup_2019" => {
			Some("T-80U YT cup")
		}
		"ussr_t_72b3_2017_race" => {
			Some("T-72B3 race")
		}
		"cn_ztz_96a_race" => {
			Some("ZTZ96A race")
		}
		"ussr_t_80u_race" => {
			Some("T-80U race")
		}
		_ => {
			None
		}
	}
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