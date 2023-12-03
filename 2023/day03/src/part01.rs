use derive_new::new;
use itertools::Itertools;
use std::{ops::RangeInclusive, str};
use strum::EnumIs;

/// A 2D grid of `Char`s.
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
        Char::Symbol,
    ]]);
    assert_eq!(char_grid, "467..114.*".parse().unwrap());
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

    /// In row `row_idx`, search every column in `col_range` for a symbol, and
    /// return whether at least one symbol was found.
    fn find_symbol_in_region(&self, row_idx: usize, mut col_range: RangeInclusive<usize>) -> bool {
        col_range.any(|col_idx| {
            self.0
                .get(row_idx)
                .and_then(|row| row.get(col_idx))
                .map_or(false, Char::is_symbol)
        })
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
fn test_char_grid_find_symbol_in_region() {
    // col_idx:                0123456789
    let char_grid: CharGrid = "467..114.*".parse().unwrap();
    assert!(!char_grid.find_symbol_in_region(0, 0..=7));
    assert!(!char_grid.find_symbol_in_region(0, 0..=8));
    assert!(char_grid.find_symbol_in_region(0, 0..=9));
    assert!(char_grid.find_symbol_in_region(0, 9..=9));
}

#[derive(new)]
#[cfg_attr(test, derive(Eq, PartialEq, Debug))]
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
    /// right of, including the diagonal corners) to a symbol char.
    fn is_adjacent_to_symbol(&self, char_grid: &CharGrid) -> bool {
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
        let check_above =
            || char_grid.find_symbol_in_region(above_row_idx, entire_col_range.clone());

        // Check below (include corners).
        let check_below =
            || char_grid.find_symbol_in_region(below_row_idx, entire_col_range.clone());

        // Check left (exclude corners).
        let check_left = || char_grid.find_symbol_in_region(same_row_idx, left_col_range.clone());

        // Check right (exclude corners).
        let check_right = || char_grid.find_symbol_in_region(same_row_idx, right_col_range.clone());

        // Lazily check all possible directions for adjacent symbols.
        check_above().or_else(|| check_below().or_else(|| check_left().or_else(check_right)))
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
    Symbol,
}

impl From<char> for Char {
    fn from(value: char) -> Self {
        if value.is_ascii_digit() {
            Char::Digit(value)
        } else if value == '.' {
            Char::Dot
        } else {
            Char::Symbol
        }
    }
}

pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let char_grid: CharGrid = puzzle_input.parse()?;
    // TODO: since we're calling iter() here anyway, find_numbers() should just return an iterator
    let part_number_sum: usize = char_grid
        .find_numbers()
        .iter()
        .filter(|number| number.is_adjacent_to_symbol(&char_grid))
        .map(usize::from)
        .sum();

    Ok(part_number_sum.to_string())
}
