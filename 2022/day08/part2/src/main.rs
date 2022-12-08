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

	// let max_scenic_score = trees
	// 	.clone()
	// 	.iter()
	// 	.enumerate()
	// 	.map(|(y, tree_line)| {
	// 		tree_line.iter().enumerate().map(|(x, tree)| {
	// 			let mut scores = [0, 0, 0, 0];

	// 			// right score
	// 			for x_move in x + 1..trees[0].len() {
	// 				scores[0] += 1;
	// 				if trees[y][x_move] > *tree {
	// 					break;
	// 				}
	// 			}

	// 			// left score
	// 			for x_move in (0..x).rev() {
	// 				scores[1] += 1;
	// 				if trees[y][x_move] > *tree {
	// 					break;
	// 				}
	// 			}

	// 			// down score
	// 			for y_move in y + 1..trees.len() {
	// 				scores[2] += 1;
	// 				if trees[y_move][x] > *tree {
	// 					break;
	// 				}
	// 			}

	// 			// up score
	// 			for y_move in (0..y).rev() {
	// 				scores[3] += 1;
	// 				if trees[y_move][x] > *tree {
	// 					break;
	// 				}
	// 			}

	// 			scores.iter().product::<u32>()
	// 		})
	// 	})
	// 	.flatten()
	// 	.max()
	// 	.unwrap();

	let mut max_scenic_score = 0;

	for (y, tree_line) in trees.iter().enumerate() {
		for (x, tree) in tree_line.iter().enumerate() {
			let mut scores = [0, 0, 0, 0];

			// right score
			for x_move in x+1..trees[0].len() {
				scores[0] += 1;
				if trees[y][x_move] >= *tree {
					break;
				}
			}

			// left score
			for x_move in (0..x).rev() {
				scores[1] += 1;
				if trees[y][x_move] >= *tree {
					break;
				}
			}

			// down score
			for y_move in y+1..trees.len() {
				scores[2] += 1;
				if trees[y_move][x] >= *tree {
					break;
				}
			}

			// up score
			for y_move in (0..y).rev() {
				scores[3] += 1;
				if trees[y_move][x] >= *tree {
					break;
				}
			}

			max_scenic_score = std::cmp::max(scores.iter().product::<u32>(), max_scenic_score);
			// let scenic_score: u32 = scores.iter().product();
			// if scenic_score > max_scenic_score {
			// 	max_scenic_score = scenic_score;
			// }
			
		}
	}

	println!("total points: {}", max_scenic_score);

	Ok(())
}
