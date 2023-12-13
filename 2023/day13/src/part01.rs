use derive_new::new;
use grid::Grid;
use itertools::{izip, Itertools, Position};
use parse_display::{Display, FromStr};
use std::{fmt, str};

/// The grid we observe when taking our walk.
#[derive(Debug)]
struct WalkGrid(Grid<Material>);

impl str::FromStr for WalkGrid {
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
                line.chars().map(move |c| {
                    // TODO: remove unwrap from parse
                    c.to_string().as_str().parse::<Material>().unwrap()
                })
            })
            .collect_vec();

        Ok(Self(Grid::from_vec(grid_vec, cols)))
    }
}

impl fmt::Display for WalkGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (position, row) in self.0.iter_rows().with_position() {
            for space_object in row {
                write!(f, "{}", space_object)?;
            }
            if let Position::First | Position::Middle = position {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl From<Vec<Vec<Material>>> for WalkGrid {
    fn from(grid: Vec<Vec<Material>>) -> Self {
        let cols = grid.first().map_or(0, |first| first.len());
        Self(Grid::from_vec(grid.into_iter().flatten().collect(), cols))
    }
}

impl WalkGrid {
    fn find_mirror(self) -> Mirror {
        self.find_vertical_mirror()
            .or_else(|| self.find_horizontal_mirror())
            .expect("expected there to be a possible mirror")
    }

    /// Find a vertical mirror, if one exists.
    fn find_vertical_mirror(&self) -> Option<Mirror> {
        let get_ith_row = |i| self.0.iter_row(i);
        let get_rows_cols = || (self.0.rows(), self.0.cols());
        let mirror_index = find_mirror_index(get_rows_cols, get_ith_row);
        mirror_index.map(|index| Mirror::new(index, MirrorAxis::Vertical))
    }

    /// Find a horizontal mirror, if one exists.
    fn find_horizontal_mirror(&self) -> Option<Mirror> {
        let get_ith_col = |i| self.0.iter_col(i);
        let get_cols_rows = || (self.0.cols(), self.0.rows());
        let mirror_index = find_mirror_index(get_cols_rows, get_ith_col);
        mirror_index.map(|index| Mirror::new(index, MirrorAxis::Horizontal))
    }
}

/// Try finding a mirror's index (which is an index into a row, i.e. a column
/// index) that is the same for all rows. This function takes some generic
/// parameters so it can be used to find both vertical and horizontal mirrors.
/// The comments and variable names assume we are searching for a vertical
/// mirror. When this function is called with row and col swapped,
/// it searches for a horizontal mirror.
fn find_mirror_index<'a, F, G, I>(get_rows_cols: G, get_ith_row: F) -> Option<usize>
where
    G: Fn() -> (usize, usize),
    F: Fn(usize) -> I,
    I: Iterator<Item = &'a Material> + ExactSizeIterator + DoubleEndedIterator,
{
    let (rows, cols) = get_rows_cols();

    // Test all possible mirror indexes.
    (0..cols).find(|mirror_col_idx| {
        // All rows must have the same mirror.
        (0..rows).all(|row_idx| is_mirror(|| get_ith_row(row_idx), *mirror_col_idx))
    })
}

#[test]
fn test_is_mirror() {
    //     4
    //     |
    //    ><
    // ##.##.#
    // .#.##.#
    let vec = vec![
        vec![
            Material::Rock,
            Material::Rock,
            Material::Ash,
            Material::Rock,
            Material::Rock,
            Material::Ash,
            Material::Rock,
        ],
        vec![
            Material::Ash,
            Material::Rock,
            Material::Ash,
            Material::Rock,
            Material::Rock,
            Material::Ash,
            Material::Rock,
        ],
    ];
    let grid: WalkGrid = vec.into();

    assert_eq!(4, grid.find_mirror().index);
}

// TODO: if the `grid` crate implements Clone on its iterator type, there is no need to use this work-around with passing closures

/// Check whether this row (i.e. column iterator) contains a mirror at the
/// specified column index.
fn is_mirror<'a, F, I>(get_col_iter: F, mirror_col_idx: usize) -> bool
where
    F: Fn() -> I,
    I: Iterator<Item = &'a Material> + ExactSizeIterator + DoubleEndedIterator,
{
    // mirror_index = 5
    //      |
    // #.##..##.
    // aaaaABbbb
    //     ><
    // left_side_iter = [Aa...]
    // right_side_iter = [Bb...]

    let left_side_iter = get_col_iter().take(mirror_col_idx).rev();
    let right_side_iter = get_col_iter().skip(mirror_col_idx);
    let mut both_sides_iter = izip!(left_side_iter, right_side_iter).peekable();

    // There are no mirrors where one of the sides is empty.
    if both_sides_iter.peek().is_none() {
        return false;
    }

    both_sides_iter.all(|(left, right)| left == right)
}

#[derive(Debug, new)]
struct Mirror {
    index: usize,
    axis: MirrorAxis,
}

impl Mirror {
    fn value(self) -> usize {
        match self.axis {
            MirrorAxis::Vertical => self.index,
            MirrorAxis::Horizontal => self.index * 100,
        }
    }
}

/// The axis that a mirror reflects perfectly.
#[derive(Debug)]
enum MirrorAxis {
    /// The mirror is vertical, so all columns are reflected perfectly.
    Vertical,
    /// The mirror is horizontal, so all rows are reflected perfectly.
    Horizontal,
}
#[derive(Debug, FromStr, Display, Clone, PartialEq, Eq)]
enum Material {
    #[display("#")]
    Rock,

    #[display(".")]
    Ash,
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let sum: usize = puzzle_input
        .split("\n\n")
        // TODO: remove unwrap
        .map(|block| block.parse::<WalkGrid>().unwrap())
        .map(WalkGrid::find_mirror)
        .map(Mirror::value)
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide the example details as `(puzzle input, expected solution)`.
    pub fn example_details() -> (&'static str, String) {
        let puzzle_input = indoc! {"
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        "};
        let expected_solution = 405;
        (puzzle_input, expected_solution.to_string())
    }
}
