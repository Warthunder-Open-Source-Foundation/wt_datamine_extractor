use std::fs;
use std::process::exit;

use regex::{Regex, RegexBuilder};

pub fn extract_csv() {
	let units = fs::read("resources/cache/lang.vromfs.bin_u/lang/units.csv").unwrap();
	let weaponry = fs::read("resources/cache/lang.vromfs.bin_u/lang/units_weaponry.csv").unwrap();

	fs::write("lang/units.csv", units).unwrap();
	fs::write("lang/weaponry.csv", weaponry).unwrap();
}

pub fn unit_to_local(target: &str, path: &str) -> String {
	let unit_string = String::from_utf8(fs::read(path).unwrap()).unwrap();

	// Following this attempts to find the most accurate localization, therefore the multiple cases
	let regex = RegexBuilder::new(&target)
		.case_insensitive(true)
		.build()
		.unwrap();

	let short_regex = RegexBuilder::new(&format!("{}/short", target))
		.case_insensitive(true)
		.build()
		.unwrap();

	let shop_regex = RegexBuilder::new(&format!("{}_shop", target))
		.case_insensitive(true)
		.build()
		.unwrap();

	let mut found: usize;

	if let Some(line) = edge_case_localize_before(&target) {
		return line.to_owned();
	}

	if let Some(line) = short_regex.find(&unit_string) {
		found = line.start();
	} else if let Some(line) = shop_regex.find(&unit_string) {
		found = line.start();
	} else if let Some(line) = regex.find(&unit_string) {
		found = line.start();
	} else if let Some(line) = edge_case_localize_after(&target) {
		return line.to_owned();
	} else {
		eprintln!("target = {:?}", target);
		panic!("Cannot localize")
	}
	let untrimmed = unit_string.split_at(found).1.split(";").collect::<Vec<&str>>()[2];

	let trimmed = untrimmed.replace("\"", "").replace("\\", "");
	trimmed
}

fn edge_case_localize_after(raw: &str) -> Option<&str> {
	match raw {
		"us_fim-92b" => {
			Some("Fim-92B")
		}
		"space_rocket_launcher" => {
			Some("Space rocket launcher")
		}
		_ => {
			None
		}
	}
}

fn edge_case_localize_before(raw: &str) -> Option<&str> {
	match raw {
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
		_ => {
			None
		}
	}
}