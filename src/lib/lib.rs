#![allow(
clippy::missing_panics_doc,
clippy::must_use_candidate,
clippy::module_inception,
clippy::cast_sign_loss,
clippy::cast_possible_truncation,
clippy::module_name_repetitions
)]

pub mod missile;
pub mod thermal;
pub mod util;
pub mod lang;
pub mod shell;
pub mod explosive;

pub const MISSILES_PATH: &str = "./missile_index/missiles";