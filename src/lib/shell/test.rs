#[cfg(test)]
mod tests {
	use crate::shell::known_shells::KnownShells;
	use crate::shell::shells::{Shell, ShellType};

	#[test]
	fn aphe_has_tnt_and_type() {
		let shells = Shell::generate_from_index(&KnownShells::generate_index());

		for shell in shells {
			if shell.shell_type == ShellType::ApHe {
				// println!("{}", shell.name);
				assert_ne!(shell.explosive.0, "");
				assert_ne!(shell.explosive.1, 0.0);
			}
		}
	}
}
