#[cfg(test)]
mod tests {
	use std::collections::{HashSet};

	use lazy_static::lazy_static;

	use crate::shell::known_shells::KnownShells;
	use crate::shell::shells::{Shell, ShellType};

	#[cfg(test)]
	lazy_static! {
    static ref SHELLS: Vec<Shell> = {
        let shells = Shell::generate_from_index(&KnownShells::from_file());
			shells
		};
	}

	// Testing IF something has explosive properly

	#[test]
	fn heat_fs_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::HeatFs {
				// println!("{}", shell.name);
				assert_ne!(shell.explosive.0, "");
				assert_ne!(shell.explosive.1, 0);
			}
		}
	}

	#[test]
	fn he_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::He {
				// println!("{}", shell.name);
				assert_ne!(shell.explosive.0, "");
				assert_ne!(shell.explosive.1, 0);
			}
		}
	}

	#[test]
	fn aphe_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::ApHe {
				// println!("{}", shell.name);
				assert_ne!(shell.explosive.0, "");
				assert_ne!(shell.explosive.1, 0);
			}
		}
	}

	// #[test]
	// fn atgm_has_tnt_and_type() {
	// 	let shells = Shell::generate_from_index(&KnownShells::generate_index());
	//
	// 	for shell in shells {
	// 		if shell.shell_type == ShellType::Atgm {
	// 			println!("{}", shell.name);
	// 			assert_ne!(shell.explosive.0, "");
	// 			assert_ne!(shell.explosive.1, 0.0);
	// 		}
	// 	}
	// }

	#[test]
	fn hesh_he_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::Hesh {
				// println!("{}", shell.name);
				assert_ne!(shell.explosive.0, "");
				assert_ne!(shell.explosive.1, 0);
			}
		}
	}

	#[test]
	fn heat_he_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::Heat {
				// println!("{}", shell.name);
				assert_ne!(shell.explosive.0, "");
				assert_ne!(shell.explosive.1, 0);
			}
		}
	}

	#[test]
	fn sap_he_i_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::SapHei {
				// println!("{}", shell.name);
				assert_ne!(shell.explosive.0, "");
				assert_ne!(shell.explosive.1, 0);
			}
		}
	}

	#[test]
	fn sam_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::Sam {
				// println!("{}", shell.name);
				assert_ne!(shell.explosive.0, "");
				assert_ne!(shell.explosive.1, 0);
			}
		}
	}

	#[test]
	fn atgm_he_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::AtgmHe {
				// println!("{}", shell.name);
				assert_ne!(shell.explosive.0, "");
				assert_ne!(shell.explosive.1, 0);
			}
		}
	}

	#[test]
	fn shrapnel_he_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::Shrapnel {
				// println!("{}", shell.name);
				assert_ne!(shell.explosive.0, "");
				assert_ne!(shell.explosive.1, 0);
			}
		}
	}

	#[test]
	fn aam_he_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::Aam {
				// println!("{}", shell.name);
				assert_ne!(shell.explosive.0, "");
				assert_ne!(shell.explosive.1, 0);
			}
		}
	}

	// testing if shell DOESNT have HE like it shouldn't

	#[test]
	fn ap_fs_ds_doesnt_have_tnt() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::ApFsDs {
				// println!("{}", shell.name);
				assert_eq!(shell.explosive.0, "");
				assert_eq!(shell.explosive.1, 0);
			}
		}
	}

	#[test]
	fn practice_doesnt_have_tnt() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::Practice {
				// println!("{}", shell.name);
				assert_eq!(shell.explosive.0, "");
				assert_eq!(shell.explosive.1, 0);
			}
		}
	}

	#[test]
	fn ap_cr_doesnt_have_tnt() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::ApCr {
				// println!("{}", shell.name);
				assert_eq!(shell.explosive.0, "");
				assert_eq!(shell.explosive.1, 0);
			}
		}
	}

	#[test]
	fn solid_ap_doesnt_have_tnt() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::ApSolid {
				// println!("{}", shell.name);
				assert_eq!(shell.explosive.0, "");
				assert_eq!(shell.explosive.1, 0);
			}
		}
	}

	// GP tests

	#[test]
	fn shell_has_caliber() {
		for shell in SHELLS.iter() {
			// println!("{}", shell.name);
			assert_ne!(shell.caliber, 0);
			assert_ne!(shell.true_caliber, 0);
		}
	}

	#[test]
	fn shell_has_parent_gun() {
		for shell in SHELLS.iter() {
			// println!("{}", shell.name);
			assert_ne!(shell.parent_guns.len(), 0);
		}
	}

	#[test]
	fn no_duplicate_shell_hash() {
		let mut map = HashSet::new();
		for shell in SHELLS.iter() {
			if map.get(&shell.hash).is_some() {
				panic!("Duplicate shell {}", shell.name);
			};
			map.insert(&shell.hash);
		}
	}
}
