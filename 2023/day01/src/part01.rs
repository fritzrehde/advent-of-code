use derive_more::Into;
use std::str::FromStr;

/// The calibration value can be found by combining the first digit and the
/// last digit (in that order) to form a single two-digit number.
#[derive(Into)]
struct CalibrationValue(u32);

impl FromStr for CalibrationValue {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_digit = s
            .chars()
            .find_map(|c| c.to_digit(10))
            .expect("there should always be a first digit");

        let last_digit = s
            .chars()
            .rev()
            .find_map(|c| c.to_digit(10))
            .expect("there should always be a last digit");

        Ok(Self(first_digit * 10 + last_digit))
    }
}

pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let calibration_values_sum: u32 = puzzle_input
        .lines()
        .map(|line| line.parse::<CalibrationValue>().unwrap())
        .map(u32::from)
        .sum();

    Ok(calibration_values_sum.to_string())
}
