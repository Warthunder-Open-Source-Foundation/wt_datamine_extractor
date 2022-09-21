use serde::Serialize;

use crate::extraction_traits::core::ExtractCore;
use crate::extraction_traits::known::{Index, KnownItem, OwnedIndex};

// Used for emulating functionality for WIP or on extracting structs
#[derive(Serialize, Default)]
pub struct DummyCore;


impl ExtractCore for DummyCore {
	fn generate_from_index(_: impl KnownItem, _: &str) -> Vec<Self> where Self: Sized {
		unimplemented!("Dummy fn should not be called")
	}

	fn write_all(_: Vec<Self>, _: &str) where Self: Sized, Self: Serialize {
		unimplemented!("Dummy fn should not be called")
	}

	fn new_from_file(_: &[u8], _: String) -> Self {
		unimplemented!("Dummy fn should not be called")
	}

	fn sort(_: &mut Vec<Self>) where Self: Sized {
		unimplemented!("Dummy fn should not be called")
	}
}

impl KnownItem for DummyCore {
	fn generate_index(_: &str) -> Self where Self: Default {
		unimplemented!("Dummy fn should not be called")
	}

	fn write_index(self, _: &str) -> Self where Self: Serialize + Sized {
		unimplemented!("Dummy fn should not be called")
	}

	fn copy_index_to_folder(self, _: &str, _: &str) -> Self where Self: Sized {
		unimplemented!("Dummy fn should not be called")
	}

	fn push_index(&mut self, _: OwnedIndex) {
		unimplemented!("Dummy fn should not be called")
	}

	fn get_index(&self) -> Index {
		unimplemented!("Dummy fn should not be called")
	}
}