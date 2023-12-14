use derive_new::new;
use grid::Grid;
use indicatif::ProgressIterator;
use itertools::{Itertools, Position};
use parse_display::{Display, FromStr};
use std::{fmt, str};
use strum::EnumIs;

/// A platform with a control panel with which it can be tilted in four
/// directions.
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// Perform a specified number of spin cycles.
    fn n_spin_cycles(&mut self, spin_cycles: usize) {
        let mut seen_grids = vec![self.clone()];

        for _ in (0..spin_cycles).progress() {
            self.spin_cycle();

            // Trick: We assume that repeatedly spin-cycling creates a cycle.
            // This means that once we identify the cycle, we can mimic
            // executing any number of spin cycles directly.

            // If we have seen this grid before, get its position.
            if let Some(cycle_start_idx) = seen_grids.iter().position(|seen_grid| self == seen_grid)
            {
                let cycle_len = seen_grids.len() - cycle_start_idx;
                let idx_in_cycle = (spin_cycles - cycle_start_idx) % cycle_len;
                let final_idx = cycle_start_idx + idx_in_cycle;

                // Fast-forward self to what its state would be after
                // spin-cycling `spin_cycles` times.
                *self = seen_grids.swap_remove(final_idx);
                break;
            }

            seen_grids.push(self.clone());
        }
    }

    /// Execute one spin cycle.
    fn spin_cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    /// Tilt the whole platform in `tilting_direction` until further tilting
    /// would not result in any more changes.
    fn tilt(&mut self, tilting_direction: Direction) {
        while self.tilt_once(&tilting_direction) {}
    }

    /// Tilt the whole platform in north so that each movable object gets
    /// to slide in the north until it hits an immovable object.
    /// Return whether the position of any object changed (i.e. whether any
    /// object slid somewhere).
    fn tilt_once(&mut self, tilting_direction: &Direction) -> bool {
        let mut any_object_moved = false;

        let (rows, cols) = self.0.size();
        for row_idx in 0..rows {
            for col_idx in 0..cols {
                if let Some(material) = self.0.get(row_idx, col_idx) {
                    // We only want to move round rocks.
                    if !material.material.is_round_rock() {
                        continue;
                    }

                    let (new_pos, old_pos) = material.move_until_boundary(self, &tilting_direction);

                    // Move this grid element to its new position.
                    if self.swap_elements(new_pos, old_pos) {
                        any_object_moved = true;
                    }
                }
            }
        }

        any_object_moved
    }

    fn total_load_north(&self) -> usize {
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

#[derive(Debug, new, Display, Clone, PartialEq, Eq)]
#[display("{material}")]
struct MaterialInGrid {
    material: Material,
    position: PositionInGrid,
}

impl MaterialInGrid {
    /// Move an element so far in `direction` until it hits a boundary. Don't
    /// actually move the object, just return the new and old positions.
    fn move_until_boundary(
        &self,
        platform: &PlatformGrid,
        direction: &Direction,
    ) -> (PositionInGrid, PositionInGrid) {
        let old_pos = self.position;
        let mut new_pos = old_pos;

        loop {
            let above_pos = direction.translate(&new_pos);

            // We can't go in `direction` any further.
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

#[derive(Debug, FromStr, Display, Clone, EnumIs, PartialEq, Eq)]
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
    East,
    South,
    West,
}

impl Direction {
    /// Translate moving in this direction in a grid to changes to the row and
    /// column indexes.
    fn translate(&self, pos: &PositionInGrid) -> PositionInGrid {
        let (row, col) = match self {
            Direction::North => (pos.row.saturating_sub(1), pos.col),
            Direction::South => (pos.row + 1, pos.col),
            Direction::East => (pos.row, pos.col + 1),
            Direction::West => (pos.row, pos.col.saturating_sub(1)),
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
        Direction::South.translate(&PositionInGrid::new(1, 1))
    );
    assert_eq!(
        PositionInGrid::new(1, 2),
        Direction::East.translate(&PositionInGrid::new(1, 1))
    );
    assert_eq!(
        PositionInGrid::new(1, 0),
        Direction::West.translate(&PositionInGrid::new(1, 1))
    );
}

const SPIN_CYCLES: usize = 1_000_000_000;

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let mut platform: PlatformGrid = puzzle_input.parse()?;

    platform.n_spin_cycles(SPIN_CYCLES);

    let total_load = platform.total_load_north();

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

        // After one tilt northwards:
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

        // After 1 cycle:
        // .....#....
        // ....#...O#
        // ...OO##...
        // .OO#......
        // .....OOO#.
        // .O#...O#.#
        // ....O#....
        // ......OOOO
        // #...O###..
        // #..OO#....

        // After 2 cycles:
        // .....#....
        // ....#...O#
        // .....##...
        // ..O#......
        // .....OOO#.
        // .O#...O#.#
        // ....O#...O
        // .......OOO
        // #..OO###..
        // #.OOO#...O

        // After 3 cycles:
        // .....#....
        // ....#...O#
        // .....##...
        // ..O#......
        // .....OOO#.
        // .O#...O#.#
        // ....O#...O
        // .......OOO
        // #...O###.O
        // #.OOO#...O

        let expected_solution = 64;
        (puzzle_input, expected_solution.to_string())
    }
}
