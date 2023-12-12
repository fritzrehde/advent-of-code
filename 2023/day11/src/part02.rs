use derive_new::new;
use grid::Grid;
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::{fmt, str};
use strum::EnumIs;

#[cfg(not(test))]
const SPACE_EXPANSION_FACTOR: usize = 1_000_000;

#[derive(Debug)]
struct SpaceGrid(Grid<SpaceObjectInGrid>);

impl str::FromStr for SpaceGrid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols = s
            .lines()
            .next()
            .map(|first_row| first_row.chars().count())
            .unwrap_or(0);

        let grid_vec = s
            .lines()
            .enumerate()
            .flat_map(|(row_idx, line)| {
                line.chars().enumerate().map(move |(col_idx, c)| {
                    // TODO: remove unwrap from parse
                    let space_object = c.to_string().as_str().parse::<SpaceObject>().unwrap();
                    SpaceObjectInGrid::new(space_object, row_idx, col_idx)
                })
            })
            .collect_vec();

        let grid = Grid::from_vec(grid_vec, cols);

        Ok(Self(grid).account_for_cosmic_expansion())
    }
}

impl fmt::Display for SpaceGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0.iter_rows() {
            for space_object in row {
                write!(f, "{}", space_object)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl SpaceGrid {
    /// Account for the cosmic expansion, where each row or column of completely
    /// empty space is expanded by the `SPACE_EXPANSION_FACTOR`. For performance
    /// reasons, we do not actually insert all of these new rows or columns, but
    /// only adjust the internal row and column indexes of the galaxies.
    fn account_for_cosmic_expansion(mut self) -> Self {
        // TODO: remove code duplication between finding empty rows and columns.

        let empty_row_indexes = self
            .0
            .iter_rows()
            .enumerate()
            // Only keep empty rows.
            .filter(|(_, row)| row.clone().all(|o| o.space_object.is_empty_space()))
            .map(|(row_idx, _)| row_idx)
            .collect_vec();

        let empty_col_indexes = self
            .0
            .iter_cols()
            .enumerate()
            // Only keep empty columns.
            .filter(|(_, col)| col.clone().all(|o| o.space_object.is_empty_space()))
            .map(|(col_idx, _)| col_idx)
            .collect_vec();

        // Adjust the row and column indexes of each galaxy to reflect the
        // space expansion.
        self.0
            .iter_mut()
            .filter(|o| o.space_object.is_galaxy())
            .for_each(|space_object| {
                // TODO: remove code duplication

                let row_factor = empty_row_indexes
                    .iter()
                    .filter(|&&idx| idx < space_object.row)
                    .count();
                let new_row = space_object.row + row_factor * (SPACE_EXPANSION_FACTOR - 1);

                let col_factor = empty_col_indexes
                    .iter()
                    .filter(|&&idx| idx < space_object.col)
                    .count();
                let new_col = space_object.col + col_factor * (SPACE_EXPANSION_FACTOR - 1);

                space_object.row = new_row;
                space_object.col = new_col;
            });

        self
    }

    /// Find all pairs of galaxies, and only count each pair once.
    fn find_all_galaxy_pairs(&self) -> impl Iterator<Item = GalaxyPair> {
        self.0
            .iter()
            .filter(|space_object| space_object.space_object.is_galaxy())
            .tuple_combinations()
            .map(|(a, b)| GalaxyPair(a, b))
    }
}

struct GalaxyPair<'a>(&'a SpaceObjectInGrid, &'a SpaceObjectInGrid);

impl<'a> GalaxyPair<'a> {
    /// Find the length of the shortest path between two galaxies.
    fn shortest_path_len(self) -> usize {
        let (a, b) = (self.0, self.1);

        let row_delta = a.row.abs_diff(b.row);
        let col_delta = a.col.abs_diff(b.col);
        row_delta + col_delta
    }
}

/// An index into a `ndarray::Array2`.
type Array2Index = usize;

/// A `SpaceObject`, but including row and column indexes in the `SpaceGrid`.
#[derive(Debug, Display, new, PartialEq, Eq)]
#[display("{space_object}")]
struct SpaceObjectInGrid {
    space_object: SpaceObject,
    row: Array2Index,
    col: Array2Index,
}

#[derive(Debug, FromStr, Display, Clone, PartialEq, Eq, EnumIs)]
enum SpaceObject {
    #[display("#")]
    Galaxy,

    #[display(".")]
    EmptySpace,
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    // TODO: remove unwrap
    let space_grid: SpaceGrid = puzzle_input.parse().unwrap();

    let shortest_path_lengths_sum: usize = space_grid
        .find_all_galaxy_pairs()
        .map(GalaxyPair::shortest_path_len)
        .sum();

    Ok(shortest_path_lengths_sum.to_string())
}

#[cfg(test)]
const SPACE_EXPANSION_FACTOR: usize = 10;

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide the example details as `(puzzle input, expected solution)`.
    pub fn example_details() -> (&'static str, String) {
        let puzzle_input = indoc! {"
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        "};
        let expected_solution = 1030;
        (puzzle_input, expected_solution.to_string())
    }
}
