mod part01;
mod part02;

use std::fs::read_to_string;

fn main() -> anyhow::Result<()> {
    let puzzle_input = read_to_string("puzzle_input.txt")?;

    println!("Part 01: {}", part01::solve(&puzzle_input)?);
    println!("Part 02: {}", part02::solve(&puzzle_input)?);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_part01_example() -> Result<()> {
        test_example(part01::solve, part01::example::example_details)
    }

    #[test]
    fn test_part02_example() -> Result<()> {
        test_example(part02::solve, part02::example::example_details)
    }

    fn test_example<F, G>(solver: F, example_details: G) -> Result<()>
    where
        F: Fn(&str) -> Result<String>,
        G: Fn() -> (&'static str, String),
    {
        let (puzzle_input_newline, expected_solution) = example_details();
        let puzzle_input = puzzle_input_newline
            .strip_suffix("\n")
            .expect("there should be a newline at the end generated by indoc");

        assert_eq!(expected_solution, solver(puzzle_input)?);

        Ok(())
    }
}