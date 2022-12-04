use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::ops::Range;

fn main() -> io::Result<()> {
	let file = File::open("../puzzle_input.txt")?;
	let reader = BufReader::new(file);

	let points = reader.lines()
	// reader.lines()
		.map(|line| line.unwrap())
		// split line into two ranges
		.map(|line| {
			let (s1, s2) = line.split_once(',').unwrap();
			(s1.to_string(), s2.to_string())
		})
		// convert strings to ranges
		.map(|(r1, r2)| {
			let str2range = |s: String| {
				let (start, end) = s.split_once('-').unwrap();
				start.parse::<u32>().unwrap()..end.parse::<u32>().unwrap()
			};
			(str2range(r1), str2range(r2))
		})
		// which ranges fully contain each other?
		.filter(|(r1, r2)| {
			let contains = |r1: &Range::<u32>, r2: &Range::<u32>| (r1.start <= r2.start && r1.end >= r2.end);
			contains(r1, r2) || contains(r2, r1)
		})
		// .for_each(|(range1, range2)| println!("{}   {}", range1, range2));
		.count();

	println!("total points: {}", points);
	Ok(())
}
