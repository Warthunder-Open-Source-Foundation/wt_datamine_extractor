#![allow(
clippy::missing_panics_doc,
clippy::must_use_candidate,
clippy::module_inception,
clippy::cast_sign_loss,
clippy::cast_possible_truncation,
clippy::module_name_repetitions,
clippy::or_fun_call,
clippy::missing_errors_doc,
)]

extern crate core;

pub mod missile;
pub mod thermal;
pub mod util;
pub mod lang;
pub mod shell;
pub mod explosive;
pub mod custom_loadouts;
pub mod bombs;
pub mod extraction_traits;
pub mod battle_rating;
pub mod atgm_index;

pub const MISSILES_PATH: &str = "./missile_index/missiles";