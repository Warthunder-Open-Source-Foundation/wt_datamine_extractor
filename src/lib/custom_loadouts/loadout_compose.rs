use crate::custom_loadouts::custom_loadouts::{CustomLoadout, Pylon};
use crate::custom_loadouts::loadout_compose::CLError::{BadSelection, TgpNotSatisfied};

#[derive(Debug, PartialEq, Clone)]
pub struct CLComposition {
	pub total_mass: f64,
	pub max_imbalance: f64,
	pub dist_loads: (f64, f64, f64),
}


type ExcessMass = f64;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum CLError {
	// Total weight of the aircraft's carry capacity exceeded
	TooHighTotalMass(ExcessMass),

	// Negative value means left wing too high, positive means right wing too high
	TooHighImbalance(ExcessMass),

	// 0 means load is OK, any other value means load for specific wing exceeded
	WingExcess((ExcessMass, ExcessMass)),

	// Bad pylon selection (out of bounds or the like)
	BadSelection(usize),

	// Targeting pod not provided, with failing slot left, required slot right
	TgpNotSatisfied((usize, usize)),

	// Current exception with the Kfir C.2
	NoExemptCenter,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum WingState {
	LeftWing,
	Center,
	RightWing,
}

impl CustomLoadout {
	pub fn compose_loadout(&self, selection: &[usize]) -> Result<CLComposition, Vec<CLError>> {
		// errors in multiple places will first be collected and returned at the very end
		let mut errs: Vec<CLError> = Vec::new();

		let mut totals: (f64, f64, f64) = (0.0, 0.0, 0.0);

		// states are
		// 0 = left wing
		// 1 = center (except from imbalance)
		// 2 = right wing
		let mut wing_state = WingState::LeftWing;

		// This loop only begins adding up masses, error checking comes after
		for (i, pylon) in self.pylons.iter().enumerate() {
			let mut compute_pylon = |pylon: &Pylon, totals: &mut f64| {
				// A selection of 0 means empty pylon, not adding any value, this means the array access needs to be subtracted by that extra
				if selection[i] == 0 {
					return;
				}
				if let Some(item) = pylon.weapons.get(selection[i] - 1) {
					*totals += item.total_mass;

					if let Some(depend) = &item.depend_on {
						// Slot which requires named item
						let required_slot = depend.slot as usize - 1;

						// The slot that the user has chosen
						#[allow(clippy::manual_saturating_arithmetic)]
							let user_selected_slot = selection[required_slot].checked_sub(1).unwrap_or(0);

						// The slot the pylon actually needs
						let user_named_selection = &self.pylons[required_slot].weapons[user_selected_slot].name;

						if &depend.name != user_named_selection {
							errs.push(TgpNotSatisfied((0, 0)));
						}
					};
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

			// Check if any exempt center was detected
			if i == self.pylons.len() - 1 && wing_state != WingState::RightWing {
				errs.push(CLError::NoExemptCenter);
			}
		}

		let delta_total = totals.0 + totals.1 + totals.2;
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
			let sign = (totals.0 - totals.2).is_sign_positive();
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