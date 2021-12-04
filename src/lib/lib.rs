#![allow(
clippy::missing_panics_doc,
clippy::must_use_candidate,
clippy::module_inception
)]

pub mod missile;
pub mod thermal;
pub mod util;
pub mod lang;
pub mod shell;

pub const MISSILES_PATH: &str = "./missile_index/missiles";