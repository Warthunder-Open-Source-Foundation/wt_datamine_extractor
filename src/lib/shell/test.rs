#[cfg(test)]
mod tests {
	use std::collections::HashSet;

	use lazy_static::lazy_static;
	use crate::shell::explosive::ExplosiveType;

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
			// They fucking classified the sonic wave launcher as heat-fs
			if shell.shell_type == ShellType::HeatFs && shell.name != "sonicWave" {
				assert!(!shell.explosive.is_inert());
			}
		}
	}

	#[test]
	fn he_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::He {
				assert!(!shell.explosive.is_inert());
			}
		}
	}

	#[test]
	fn aphe_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::ApHe {
				assert!(!shell.explosive.is_inert());
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
				assert!(!shell.explosive.is_inert());
			}
		}
	}

	#[test]
	fn heat_he_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::Heat {
				assert!(!shell.explosive.is_inert());
			}
		}
	}

	#[test]
	fn sap_he_i_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::SapHei {
				assert!(!shell.explosive.is_inert());
			}
		}
	}

	#[test]
	fn sam_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::Sam {
				assert!(!shell.explosive.is_inert());
			}
		}
	}

	#[test]
	fn atgm_he_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::AtgmHe {
				assert!(!shell.explosive.is_inert());
			}
		}
	}

	#[test]
	fn shrapnel_he_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::Shrapnel {
				assert!(!shell.explosive.is_inert());
			}
		}
	}

	#[test]
	fn aam_he_has_tnt_and_type() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::Aam {
				assert!(!shell.explosive.is_inert());
			}
		}
	}

	// testing if shell DOESNT have HE like it shouldn't

	#[test]
	fn ap_fs_ds_doesnt_have_tnt() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::ApFsDs {
				assert!(shell.explosive.is_inert());
			}
		}
	}

	#[test]
	fn practice_doesnt_have_tnt() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::Practice {
				assert!(shell.explosive.is_inert());
			}
		}
	}

	#[test]
	fn ap_cr_doesnt_have_tnt() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::ApCr {
				assert!(shell.explosive.is_inert());
			}
		}
	}

	#[test]
	fn solid_ap_doesnt_have_tnt() {
		for shell in SHELLS.iter() {
			if shell.shell_type == ShellType::ApSolid {
				assert!(shell.explosive.is_inert());
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
	fn no_duplicate_shell() {
		let mut map = HashSet::new();
		for shell in SHELLS.iter() {
			if map.get(shell).is_some() {
				panic!("Duplicate shell {}", shell.name);
			};
			map.insert(shell.clone());
		}
	}
}
