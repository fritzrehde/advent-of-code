use derive_new::new;
use grid::Grid;
use itertools::{Itertools, Position};
use parse_display::{Display, FromStr};
use std::{fmt, str};
use strum::EnumIs;

/// A platform with a control panel with which it can be tiltet in four
/// directions.
#[derive(Debug)]
struct PlatformGrid(Grid<MaterialInGrid>);

impl str::FromStr for PlatformGrid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols = s
            .lines()
            .next()
            .map_or(0, |first_row| first_row.chars().count());

        let grid_vec = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(move |(col, c)| {
                    // TODO: remove unwrap from parse
                    let material = c.to_string().as_str().parse::<Material>().unwrap();
                    MaterialInGrid::new(material, PositionInGrid { row, col })
                })
            })
            .collect_vec();

        // TODO: we'll be iterating over columns mostly for part 1, so make sure that is more efficient than iterating over rows.
        Ok(Self(Grid::from_vec(grid_vec, cols)))
    }
}

impl fmt::Display for PlatformGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (position, row) in self.0.iter_rows().with_position() {
            for material in row {
                write!(f, "{}", material)?;
            }
            if let Position::First | Position::Middle = position {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl From<Vec<Vec<MaterialInGrid>>> for PlatformGrid {
    fn from(grid: Vec<Vec<MaterialInGrid>>) -> Self {
        let cols = grid.first().map_or(0, |first| first.len());
        Self(Grid::from_vec(grid.into_iter().flatten().collect(), cols))
    }
}

impl PlatformGrid {
    /// Tilt the whole platform north until further tilting would not result
    /// in any more changes.
    fn tilt_north(&mut self) {
        while self.tilt_north_once() {}
    }

    /// Tilt the whole platform in north so that each movable object gets
    /// to slide in the north until it hits an immovable object.
    /// Return whether the position of any object changed (i.e. whether any
    /// object slid somewhere).
    fn tilt_north_once(&mut self) -> bool {
        let mut any_object_moved = false;

        let (rows, cols) = self.0.size();
        // TODO: optimization: we could skip checking the first row, as we will never be able to move any objects any further.
        for col_idx in 0..cols {
            for row_idx in 0..rows {
                if let Some(material) = self.0.get(row_idx, col_idx) {
                    // We only want to move round rocks.
                    if !material.material.is_round_rock() {
                        continue;
                    }

                    let (new_pos, old_pos) = material.move_until_boundary(self);

                    // Move this grid element to its new position.
                    if self.swap_elements(new_pos, old_pos) {
                        any_object_moved = true;
                    }
                }
            }
        }

        any_object_moved
    }

    fn total_load(&self) -> usize {
        (0..self.0.rows())
            .rev()
            .enumerate()
            // Load for bottom row starts at 1.
            .map(|(i, row_idx)| (i + 1, row_idx))
            .flat_map(|(load, row_idx)| {
                self.0
                    .iter_row(row_idx)
                    .filter(|material| material.material.is_round_rock())
                    .map(move |_material| load)
            })
            .sum()
    }

    /// Swap two elements, and return whether the swap was successful.
    fn swap_elements(&mut self, pos_a: PositionInGrid, pos_b: PositionInGrid) -> bool {
        // An element can't be swapped with itself.
        if pos_a == pos_b {
            return false;
        }

        // TODO: we have to clone here because we can't get owned data from grid. The crate itself would have to provide this method to avoid cloning.
        let (a, b) = match (
            self.0.get(pos_a.row, pos_a.col),
            self.0.get(pos_b.row, pos_b.col),
        ) {
            (Some(a), Some(b)) => (a.material.clone(), b.material.clone()),
            _ => return false,
        };

        self.0.get_mut(pos_a.row, pos_a.col).map(|e| e.material = b);
        self.0.get_mut(pos_b.row, pos_b.col).map(|e| e.material = a);

        true
    }
}

type GridIndex = usize;

#[derive(Debug, new, Clone, Copy, PartialEq, Eq)]
struct PositionInGrid {
    row: GridIndex,
    col: GridIndex,
}

#[derive(Debug, new, Display, Clone)]
#[display("{material}")]
struct MaterialInGrid {
    material: Material,
    position: PositionInGrid,
}

impl MaterialInGrid {
    /// Move an element so far north until it hits a boundary. Don't actually
    /// move the object, just return the new and old row and col indexes.
    fn move_until_boundary(&self, platform: &PlatformGrid) -> (PositionInGrid, PositionInGrid) {
        let old_pos = self.position;
        let mut new_pos = old_pos;

        loop {
            let above_pos = Direction::North.translate(&new_pos);

            // We can't go north any further.
            if above_pos == new_pos {
                break;
            }

            if let Some(above_material) = platform.0.get(above_pos.row, above_pos.col) {
                if above_material.material.is_movable() {
                    new_pos = above_pos;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        (new_pos, old_pos)
    }
}

#[derive(Debug, FromStr, Display, Clone, EnumIs)]
enum Material {
    #[display("O")]
    RoundRock,

    #[display("#")]
    CubeShapedRock,

    #[display(".")]
    Empty,
}

impl Material {
    fn is_movable(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    _East,
    _South,
    _West,
}

impl Direction {
    /// Translate moving in this direction in a grid to changes to the row and
    /// column indexes.
    fn translate(&self, pos: &PositionInGrid) -> PositionInGrid {
        let (row, col) = match self {
            Direction::North => (pos.row.saturating_sub(1), pos.col),
            Direction::_South => (pos.row + 1, pos.col),
            Direction::_East => (pos.row, pos.col + 1),
            Direction::_West => (pos.row, pos.col.saturating_sub(1)),
        };
        PositionInGrid { row, col }
    }
}

#[test]
fn test_translate_direction() {
    assert_eq!(
        PositionInGrid::new(0, 1),
        Direction::North.translate(&PositionInGrid::new(1, 1))
    );
    assert_eq!(
        PositionInGrid::new(2, 1),
        Direction::_South.translate(&PositionInGrid::new(1, 1))
    );
    assert_eq!(
        PositionInGrid::new(1, 2),
        Direction::_East.translate(&PositionInGrid::new(1, 1))
    );
    assert_eq!(
        PositionInGrid::new(1, 0),
        Direction::_West.translate(&PositionInGrid::new(1, 1))
    );
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let mut platform: PlatformGrid = puzzle_input.parse()?;
    platform.tilt_north();

    println!("After tilting:\n{}", &platform);

    let total_load = platform.total_load();

    Ok(total_load.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide the example details as `(puzzle input, expected solution)`.
    pub fn example_details() -> (&'static str, String) {
        let puzzle_input = indoc! {"
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        "};

        // After tilt northwards:
        // OOOO.#.O..
        // OO..#....#
        // OO..O##..O
        // O..#.OO...
        // ........#.
        // ..#....#.#
        // ..O..#.O.O
        // ..O.......
        // #....###..
        // #....#....

        let expected_solution = 136;
        (puzzle_input, expected_solution.to_string())
    }
}
