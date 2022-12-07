use std::fs::read_to_string;
use std::io;
use std::collections::HashMap;

// enum Files {
// 	Dir(String),
// 	// File(String, usize),
// 	File {
// 		name: String,
// 		size: usize,
// 	},
// }

// enum Cmd {
// 	Cd(String),
// 	Ls,
// }

#[derive(Clone)]
struct Dir {
	name: String,
	size: usize,
	parent: Option<Box<Dir>>,
	children: HashMap<String, Box<Dir>>,
}

impl Dir {
	fn new(name: &str, parent: Option<Box<Dir>>) -> Dir {
		Dir {
			name: name.to_string(),
			size: 0,
			parent,
			children: HashMap::new(),
		}
	}
}

fn main() -> io::Result<()> {
	let input = read_to_string("../puzzle_input.txt")?;

	let max_dir_size = 100000;
	let mut total_size = 0;
	let mut current_dir = Box::new(Dir::new("/", None));

	// for line in input.lines() {
	// 	match line.split(" ").collect::<Vec<&str>>()[..] {
	// 		["$", "ls"] => continue,
	// 		["$", "cd", ".."] => {
	// 			if current_dir.size <= max_dir_size {
	// 				total_size += current_dir.size;
	// 			}
	// 			current_dir.size += current_dir.size;
	// 			// current_dir = *current_dir.parent.unwrap();
	// 			current_dir = current_dir.parent.unwrap();
	// 		},
	// 		["$", "cd", dir] => current_dir = *current_dir.children.get(dir).unwrap(),
	// 		["dir", dir_name] => {
	// 			current_dir.children.insert(dir_name.to_string(), Box::new(Dir::new(dir_name, current_dir.parent)));
	// 		},
	// 		_ => (),

	// 	};
	// }


	println!("total size: {}", total_size);

	// parse input
	// let commands: Vec<Cmd> = input
	// 	.split("$ ")
	// 	.filter(|line| !line.is_empty())
	// 	.inspect(|v| {
	// 		dbg!(v);
	// 	})
	// 	.map(|cmd| {
	// 		let mut lines = cmd.lines();
	// 		if lines.next().unwrap() == "cd" {
	// 			Cmd::Cd(lines.next().unwrap().to_string())
	// 		}

	// 	})
	// 	.count();


	// println!("total points: {}", points);

	Ok(())
}
