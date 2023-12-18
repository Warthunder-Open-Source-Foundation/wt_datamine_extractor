#[cfg(test)]
mod tests {
	use lazy_static::lazy_static;

	use crate::missile::missile::Missile;

	#[cfg(test)]
	lazy_static! {
    static ref MISSILES: Vec<Missile> = {
			let file = std::fs::read_to_string("missile_index/all.json").unwrap();
			let missiles: Vec<Missile> = serde_json::from_str(&file).unwrap();
			missiles
		};
	}

	#[test]
	fn has_name() {
		for missile in MISSILES.iter() {
			if missile.name.is_empty() {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_localized_name() {
		for missile in MISSILES.iter() {
			if missile.localized.is_empty() {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_mass() {
		for missile in MISSILES.iter() {
			if missile.mass == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_end_mass() {
		for missile in MISSILES.iter() {
			if missile.mass_end == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_caliber() {
		for missile in MISSILES.iter() {
			if missile.caliber == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}

	/*
	Disabled because mistral breaks this "rule"
	#[test]
	fn has_force0_and_time_fire0() {
		for missile in MISSILES.iter() {
			if missile.force0 == 0.0 || missile.timefire0 == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}
	 */

	#[test]
	fn time_fire1_require_force1() {
		for missile in MISSILES.iter() {
			if missile.force1 != 0.0 {
				if missile.timefire1 == 0.0 {
					panic!("{}", missile.name)
				}
			}
			if missile.timefire1 != 0.0 {
				if missile.force1 == 0.0 {
					panic!("{}", missile.name)
				}
			}
		}
	}

	#[test]
	fn has_cxk() {
		for missile in MISSILES.iter() {
			if missile.cxk == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_cx() {
		for missile in MISSILES.iter() {
			if missile.dragcx == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_time_life() {
		for missile in MISSILES.iter() {
			if missile.timelife == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_exp_mass() {
		for missile in MISSILES.iter() {
			if missile.exp_mass == 0 {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_lock_angle_max() {
		for missile in MISSILES.iter() {
			if missile.lockanglemax == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_angle_max() {
		for missile in MISSILES.iter() {
			if missile.anglemax == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_warm_up_time() {
		for missile in MISSILES.iter() {
			if missile.warmuptime == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_work_time() {
		for missile in MISSILES.iter() {
			if missile.worktime == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}

	#[test]
	fn has_delta_v() {
		for missile in MISSILES.iter() {
			if missile.deltav == 0.0 {
				panic!("{}", missile.name)
			}
		}
	}
}