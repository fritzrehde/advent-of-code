mod part01;
mod part02;

fn main() -> anyhow::Result<()> {
    let puzzle_input = include_str!("../puzzle_input.txt");

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

    fn test_example<F, G, I>(solver: F, example_details: G) -> Result<()>
    where
        F: Fn(&str) -> Result<String>,
        I: Iterator<Item = (&'static str, String)>,
        G: Fn() -> I,
    {
        for (puzzle_input_newline, expected_solution) in example_details() {
            let puzzle_input = puzzle_input_newline
                .strip_suffix("\n")
                .expect("there should be a newline at the end generated by indoc");

            assert_eq!(expected_solution, solver(puzzle_input)?);
        }

        Ok(())
    }
}
