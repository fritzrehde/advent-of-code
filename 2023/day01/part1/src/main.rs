use derive_more::Into;
use std::fs::File;
use std::io::{prelude::*, BufReader};
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

fn main() -> anyhow::Result<()> {
    let file = File::open("../puzzle_input.txt")?;
    let reader = BufReader::new(file);

    let calibration_values_sum: u32 = reader
        .lines()
        // TODO: remove unwrap, handle errors more nicely
        .map(|line| line.unwrap())
        .map(|line| line.parse::<CalibrationValue>().unwrap())
        .map(u32::from)
        .sum();

    println!("sum of calibration values: {}", calibration_values_sum);
    Ok(())
}
