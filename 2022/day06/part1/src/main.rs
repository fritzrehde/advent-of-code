#![feature(slice_partition_dedup)]
use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
	let input = read_to_string("../puzzle_input.txt")?;

	let index = input
		.chars()
		.collect::<Vec<char>>()
		// e.g. [a,b,c].iter().windows(2) -> [[a,b], [b,c]].iter()
		.windows(4)
		.position(|four| {
			// let (_, duplicated) = four.partition_dedup();
			// duplicated.is_empty()

			// if four contains duplicates, no_duplicates will contain less elements than four
			let no_duplicates: BTreeSet<&char> = four.iter().collect();
			four.len() == no_duplicates.len()
		})
		.unwrap();

	// get last element of four slice [0,1,2,3], starts at count 1 instead of index 0
	let count = index + 3 + 1;

	println!("total points: {}", count);

	Ok(())
}
