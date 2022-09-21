use std::fs::read;
use std::io::Read;
use csv::Terminator;
use serde::{Deserialize, Serialize};
use crate::extraction_traits::core::ExtractCore;

pub const DELIM: u8 = b';';
pub const TERMIN: u8 = b'\n';

pub trait CsvSerialize<T: ExtractCore = Self> {
	fn to_csv(items: Vec<Self>, path: &str) -> Result<(), csv::Error> where Self: Sized + Serialize {
		let mut writer = csv::WriterBuilder::default()
			.delimiter(b';')
			.terminator(Terminator::Any(TERMIN))
			.has_headers(false)
			.from_path(path)?;

		for item in items {
			writer.serialize(item)?;
		}

		Ok(())
	}
}