use std::str;

#[derive(Debug)]
struct HashableStrings(Vec<HashableString>);

impl str::FromStr for HashableStrings {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hashable_strings = s
            .split(',')
            .map(|hashable| hashable.parse::<HashableString>())
            .collect::<Result<_, _>>()?;
        Ok(Self(hashable_strings))
    }
}

#[derive(Debug)]
struct HashableString(Vec<char>);

impl str::FromStr for HashableString {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hashable_string = s.chars().collect();
        Ok(Self(hashable_string))
    }
}

impl HashableString {
    fn hash(&self) -> usize {
        let mut current_value = 0;
        for c in self.0.iter() {
            let ascii_code = *c as usize;
            current_value += ascii_code;
            current_value *= 17;
            current_value %= 256;
        }
        current_value
    }
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let hashable_strings: HashableStrings = puzzle_input.parse()?;

    let sum_of_hashes: usize = hashable_strings.0.iter().map(HashableString::hash).sum();

    Ok(sum_of_hashes.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide the example details as `(puzzle input, expected solution)`.
    pub fn example_details() -> (&'static str, String) {
        let puzzle_input = indoc! {"
            rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        "};
        let expected_solution = 1320;
        (puzzle_input, expected_solution.to_string())
    }
}
