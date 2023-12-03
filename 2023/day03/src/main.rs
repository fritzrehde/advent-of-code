mod part01;
mod part02;

use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let puzzle_input = read_to_string("puzzle_input.txt")?;

    println!("Part 01: {}", part01::solve(&puzzle_input)?);
    println!("Part 02: {}", part02::solve(&puzzle_input)?);

    Ok(())
}
