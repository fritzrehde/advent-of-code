use derive_new::new;
use itertools::Itertools;
use std::{collections::HashMap, ops::RangeInclusive, str};
use strum::EnumIs;
use uuid::Uuid;

/// A 2D grid of `Char`s.
#[derive(Clone)]
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
struct CharGrid(Vec<Vec<Char>>);

impl str::FromStr for CharGrid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let char_grid = s
            .lines()
            .map(|line| line.chars().map(Char::from).collect())
            .collect();
        Ok(Self(char_grid))
    }
}

#[test]
fn test_parsing_char_grid() {
    let char_grid = CharGrid(vec![vec![
        Char::Digit('4'),
        Char::Digit('6'),
        Char::Digit('7'),
        Char::Dot,
        Char::Dot,
        Char::Digit('1'),
        Char::Digit('1'),
        Char::Digit('4'),
        Char::Dot,
        Char::Dot,
    ]]);
    assert_eq!(char_grid, "467..114..".parse().unwrap());
}

impl CharGrid {
    // TODO: too many unnecessary Vec allocations

    /// Find all `Number`s (series of digit chars) contained in the grid.
    fn find_numbers(&self) -> Vec<Number> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(row_idx, row)| {
                row.iter()
                    .enumerate()
                    .map(|(col_idx, c)| CharWithPosition::new(c.clone(), row_idx, col_idx))
                    .group_by(|c| matches!(c.char, Char::Digit(_)))
                    .into_iter()
                    .filter_map(|(is_digit, char_group)| is_digit.then_some(char_group))
                    .map(|digit_group| digit_group.collect())
                    .collect::<Vec<Number>>()
            })
            .collect()
    }

    /// In row `row_idx`, search every column in `col_range` for gears.
    /// If no gears are found, `None` is returned, else a non-empty list
    /// of gears is returned.
    fn find_gear_in_region(
        &self,
        row_idx: usize,
        col_range: RangeInclusive<usize>,
    ) -> Option<Vec<Gear>> {
        let Some(row) = self.0.get(row_idx) else {
            return None;
        };
        let gears: Vec<Gear> = col_range
            .into_iter()
            .filter_map(|col_idx| match row.get(col_idx) {
                Some(Char::Gear(gear)) => Some(gear.clone()),
                _ => None,
            })
            .collect();
        (!gears.is_empty()).then_some(gears)
    }
}

#[test]
fn test_char_grid_find_numbers() {
    let numbers = vec![Number::new(467, 3, 0, 0), Number::new(114, 3, 0, 5)];
    assert_eq!(
        numbers,
        "467..114.*".parse::<CharGrid>().unwrap().find_numbers()
    );
}

#[test]
fn test_char_grid_find_gear_in_region() {
    // col_idx:                0123456789
    let char_grid: CharGrid = "467..114.*".parse().unwrap();
    assert!(char_grid.find_gear_in_region(0, 0..=7).is_none());
    assert!(char_grid.find_gear_in_region(0, 0..=8).is_none());
    assert!(char_grid.find_gear_in_region(0, 0..=9).is_some());
    assert!(char_grid.find_gear_in_region(0, 9..=9).is_some());
}

#[derive(Clone, new, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct Number {
    /// The value of the number.
    value: usize,
    /// The number of digits in the number, i.e. the lenght of the number's
    /// char list representation.
    len: usize,
    /// The row index of the first char of this number's char list
    /// representation.
    start_row_idx: usize,
    /// The column index of the first char of this number's char list
    /// representation.
    start_column_idx: usize,
}

impl From<&Number> for usize {
    fn from(number: &Number) -> Self {
        number.value
    }
}

impl Number {
    /// Check whether this number is adjacent (meaning above, below, left or
    /// right of, including the diagonal corners) to any gears, and return them.
    fn find_adjacent_gears(&self, char_grid: &CharGrid) -> Vec<Gear> {
        let (start_row_idx, start_col_idx) = (self.start_row_idx, self.start_column_idx);

        let above_row_idx = start_row_idx.saturating_sub(1);
        let same_row_idx = start_row_idx;
        let below_row_idx = start_row_idx + 1;

        // Indexes of the chars to the left and right of the number.
        let left_char_idx = start_col_idx.saturating_sub(1);
        let right_char_idx = start_col_idx + self.len;

        let left_col_range = left_char_idx..=left_char_idx;
        let right_col_range = right_char_idx..=right_char_idx;
        let entire_col_range = left_char_idx..=right_char_idx;

        // Check above (include corners).
        let check_above = || char_grid.find_gear_in_region(above_row_idx, entire_col_range.clone());

        // Check below (include corners).
        let check_below = || char_grid.find_gear_in_region(below_row_idx, entire_col_range.clone());

        // Check left (exclude corners).
        let check_left = || char_grid.find_gear_in_region(same_row_idx, left_col_range.clone());

        // Check right (exclude corners).
        let check_right = || char_grid.find_gear_in_region(same_row_idx, right_col_range.clone());

        // Check all possible directions for adjacent gears.
        let mut gears = vec![];
        if let Some(above_gears) = check_above() {
            gears.extend(above_gears);
        }
        if let Some(below_gears) = check_below() {
            gears.extend(below_gears);
        }
        if let Some(left_gears) = check_left() {
            gears.extend(left_gears);
        }
        if let Some(right_gears) = check_right() {
            gears.extend(right_gears);
        }
        gears
    }

    /// Calculate the gear ratio for two numbers.
    fn gear_ratio(&self, other: &Self) -> usize {
        self.value * other.value
    }
}

// TODO: turn this into an actual rust crate. Potentially name `LazyBoolOr`.
trait LazyBool {
    /// If `self` is `true`, return `true`, otherwise evaluate `f` and return
    /// its returned value.
    fn or_else<F>(self, f: F) -> bool
    where
        F: FnOnce() -> bool;
}

impl LazyBool for bool {
    fn or_else<F>(self, f: F) -> bool
    where
        F: FnOnce() -> bool,
    {
        if self {
            true
        } else {
            f()
        }
    }
}

impl FromIterator<CharWithPosition> for Number {
    fn from_iter<T: IntoIterator<Item = CharWithPosition>>(chars_iter: T) -> Self {
        let mut iter = chars_iter.into_iter().peekable();

        let first = iter.peek().expect("char list should not be empty");
        let start_row_idx = first.row_idx;
        let start_column_idx = first.column_idx;

        let (num, len): (usize, usize) = iter
            .filter_map(|c| match c.char {
                Char::Digit(digit) => digit.to_digit(10),
                _ => None,
            })
            .map(|digit| usize::try_from(digit).unwrap())
            .fold((0, 0), |(num, len), digit| (10 * num + digit, len + 1));

        Self {
            value: num,
            len,
            start_row_idx,
            start_column_idx,
        }
    }
}

#[test]
fn test_parsing_number() {
    let number: Number = [
        CharWithPosition::new(Char::Digit('4'), 0, 0),
        CharWithPosition::new(Char::Digit('6'), 0, 0),
        CharWithPosition::new(Char::Digit('7'), 0, 0),
    ]
    .into_iter()
    .collect();

    assert_eq!(number, Number::new(467, 3, 0, 0),);
}

/// A `Char` that also stores its position in the `CharGrid`.
#[derive(Clone, new)]
struct CharWithPosition {
    /// The underlying character.
    char: Char,
    /// The row index of this char in the `CharGrid`.
    row_idx: usize,
    /// The column index of this char in the `CharGrid`.
    column_idx: usize,
}

/// A `Char` inside the `CharGrid`.
#[derive(Clone, EnumIs)]
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
enum Char {
    Digit(char),
    Dot,
    Gear(Gear),
    Other,
}

impl From<char> for Char {
    fn from(value: char) -> Self {
        if value.is_ascii_digit() {
            Char::Digit(value)
        } else if value == '.' {
            Char::Dot
        } else if value == '*' {
            Char::Gear(Default::default())
        } else {
            Char::Other
        }
    }
}

/// A uniquely identified Gear.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
struct Gear {
    unique_id: Uuid,
}

impl Default for Gear {
    fn default() -> Self {
        Self {
            unique_id: Uuid::new_v4(),
        }
    }
}

/// Idea: Build a gear-to-numbers map, where numbers that share a gear are
/// grouped together, by using `Itertools::group_by`.
fn solution_grouped_by(char_grid: CharGrid) -> usize {
    let gear_ratio_sum: usize = char_grid
        .find_numbers()
        // TODO: since we're calling iter() here anyway, find_numbers() should just return an iterator
        .iter()
        .flat_map(|number| {
            number
                .find_adjacent_gears(&char_grid)
                .into_iter()
                .map(|gear| (gear, number))
                .collect::<Vec<(Gear, &Number)>>()
        })
        // Sort by gear to ensure same gears appear consecutively, which is
        // required for `group_by` to work.
        .sorted_by(|(gear_a, _), (gear_b, _)| Ord::cmp(&gear_a, &gear_b))
        // Group into groups of common gears (compared via uuid's).
        .group_by(|(gear, _)| gear.clone())
        .into_iter()
        .filter_map(|(_, group)| {
            let mut num_iter = group.map(|(_, number)| number);
            // A gear must be adjacent to **exactly** two part numbers.
            match (num_iter.next(), num_iter.next(), num_iter.next()) {
                (Some(first), Some(second), None) => Some(first.gear_ratio(second)),
                _ => None,
            }
        })
        .sum();

    gear_ratio_sum
}

/// Idea: Build a gear-to-numbers map, where numbers that share a gear are
/// grouped together, by inserting numbers into a hashmap with the gear as
/// the key.
fn solution_hashmap(char_grid: CharGrid) -> usize {
    let mut gear_number_map: HashMap<Gear, Vec<&Number>> = HashMap::new();
    let numbers = char_grid.find_numbers();
    numbers
        // TODO: since we're calling iter() here anyway, find_numbers() should just return an iterator
        .iter()
        .flat_map(|number| {
            number
                .find_adjacent_gears(&char_grid)
                .into_iter()
                .map(move |gear| (gear, number))
        })
        .for_each(|(gear, number)| {
            gear_number_map.entry(gear).or_default().push(number);
        });

    let gear_ratio_sum: usize = gear_number_map
        .into_iter()
        .filter_map(|(_, numbers)| {
            // A gear must be adjacent to **exactly** two part numbers.
            match (numbers.get(0), numbers.get(1), numbers.get(2)) {
                (Some(first), Some(second), None) => Some(first.gear_ratio(second)),
                _ => None,
            }
        })
        .sum();

    gear_ratio_sum
}

pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let char_grid: CharGrid = puzzle_input.parse()?;

    let grouped_by = solution_grouped_by(char_grid.clone());
    let hashmap = solution_hashmap(char_grid);

    Ok(format!(
        "grouped_by solution: {}, hashmap solution: {}",
        grouped_by, hashmap
    ))
}
