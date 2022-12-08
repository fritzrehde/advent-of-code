use std::fs::read_to_string;
use std::io;

fn main() -> io::Result<()> {
	let input = read_to_string("../puzzle_input.txt")?;

	// parse trees
	let trees: Vec<Vec<u32>> = input
		.lines()
		.map(|line| {
			line
				.chars()
				.map(|tree| tree.to_digit(10).unwrap())
				.collect()
		})
		.collect();

	// set visibility of all edges to true
	let max_y = trees.len() - 1;
	let max_x = trees[0].len() - 1;
	let mut trees_visible: Vec<Vec<bool>> = trees
		.clone()
		.iter()
		.enumerate()
		.map(|(y, tree_line)| {
			tree_line
				.iter()
				.enumerate()
				.map(|(x, _)| (y == 0 || y == max_y) || (x == 0 || x == max_x))
				.collect()
		})
		.collect();

	// check left
	trees
		.clone()
		.iter()
		.enumerate()
		.for_each(|(y, tree_line)| {
			let mut largest = 0;
			tree_line
				.iter()
				.enumerate()
				.for_each(|(x, tree)| {
					if x == 0 {
						largest = *tree;
					} else if *tree > largest {
						largest = *tree;
						trees_visible[y][x] = true;
					}
				})
		});

	// check right
	trees
		.clone()
		.iter()
		.enumerate()
		.for_each(|(y, tree_line)| {
			let mut largest = 0;
			tree_line
				.iter()
				.enumerate()
				.rev()
				.for_each(|(x, tree)| {
					if x == 0 {
						largest = *tree;
					} else if *tree > largest {
						largest = *tree;
						trees_visible[y][x] = true;
					}
				})
		});


	// check down
	for x in 0..trees.len() {
		let mut largest = 0;
		for y in 0..trees[x].len() {
			if y == 0 {
				largest = trees[y][x];
			} else if trees[y][x] > largest {
				largest = trees[y][x];
				trees_visible[y][x] = true;
			}
		}
	}

	// check up
	for x in (0..trees.len()).rev() {
		let mut largest = 0;
		for y in (0..trees[x].len()).rev() {
			if y == trees.len() - 1 {
				largest = trees[y][x];
			} else if trees[y][x] > largest {
				largest = trees[y][x];
				trees_visible[y][x] = true;
			}
		}
	}

	// dbg!(visible_trees);

	// TODO: learned about flatten
	// let total_visible_trees: usize = trees_visible
	// 	.iter()
	// 	.map(|tree_line| tree_line.iter().filter(|&&tree| tree).count())
	// 	.sum();
	let total_visible_trees: usize = trees_visible
		.iter()
		.flatten()
		.filter(|&&tree| tree)
		.count();

	println!("total points: {}", total_visible_trees);

	Ok(())
}
