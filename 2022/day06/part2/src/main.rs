use std::collections::BTreeSet;
use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
	let input = read_to_string("../puzzle_input.txt")?;

	let index = input
		.chars()
		.collect::<Vec<char>>()
		// e.g. [a,b,c].iter().windows(2) -> [[a,b], [b,c]].iter()
		.windows(14)
		.position(|slice| {
			// if slice contains duplicates, no_duplicates will contain less elements than slice
			let no_duplicates: BTreeSet<&char> = slice.iter().collect();
			slice.len() == no_duplicates.len()
		})
		.unwrap();

	let count = index + 13 + 1;

	println!("total points: {}", count);

	Ok(())
}
