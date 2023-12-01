use derive_more::Into;
use parse_display::FromStr;
use std::str::FromStr;

/// A digit that can be parsed from both a `SpelledOutDigit` and a `char`.
#[derive(Into)]
struct Digit(u32);

impl FromStr for Digit {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Try parsing the string into a `SpelledOutDigit` first.
        let digit: u32 = match s.parse::<SpelledOutDigit>() {
            Ok(digit) => digit.into(),
            // Try parsing a char into a u32.
            Err(_) => match s.parse::<char>() {
                Ok(c) => match c.to_digit(10) {
                    Some(digit) => digit,
                    None => anyhow::bail!("Failed to convert char to digit: {}", c),
                },
                Err(_) => anyhow::bail!("Failed to convert string to char: {}", s),
            },
        };
        Ok(Self(digit))
    }
}

#[test]
fn test_parsing_digit() {
    assert_eq!(2u32, "two".parse::<Digit>().expect("should parse").into());
    assert_eq!(2u32, "2".parse::<Digit>().expect("should parse").into());

    assert!("c".parse::<Digit>().is_err());
}

/// Spelled out digits without `Zero`.
#[derive(FromStr)]
#[display(style = "lowercase")]
enum SpelledOutDigit {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl From<SpelledOutDigit> for u32 {
    fn from(spelled_out_digit: SpelledOutDigit) -> Self {
        // Use the discriminant of the enum to get numeric value.
        spelled_out_digit as u32
    }
}

#[test]
fn test_parsing_spelled_out_digit() {
    let digit = "two".parse::<SpelledOutDigit>().expect("should parse");
    assert_eq!(2, u32::from(digit));

    assert!("not a number".parse::<SpelledOutDigit>().is_err());
}

/// Find the first `Digit` in a string, if one exists.
fn find_first_digit(s: &str) -> Option<Digit> {
    // Use a sliding window.
    for window_start in 0..s.len() {
        for window_end in window_start..s.len() {
            if let Ok(digit) = s[window_start..=window_end].parse::<Digit>() {
                return Some(digit);
            }
        }
    }
    None
}

/// Find the last `Digit` in a string, if one exists.
fn find_last_digit(s: &str) -> Option<Digit> {
    // Use a sliding window.
    for window_end in (0..s.len()).rev() {
        for window_start in (0..=window_end).rev() {
            if let Ok(digit) = s[window_start..=window_end].parse::<Digit>() {
                return Some(digit);
            }
        }
    }
    None
}

#[test]
fn test_find_first_digit() {
    assert_eq!(2u32, find_first_digit("abc23").unwrap().into());
    assert_eq!(4u32, find_first_digit("abcfour3").unwrap().into());
}

#[test]
fn test_find_last_digit() {
    assert_eq!(9u32, find_last_digit("4abc9").unwrap().into());
    assert_eq!(1u32, find_last_digit("threeabc2oneabc").unwrap().into());
}

/// The calibration value can be found by combining the first digit and the
/// last digit (in that order) to form a single two-digit number.
#[derive(Into)]
struct CalibrationValue(u32);

impl FromStr for CalibrationValue {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_digit = find_first_digit(s).expect("there should always be a first digit");
        let last_digit = find_last_digit(s).expect("there should always be a last digit");

        Ok(Self(u32::from(first_digit) * 10 + u32::from(last_digit)))
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
