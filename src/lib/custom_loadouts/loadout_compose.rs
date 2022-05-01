use crate::custom_loadouts::custom_loadouts::{CustomLoadout, Pylon};
use crate::custom_loadouts::loadout_compose::CLError::BadSelection;

#[derive(Debug, PartialEq)]
pub struct CLComposition {
	pub total_mass: f64,
	pub max_imbalance: f64,
	pub dist_loads: (f64, f64, f64),
}


type ExcessMass = f64;

#[derive(Debug, PartialEq)]
pub enum CLError {
	// Total weight of the aircraft's carry capacity exceeded
	TooHighTotalMass(ExcessMass),

	// Negative value means left wing too high, positive means right wing too high
	TooHighImbalance(ExcessMass),

	// 0 means load is OK, any other value means load for specific wing exceeded
	WingExcess((ExcessMass, ExcessMass)),

	// Bad pylon selection (out of bounds or the like)
	BadSelection(usize),

	NoExemptCenter,
}

#[derive(Eq, PartialEq)]
enum WingState {
	LeftWing,
	Center,
	RightWing,
}

impl CustomLoadout {
	pub fn compose_loadout(&self, selection: &[usize]) -> Result<CLComposition, Vec<CLError>> {
		// errors in multiple places will first be collected and returned at the very end
		let mut errs: Vec<CLError> = Vec::new();


		// should never mutate but is marked as such for compilers sake
		let mut misc = 0.0;

		let mut totals: (f64, f64, f64) = (0.0, 0.0, 0.0);

		// states are
		// 0 = left wing
		// 1 = center (except from imbalance)
		// 2 = right wing
		let mut wing_state = WingState::LeftWing;

		// This loop only begins adding up masses, error checking comes after
		for (i, pylon) in self.pylons.iter().enumerate() {
			if i == 0 {
				misc = if let Some(item) = pylon.weapons.get(selection[i]) {
					item.total_mass
				} else {
					// Abort right away as this **has** to work
					errs.push(BadSelection(0));
					return Err(errs);
				}
			}

			let mut compute_pylon = |pylon: &Pylon, totals: &mut f64| {
				// A selection of 0 means empty pylon, not adding any value, this means the array access needs to be subtracted by that extra
				if selection[i] == 0 {
					return;
				}
				if let Some(item) = pylon.weapons.get(selection[i] - 1) {
					*totals += item.total_mass;
				} else {
					errs.push(BadSelection(i));
				}
			};

			// Find out what wing state were on, constellation usually goes as such:
			// (misc pylon) (L) (L) (Center Exempt) (CE) (CE) (R) (R)
			if pylon.exempt_from_imbalance && i != 0 {
				wing_state = WingState::Center;
			} else if wing_state == WingState::Center && !pylon.exempt_from_imbalance {
				wing_state = WingState::RightWing;
			}

			// Apply computed weights
			match wing_state {
				WingState::LeftWing => {
					compute_pylon(pylon, &mut totals.0);
				}
				WingState::Center => {
					compute_pylon(pylon, &mut totals.1);
				}
				WingState::RightWing => {
					compute_pylon(pylon, &mut totals.2);
				}
			}

			if i == self.pylons.len() - 1 && wing_state != WingState::RightWing {
				errs.push(CLError::NoExemptCenter);
			}
		}

		let delta_total = totals.0 + totals.1 + totals.2 + misc;
		if delta_total > self.max_load {
			errs.push(CLError::TooHighTotalMass(delta_total - self.max_load));
		}

		if totals.0 > self.max_wing_load.0 {
			errs.push(CLError::WingExcess((totals.0 - self.max_wing_load.0, 0.0)));
		}

		if totals.2 > self.max_wing_load.1 {
			errs.push(CLError::WingExcess((0.0, totals.2 - self.max_wing_load.1)));
		}

		if (totals.0 - totals.2).abs() > self.max_imbalance {
			// This might be way too much code for such a simple thing but eh, go make a pull request if you bother to do so
			let offset = (totals.0 - totals.2).abs() - self.max_imbalance;
			let sign = ( totals.0 - totals.2).is_sign_positive();
			let bad = if sign {
				-offset
			} else {
				offset
			};
			errs.push(CLError::TooHighImbalance(bad));
		}


		if errs.is_empty() {
			Ok(CLComposition {
				total_mass: delta_total,
				max_imbalance: (totals.0 - totals.2).abs(),
				dist_loads: totals,
			})
		} else {
			Err(errs)
		}
	}
}

#[cfg(test)]
mod tests {
	use std::fs;
	use crate::custom_loadouts::custom_loadouts::CustomLoadout;

	#[test]
	fn test_clean() {
		let reader = fs::read("custom_loadouts/aircraft/a_10a_early.blkx").unwrap();
		let loadouts = CustomLoadout::new_from_file(&reader, "a_10a_early".to_owned());

		if loadouts.compose_loadout(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]).is_err() {
			panic!("uh oh")
		}
	}

	#[test]
	fn test_max_total() {
		let reader = fs::read("custom_loadouts/aircraft/a_10a_early.blkx").unwrap();
		let loadouts = CustomLoadout::new_from_file(&reader, "a_10a_early".to_owned());

		if loadouts.compose_loadout(&[0, 2, 2, 6, 6, 2, 2, 6, 6, 2, 2]).is_ok() {
			panic!("uh oh")
		}
	}

	#[test]
	fn test_imbalance_right_wing() {
		let reader = fs::read("custom_loadouts/aircraft/a_10a_early.blkx").unwrap();
		let loadouts = CustomLoadout::new_from_file(&reader, "a_10a_early".to_owned());

		if loadouts.compose_loadout(&[0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 0]).is_ok() {
			panic!("uh oh")
		}
	}

	#[test]
	fn test_imbalance_left_wing() {
		let reader = fs::read("custom_loadouts/aircraft/a_10a_early.blkx").unwrap();
		let loadouts = CustomLoadout::new_from_file(&reader, "a_10a_early".to_owned());

		if loadouts.compose_loadout(&[0, 0, 0, 6, 0, 0, 0, 0, 0, 0, 0]).is_ok() {
			panic!("uh oh")
		}
	}

	#[test]
	fn test_exempt_bad() {
		let reader = fs::read("custom_loadouts/aircraft/kfir_c2.blkx").unwrap();
		let loadouts = CustomLoadout::new_from_file(&reader, "kfir_c2".to_owned());

		if loadouts.compose_loadout(&[0, 0, 0, 0, 0, 0, 0, 0]).is_ok() {
			panic!("uh oh")
		}
	}
}