use derive_new::new;
use grid::Grid;
use itertools::Itertools;
use parse_display::{Display, FromStr};
use std::{fmt, str};
use strum::EnumIs;

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
            .flat_map(|line| {
                line.chars().map(|c| {
                    // TODO: remove unwrap from parse
                    c.to_string().as_str().parse::<SpaceObject>().unwrap()
                })
            })
            .collect_vec();

        let mut grid = Grid::from_vec(grid_vec, cols);

        // TODO: remove code duplication between duplicating empty rows and columns.

        // Duplicate empty rows.
        let empty_row = vec![SpaceObject::EmptySpace; grid.cols()];

        let empty_row_indexes = grid
            .iter_rows()
            .enumerate()
            // Only keep empty rows.
            .filter(|(_, row)| row.clone().all(|o| matches!(o, SpaceObject::EmptySpace)))
            .map(|(row_idx, _)| row_idx)
            .collect_vec();

        // Insert from the back to not interfere with indexes.
        for empty_row_idx in empty_row_indexes.into_iter().rev() {
            grid.insert_row(empty_row_idx, empty_row.clone());
        }

        // Duplicate empty columns.
        let empty_col = vec![SpaceObject::EmptySpace; grid.rows()];

        let empty_col_indexes = grid
            .iter_cols()
            .enumerate()
            // Only keep empty columns.
            .filter(|(_, col)| col.clone().all(|o| matches!(o, SpaceObject::EmptySpace)))
            .map(|(col_idx, _)| col_idx)
            .collect_vec();

        // Insert from the back to not interfere with indexes.
        for empty_col_idx in empty_col_indexes.into_iter().rev() {
            grid.insert_col(empty_col_idx, empty_col.clone());
        }

        // Add an internally saved index to each grid element.
        let cols = grid.cols();
        let grid_vec_with_indexes = grid
            .into_vec()
            .into_iter()
            .enumerate()
            .map(|(idx, space_object)| {
                let row_idx = idx / cols;
                let col_idx = idx % cols;
                SpaceObjectInGrid::new(space_object, row_idx, col_idx)
            })
            .collect_vec();
        let grid = Grid::from_vec(grid_vec_with_indexes, cols);

        Ok(Self(grid))
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
        let expected_solution = 374;
        (puzzle_input, expected_solution.to_string())
    }
}
