use std::cmp::{max, min};
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
	let file = File::open("../puzzle_input.txt")?;
	let reader = BufReader::new(file);

	let points = reader
		.lines()
		// reader.lines()
		.map(|line| line.unwrap())
		// split line into two ranges
		.map(|line| {
			let (s1, s2) = line.split_once(',').unwrap();
			(s1.to_string(), s2.to_string())
		})
		// convert strings to ranges
		.map(|(s1, s2)| {
			let str2range = |s: String| {
				let (start, end) = s.split_once('-').unwrap();
				// start.parse::<u32>().unwrap()..(end.parse::<u32>().unwrap()+1)
				start.parse::<u32>().unwrap()..=end.parse::<u32>().unwrap()
			};
			(str2range(s1), str2range(s2))
		})
		// which ranges overlap with each other?
		.filter(|(r1, r2)| max(r1.start(), r2.start()) <= min(r1.end(), r2.end()))
		.count();

	println!("total points: {}", points);
	Ok(())
}
