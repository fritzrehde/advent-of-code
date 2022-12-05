use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
	let input = read_to_string("../puzzle_input.txt");

	let mut stacks: [Vec<char>; 9] = [
		vec!['T', 'P', 'Z', 'C', 'S', 'L', 'Q', 'N'],
		vec!['L', 'P', 'T', 'V', 'H', 'C', 'G'],
		vec!['D', 'C', 'Z', 'F'],
		vec!['G', 'W', 'T', 'D', 'L', 'M', 'V', 'C'],
		vec!['P', 'W', 'C'],
		vec!['P', 'F', 'J', 'D', 'C', 'T', 'S', 'Z'],
		vec!['V', 'W', 'G', 'B', 'D'],
		vec!['N', 'J', 'S', 'Q', 'H', 'W'],
		vec!['R', 'C', 'Q', 'F', 'S', 'L', 'V'],
	];

	input?.lines()
		.filter(|line| line.contains("move"))
		// TODO: this is terrible rust code, hopefully I'll have more time in the future to fix it
		// extract operations from line
		.map(|line| {
			let raw: Vec<&str> = line.split(" ").collect();
			let num = |i| {
				// will panic if invalid index
				let s: &str = raw[i];
				s.parse::<usize>().unwrap()
			};
			(num(1), num(3), num(5))
		})
		.for_each(|(repeat, src, dst)| {
			for _ in 0..repeat {
				if let Some(popped) = stacks[src-1].pop() {
					stacks[dst-1].push(popped);
				}
			}
		});

		for stack in stacks {
			print!("{}", stack.last().unwrap());
		}

	Ok(())
}
