use std::fs;
use crate::missiles::{Missile, SeekerType};

mod missiles;

fn main() {
	let file = fs::read("Missilelist/IR/su_r_60m.blkx").unwrap();
	let missile = Missile::new_from_file(&file, "aim-7e-2".to_owned());
	// println!("{}", String::from_utf8(file).unwrap());
	println!("{:?}", missile);
}