use itertools::Itertools;
use std::{collections::VecDeque, str};
use streaming_iterator::{windows_mut, StreamingIterator, StreamingIteratorMut};
use vec1::vec1;

type HistoryValue = isize;

#[derive(Debug, Clone)]
struct History(VecDeque<HistoryValue>);

// TODO: boilerplate, replace with parse_display's parsing on delimiter (here ' ') once that's ready
impl str::FromStr for History {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let history = s.split(' ').map(str::parse).collect::<Result<_, _>>()?;
        Ok(Self(history))
    }
}

impl History {
    /// Extrapolate/predict the next value in this history.
    fn extrapolate_next_value(self) -> HistoryValue {
        let mut histories = vec1![self];

        // Generate all histories of differences.

        loop {
            // let latest_history = histories.last().expect("history can't be empty");
            let latest_history = histories.last();
            if latest_history.is_all_zero() {
                break;
            }
            histories.push(latest_history.history_of_differences());
        }

        // Extrapolate from the bottom-up.

        // Generate base-case:
        // n-1:   A   B   C ...
        // n:   0   0   0 ...
        // Start by adding a new zero to the start of your list of zeroes.
        histories.last_mut().0.push_front(0);

        // TODO: switch to leading_iterator::windows_mut once it supports .rev()

        // Iterate over histories from the bottom-up.

        // NOTE: Unfortunately, traditional Rust iterators do not support
        // creating windows over mutable references, so we use a dedicated
        // crate for that.
        let mut histories_windows = windows_mut(&mut histories, 2).rev();
        while let Some([top, bottom]) = histories_windows.next_mut() {
            // Generate new value `A` from existing values `C` and `B`.
            // i-1  (top):  A   B ...
            // i (bottom):    C ...
            // where A = B - C

            // TODO: remove unwraps

            let c = *bottom.0.front().unwrap();
            let b = *top.0.front().unwrap();
            let a = b - c;

            // Insert B into the previous history.
            top.0.push_front(a);
        }

        // An alternative, less idiomatic implementation that uses indexes:

        // let n = histories.len();
        // for i in (1..n).rev() {
        //     let c = histories[i].0.front().unwrap();
        //     let b = histories[i - 1].0.front().unwrap();
        //     let a = b - c;

        //     histories[i - 1].0.push_front(a);
        // }

        let extrapolated_next_value = histories.first().0.front().unwrap();
        *extrapolated_next_value
    }

    /// Create a new `History` composed of all the differences between the
    /// values in the current history. For example, `0 3 6 9` becomes `3 3 3`.
    fn history_of_differences(&self) -> Self {
        let differences = self.0.iter().tuple_windows().map(|(a, b)| b - a).collect();
        Self(differences)
    }

    /// Return whether all elements of this `History` are equal to 0.
    fn is_all_zero(&self) -> bool {
        self.0.iter().all(|a| *a == 0)
    }
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let extrapolated_values_sum: HistoryValue = puzzle_input
        .lines()
        .map(|line| line.parse::<History>().unwrap())
        .map(History::extrapolate_next_value)
        .sum();

    Ok(extrapolated_values_sum.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide the example details as `(puzzle input, expected solution)`.
    pub fn example_details() -> (&'static str, String) {
        let puzzle_input = indoc! {"
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        "};
        let expected_solution = 2;
        (puzzle_input, expected_solution.to_string())
    }
}
