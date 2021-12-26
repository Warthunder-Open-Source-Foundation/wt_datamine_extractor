#[cfg(test)]
mod tests {
	use lazy_static::lazy_static;

	use crate::missile::extract_missiles::KnownMissiles;
	use crate::missile::missile::Missile;

	#[cfg(test)]
	lazy_static! {
    static ref MISSILES: Vec<Missile> = {
			let file = std::fs::read_to_string("missile_index/known.json").unwrap();
			let known_missiles: KnownMissiles = serde_json::from_str(&file).unwrap();
        let missiles = Missile::generate_from_index(&known_missiles);
			missiles
		};
	}

	#[test]
	fn has_seeker_fov() {
		for missile in MISSILES.iter() {
			if missile.fov == 0.0 {
				panic!("{} has no fov", missile.name)
			}
		}
	}
}