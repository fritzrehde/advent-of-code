use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use itertools::Itertools;

pub fn task1() -> io::Result<()> {
	let file = File::open("puzzle-inputs/day03.txt")?;
	let reader = BufReader::new(file);

	let points: u32 = reader.lines()
		.map(|line| line.unwrap())
		// split line into equal first and second halves
		.map(|line| {
			let mid: usize = line.len() / 2;
			(line[..mid].to_string(), line[mid..].to_string())
		})
		// find common char in both s1 and s2
		.filter_map(|(s1, s2)| {
			for c1 in s1.chars() {
				for c2 in s2.chars() {
					if c1 == c2 {
						return Some(c1);
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
		// .fold(0, |acc, points| acc + points);
		.sum();

	println!("total points: {}", points);
	Ok(())
}

pub fn task2() -> io::Result<()> {
	let file = File::open("puzzle-inputs/day03.txt")?;
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
