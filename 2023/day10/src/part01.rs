use anyhow::bail;
use derive_new::new;
use ndarray::Array2;
use parse_display::FromStr;
use std::{cmp::Ordering, str};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug)]
struct TileGrid(Array2<TileInGrid>);

impl str::FromStr for TileGrid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows = s.lines().count();
        let cols = match s.lines().next() {
            Some(first_row) => first_row.chars().count(),
            None => bail!("Expected there to be at least one row, but found none."),
        };

        let grid_vec: Vec<_> = s
            .lines()
            .enumerate()
            .flat_map(|(row, line)| {
                line.chars().enumerate().map(move |(col, c)| {
                    // TODO: remove unwrap from parse
                    let tile = Tile::from(c.to_string().as_str().parse::<TileParsed>().unwrap());
                    TileInGrid::new(tile, row, col)
                })
            })
            .collect();

        let grid = Array2::from_shape_vec((rows, cols), grid_vec)?;
        Ok(Self(grid))
    }
}

impl TileGrid {
    /// Get the starting tile.
    fn get_starting_tile(&self) -> &TileInGrid {
        self.0
            .iter()
            .find(|tile| matches!(tile.tile, Tile::AnimalStartingPosition))
            .expect("there must be a starting tile")
    }

    /// Find the tiles that make up the main loop. The main loop is a set of
    /// connected pipes that form a loop from the starting tile back to the
    /// starting tile.
    fn find_main_loop(&self) -> Vec<&TileInGrid> {
        let starting_tile = self.get_starting_tile();

        let (initial_pipe, initial_dir) = self.find_initial_connected_pipes(starting_tile);

        // TODO: try using maybe fold for more idiomatic rust, also vec1 for nice .last()

        let mut main_loop_tiles = vec![starting_tile];

        let mut cur_tile = initial_pipe;
        let mut facing_dir = &initial_dir;

        while cur_tile != starting_tile {
            main_loop_tiles.push(cur_tile);
            (cur_tile, facing_dir) = cur_tile.find_next_connected_pipe(facing_dir, self);
        }

        main_loop_tiles
    }

    /// Find one of the pipes connected to the starting tile. Return the
    /// direction in which it was found, relative to the starting pipe, along
    /// with the connected pipe tile itself.
    fn find_initial_connected_pipes<'a>(
        &'a self,
        starting_tile: &TileInGrid,
    ) -> (&'a TileInGrid, Direction) {
        // Search every direction for connected pipes.
        let mut initial_pipes: Vec<(&TileInGrid, Direction)> = Direction::iter()
            .filter_map(|direction| {
                if let Some(potential_pipe) = starting_tile.get_next_in_direction(&direction, self)
                {
                    if potential_pipe.is_connected_with_starting_tile(starting_tile) {
                        return Some((potential_pipe, direction));
                    }
                }
                None
            })
            .collect();

        // It doesn't matter which of the two initial pipes we choose to traverse.
        let (Some((initial_pipe, initial_dir)), Some(_)) =
            (initial_pipes.pop(), initial_pipes.pop())
        else {
            panic!("expected there to be exactly two pipes connected to the starting tile");
        };

        (initial_pipe, initial_dir)
    }
}

/// An index into a `ndarray::Array2`.
type Array2Index = usize;

#[derive(Debug, new, PartialEq, Eq)]
struct TileInGrid {
    tile: Tile,
    row: Array2Index,
    col: Array2Index,
}

/// The positional relation between two tiles.
#[derive(Debug, PartialEq, Eq)]
enum PositionRelation {
    Above,
    Below,
    RightOf,
    LeftOf,
    Same,
}

impl TileInGrid {
    /// Find the positional relation between the two tiles `self` and `other`.
    fn get_relation_to(&self, other: &Self) -> PositionRelation {
        match self.row.cmp(&other.row) {
            Ordering::Greater => PositionRelation::Below,
            Ordering::Less => PositionRelation::Above,
            Ordering::Equal => match self.col.cmp(&other.col) {
                Ordering::Greater => PositionRelation::RightOf,
                Ordering::Less => PositionRelation::LeftOf,
                Ordering::Equal => PositionRelation::Same,
            },
        }
    }
}

#[test]
fn test_get_relation_to() {
    // a
    // b
    let tile_a = TileInGrid::new(Tile::from(TileParsed::Ground), 0, 0);
    let tile_b = TileInGrid::new(Tile::from(TileParsed::Ground), 1, 0);
    assert_eq!(PositionRelation::Same, tile_a.get_relation_to(&tile_a));
    assert_eq!(PositionRelation::Above, tile_a.get_relation_to(&tile_b));
    assert_eq!(PositionRelation::Below, tile_b.get_relation_to(&tile_a));

    // cd
    let tile_c = TileInGrid::new(Tile::from(TileParsed::Ground), 0, 0);
    let tile_d = TileInGrid::new(Tile::from(TileParsed::Ground), 0, 1);
    assert_eq!(PositionRelation::LeftOf, tile_c.get_relation_to(&tile_d));
    assert_eq!(PositionRelation::RightOf, tile_d.get_relation_to(&tile_c));
}

impl TileInGrid {
    /// Check if `self` is connected with `other`, which is only the case if
    /// there is a touching side with two open pipe-ends.
    fn _is_connected_with(&self, other: &Self) -> bool {
        let self_must_be_pointing_in = match self.get_relation_to(other) {
            PositionRelation::Above => Direction::South,
            PositionRelation::Below => Direction::North,
            PositionRelation::LeftOf => Direction::East,
            PositionRelation::RightOf => Direction::West,
            PositionRelation::Same => return true,
        };
        let other_must_be_pointing_in = self_must_be_pointing_in.invert();

        self.tile.points_in_direction(&self_must_be_pointing_in)
            && other.tile.points_in_direction(&other_must_be_pointing_in)
    }

    /// Check if `self` is connected with the `starting_tile`. The
    /// `starting_tile` can act as any possible pipe in order to fulfill the
    /// requirements for `_is_connected_with`.
    fn is_connected_with_starting_tile(&self, starting_tile: &Self) -> bool {
        let direction_self_must_be_pointing_in = match self.get_relation_to(starting_tile) {
            PositionRelation::Above => Direction::South,
            PositionRelation::Below => Direction::North,
            PositionRelation::LeftOf => Direction::East,
            PositionRelation::RightOf => Direction::West,
            PositionRelation::Same => return true,
        };
        self.tile
            .points_in_direction(&direction_self_must_be_pointing_in)
    }
}

#[test]
fn test_is_connected_with() {
    // F
    // |
    let tile_a = TileInGrid::new(Tile::from(TileParsed::BendSouthEast), 0, 0);
    let tile_b = TileInGrid::new(Tile::from(TileParsed::PipeVertical), 1, 0);
    assert!(tile_a._is_connected_with(&tile_b));
    assert!(tile_b._is_connected_with(&tile_a));

    // |
    // -
    let tile_a = TileInGrid::new(Tile::from(TileParsed::PipeVertical), 0, 0);
    let tile_b = TileInGrid::new(Tile::from(TileParsed::PipeHorizontal), 1, 0);
    assert!(!tile_a._is_connected_with(&tile_b));
    assert!(!tile_b._is_connected_with(&tile_a));

    // -J
    let tile_a = TileInGrid::new(Tile::from(TileParsed::PipeHorizontal), 0, 0);
    let tile_b = TileInGrid::new(Tile::from(TileParsed::BendNorthWest), 0, 1);
    assert!(tile_a._is_connected_with(&tile_b));
    assert!(tile_b._is_connected_with(&tile_a));
}

impl TileInGrid {
    /// Relative to this tile, get the next tile in the specified direction,
    /// if it exists.
    fn get_next_in_direction<'a>(
        &self,
        direction: &Direction,
        tile_grid: &'a TileGrid,
    ) -> Option<&'a Self> {
        let (new_row, new_col) = direction.translate(&self.row, &self.col);

        // Don't return this tile if no tile in direction was found.
        if new_row == self.row && new_col == self.col {
            return None;
        }

        tile_grid.0.get((new_row, new_col))
    }

    /// Given the pipe tile we previously stood on and the pipe tile we are
    /// currently standing on, determine the next pipe tile we can move to.
    fn find_next_connected_pipe<'a>(
        &self,
        facing_direction: &Direction,
        tile_grid: &'a TileGrid,
    ) -> (&'a Self, &Direction) {
        // TODO: convert panics to proper error handling

        let Tile::Pipe(Pipe(a, b)) = &self.tile else {
            panic!("method should never be called on a non-pipe tile");
        };

        // Ensure we don't go back to the direction we came from.
        let came_from_direction = facing_direction.invert();
        let new_direction = if a == &came_from_direction { b } else { a };

        let new_tile = self
            .get_next_in_direction(new_direction, tile_grid)
            .expect("main tile loop has ended unexpectedly, as no next tile was found");

        (new_tile, new_direction)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Pipe(Pipe),
    Ground,
    AnimalStartingPosition,
}

impl From<TileParsed> for Tile {
    fn from(tile: TileParsed) -> Self {
        match tile {
            TileParsed::Ground => Self::Ground,
            TileParsed::AnimalStartingPosition => Self::AnimalStartingPosition,
            TileParsed::PipeVertical => Self::Pipe(Pipe(Direction::North, Direction::South)),
            TileParsed::PipeHorizontal => Self::Pipe(Pipe(Direction::West, Direction::East)),
            TileParsed::BendNorthEast => Self::Pipe(Pipe(Direction::North, Direction::East)),
            TileParsed::BendNorthWest => Self::Pipe(Pipe(Direction::North, Direction::West)),
            TileParsed::BendSouthWest => Self::Pipe(Pipe(Direction::South, Direction::West)),
            TileParsed::BendSouthEast => Self::Pipe(Pipe(Direction::South, Direction::East)),
        }
    }
}

impl Tile {
    /// Check if a tile is a pipe pointing in a specified direction with one of
    /// its pipe-ends.
    fn points_in_direction(&self, direction: &Direction) -> bool {
        match self {
            Self::Pipe(Pipe(dir_a, dir_b)) => dir_a == direction || dir_b == direction,
            _ => false,
        }
    }
}

/// A pipe that connects two `Direction`s.
#[derive(Debug, PartialEq, Eq)]
struct Pipe(Direction, Direction);

#[derive(Debug, PartialEq, Eq, EnumIter)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    /// Invert the direction. Consider: if we were facing in this direction,
    /// and turned around, in which direction would we be facing?
    fn invert(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    /// Translate moving in this direction in a grid to changes to the row and
    /// column indexes.
    fn translate(&self, row: &Array2Index, col: &Array2Index) -> (Array2Index, Array2Index) {
        match self {
            Direction::North => (row.saturating_sub(1), *col),
            Direction::South => (row + 1, *col),
            Direction::East => (*row, col + 1),
            Direction::West => (*row, col.saturating_sub(1)),
        }
    }
}

#[test]
fn test_translate_direction() {
    assert_eq!((0, 1), Direction::North.translate(&1, &1));
    assert_eq!((2, 1), Direction::South.translate(&1, &1));
    assert_eq!((1, 2), Direction::East.translate(&1, &1));
    assert_eq!((1, 0), Direction::West.translate(&1, &1));
}

#[derive(Debug, FromStr)]
enum TileParsed {
    #[display("|")]
    PipeVertical,

    #[display("-")]
    PipeHorizontal,

    #[display("L")]
    BendNorthEast,

    #[display("J")]
    BendNorthWest,

    #[display("7")]
    BendSouthWest,

    #[display("F")]
    BendSouthEast,

    #[display(".")]
    Ground,

    #[display("S")]
    AnimalStartingPosition,
}

/// Perform a division, but round up if the quotient is not a whole number.
fn round_up_div(dividend: usize, divisor: usize) -> usize {
    (dividend + 1) / divisor
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let tile_grid: TileGrid = puzzle_input.parse()?;

    let main_loop_tiles = tile_grid.find_main_loop();

    let steps_till_point_farthest_from_starting_position = round_up_div(main_loop_tiles.len(), 2);

    Ok(steps_till_point_farthest_from_starting_position.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide the example details as `(puzzle input, expected solution)`.
    pub fn example_details() -> (&'static str, String) {
        let puzzle_input = indoc! {"
            ..F7.
            .FJ|.
            SJ.L7
            |F--J
            LJ...
        "};
        // Distances from S for each tile on the main loop:
        // ..45.
        // .236.
        // 01.78
        // 14567
        // 23...
        let expected_solution = 8;
        (puzzle_input, expected_solution.to_string())
    }
}
