use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::RangeInclusive;

fn main() -> io::Result<()> {
	let file = File::open("../puzzle_input.txt")?;
	let reader = BufReader::new(file);

	let points = reader.lines()
		.map(|line| line.unwrap())
		// split line and convert into ranges
		.map(|line| {
			let (s1, s2) = line.split_once(',').unwrap();
			let str2range = |s: &str| {
				let (start, end) = s.split_once('-').unwrap();
				start.parse::<u32>().unwrap()..=end.parse::<u32>().unwrap()
			};
			(str2range(s1), str2range(s2))
		})
		// which ranges fully contain each other?
		.filter(|(r1, r2)| {
			let contains = |r1: &RangeInclusive::<u32>, r2: &RangeInclusive::<u32>| (r1.start() <= r2.start() && r1.end() >= r2.end());
			contains(r1, r2) || contains(r2, r1)
		})
		.count();

	println!("total points: {}", points);
	Ok(())
}
