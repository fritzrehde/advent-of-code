use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use itertools::Itertools;

fn main() -> io::Result<()> {
	let file = File::open("../puzzle_input.txt")?;
	let reader = BufReader::new(file);

	let points: u32 = reader.lines()
	// reader.lines()
		.map(|line| line.unwrap())
		// itertools: transform [s1, s2, ...] into [(s1, s2, s3), (s4, s5, s6), ...]
		.tuples()
		// find common char in s1, s2 and s3
		.filter_map(|(s1, s2, s3)| {
			for c1 in s1.chars() {
				for c2 in s2.chars() {
					for c3 in s3.chars() {
						if c1 == c2 && c1 == c3 {
							return Some(c1);
						}
					}
				}
			}
			None
		})
		// map each char to points
		.filter_map(|c| {
			if c.is_lowercase() {
				Some(1 + (c as u32) - ('a' as u32))
			} else if c.is_uppercase() {
				Some(27 + (c as u32) - ('A' as u32))
			} else {
				None
			}
		})
		.sum();

	println!("total points: {}", points);
	Ok(())
}
