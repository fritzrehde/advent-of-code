use derive_new::new;
use grid::Grid;
use itertools::{Itertools, Position};
use parse_display::{Display, FromStr};
use std::{collections::HashSet, fmt, str};
use strum::EnumIs;

#[derive(Debug)]
struct CaveFloorGrid(Grid<Tile>);

impl str::FromStr for CaveFloorGrid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols = s
            .lines()
            .next()
            .map_or(0, |first_row| first_row.chars().count());

        let grid_vec = s
            .lines()
            .flat_map(|line| {
                line.chars().map(|c| {
                    // TODO: remove unwrap from parse
                    let tile_type = c.to_string().as_str().parse::<TileType>().unwrap();
                    Tile::new(tile_type)
                })
            })
            .collect_vec();

        Ok(Self(Grid::from_vec(grid_vec, cols)))
    }
}

// TODO: grid should maybe provide a macro or similar to generate this boilerplate imo
impl fmt::Display for CaveFloorGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (position, row) in self.0.iter_rows().with_position() {
            for tile in row {
                write!(f, "{}", tile)?;
            }
            if let Position::First | Position::Middle = position {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl CaveFloorGrid {
    /// Simulate how an initial beam (placed anywhere in the grid) spreads.
    fn beam_enters(&mut self, initial_beam: Beam) {
        let mut beams = vec![initial_beam];

        while let Some(next_beam) = beams.pop() {
            let next_beams = next_beam.move_once(self);
            beams.extend(next_beams);
        }
    }

    fn energized_tiles(&self) -> impl Iterator<Item = &Tile> {
        self.0.iter().filter(|tile| tile.is_energized())
    }
}

#[derive(Debug, new, Clone, Hash, PartialEq, Eq, Display)]
#[display("{facing_direction}")]
struct Beam {
    /// The position of the beam inside a 2D grid.
    position: PositionInGrid,

    /// The direction the beam is currently pointing in.
    facing_direction: Direction,
}

impl Beam {
    fn move_once(&self, grid: &mut CaveFloorGrid) -> Vec<Self> {
        // Get the tile the beam is currently on.
        if let Some(tile_under_beam) = grid.0.get_mut(self.position.row, self.position.col) {
            // Only direct the beam if it hasn't passed through this tile before.
            if !tile_under_beam.seen_beams.contains(self) {
                return tile_under_beam.direct_incoming_beam(self);
            }
        }

        // TODO: probably inefficient, better to return Option<Vec1> (never empty anyways) or Option<Vec>
        vec![]
    }
}

type GridIndex = usize;

#[derive(Debug, new, Clone, Copy, PartialEq, Eq, Hash)]
struct PositionInGrid {
    row: GridIndex,
    col: GridIndex,
}

#[derive(Debug, new)]
struct Tile {
    /// The type of the tile.
    tile_type: TileType,

    /// All beams that have passed been "seen" by this tile, which
    /// includes all beams that have passed through, been reflected in, or
    /// been split in this tile.
    #[new(default)]
    seen_beams: HashSet<Beam>,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tile = match self.tile_type {
            TileType::Empty => {
                let seen_beams_len = self.seen_beams.len();

                if seen_beams_len > 1 {
                    seen_beams_len.to_string()
                } else if let Some(first) = self.seen_beams.iter().next() {
                    first.to_string()
                } else {
                    self.tile_type.to_string()
                }
            }
            _ => self.tile_type.to_string(),
        };

        write!(f, "{}", tile)?;

        Ok(())
    }
}

impl Tile {
    /// Where is an incoming beam directed, or how is it transformed, when
    /// it goes through this tile. This tile also memorizes that the incoming
    /// beam has passed through it.
    fn direct_incoming_beam(&mut self, beam: &Beam) -> Vec<Beam> {
        self.seen_beams.insert(beam.clone());
        self.tile_type.direct_incoming_beam(beam)
    }

    fn is_energized(&self) -> bool {
        !self.seen_beams.is_empty()
    }
}

#[derive(Debug, FromStr, Display, EnumIs)]
enum TileType {
    #[display(".")]
    Empty,

    #[display("{0}")]
    Mirror(Mirror),

    #[display("{0}")]
    Splitter(Splitter),
}

#[derive(Debug, FromStr, Display)]
enum Mirror {
    #[display("/")]
    SouthWestToNorthEast,

    #[display(r#"\"#)]
    NorthWestToSouthEast,
}

#[derive(Debug, FromStr, Display)]
enum Splitter {
    #[display("|")]
    Vertical,

    #[display("-")]
    Horizontal,
}

impl TileType {
    /// Where is an incoming beam directed, or how is it transformed, when
    /// it goes through this tile.
    fn direct_incoming_beam(&self, beam: &Beam) -> Vec<Beam> {
        let prev_position = &beam.position;
        let going_in_direction = &beam.facing_direction;
        let coming_from_direction = beam.facing_direction.invert();

        // TODO: no need to collect into a vector when we are iterating over it again anyways, maybe we can return an iterator directly.
        let new_directions = match self {
            // TODO: remove code duplication between the Mirror variants
            TileType::Mirror(Mirror::SouthWestToNorthEast) => match coming_from_direction {
                Direction::West | Direction::East => vec![coming_from_direction.next_clockwise()],
                Direction::North | Direction::South => {
                    vec![coming_from_direction.next_counterclockwise()]
                }
            },
            TileType::Mirror(Mirror::NorthWestToSouthEast) => match coming_from_direction {
                Direction::West | Direction::East => {
                    vec![coming_from_direction.next_counterclockwise()]
                }
                Direction::North | Direction::South => vec![coming_from_direction.next_clockwise()],
            },
            TileType::Splitter(Splitter::Vertical) if coming_from_direction.is_horizontal() => {
                vec![Direction::North, Direction::South]
            }
            TileType::Splitter(Splitter::Horizontal) if coming_from_direction.is_vertical() => {
                vec![Direction::West, Direction::East]
            }
            // The beam passes through this tile.
            _ => vec![going_in_direction.clone()],
        };

        new_directions
            .into_iter()
            .filter_map(|new_direction| {
                Some(Beam::new(
                    new_direction.translate(prev_position)?,
                    new_direction,
                ))
            })
            .collect_vec()
    }
}

// TODO: maybe this could/should even derive Copy?
#[derive(Debug, PartialEq, Eq, Clone, Hash, Display)]
enum Direction {
    #[display("^")]
    North,

    #[display(">")]
    East,

    #[display("v")]
    South,

    #[display("<")]
    West,
}

impl Direction {
    /// Invert the direction. Consider: if we were facing in this direction,
    /// and turned around, in which direction would we be facing?
    fn invert(&self) -> Self {
        // TODO: implement smarter, this is too "manual".
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }

    /// Translate moving in this direction in a grid to changes to the row and
    /// column indexes. This will never return the same position back. If going
    /// into some direction would be outside the grid bounds, return `None`.
    fn translate(&self, pos: &PositionInGrid) -> Option<PositionInGrid> {
        let (row, col) = match self {
            Self::North => (pos.row.checked_sub(1)?, pos.col),
            Self::South => (pos.row + 1, pos.col),
            Self::East => (pos.row, pos.col + 1),
            Self::West => (pos.row, pos.col.checked_sub(1)?),
        };
        Some(PositionInGrid { row, col })
    }

    /// Get the next direction, clockwise.
    fn next_clockwise(&self) -> Direction {
        // TODO: automate with strum/enum crate: get position of self in Direction::iter(), next clockwise is the next element.
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    /// Get the next direction, counter-clockwise.
    fn next_counterclockwise(&self) -> Direction {
        // TODO: automate with strum/enum crate: get position of self in Direction::iter(), next counter-clockwise is the previous element.
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }

    fn is_vertical(&self) -> bool {
        matches!(self, Self::North | Self::South)
    }

    fn is_horizontal(&self) -> bool {
        matches!(self, Self::West | Self::East)
    }
}

#[test]
fn test_translate_direction() {
    assert_eq!(
        Some(PositionInGrid::new(0, 1)),
        Direction::North.translate(&PositionInGrid::new(1, 1))
    );
    assert_eq!(
        Some(PositionInGrid::new(2, 1)),
        Direction::South.translate(&PositionInGrid::new(1, 1))
    );
    assert_eq!(
        Some(PositionInGrid::new(1, 2)),
        Direction::East.translate(&PositionInGrid::new(1, 1))
    );
    assert_eq!(
        Some(PositionInGrid::new(1, 0)),
        Direction::West.translate(&PositionInGrid::new(1, 1))
    );
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let mut cave_floor_grid: CaveFloorGrid = puzzle_input.parse()?;

    let initial_beam = Beam::new(PositionInGrid::new(0, 0), Direction::East);

    cave_floor_grid.beam_enters(initial_beam);

    println!("{}", cave_floor_grid);

    let energized_tiles_count: usize = cave_floor_grid.energized_tiles().count();

    Ok(energized_tiles_count.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide the example details as `(puzzle input, expected solution)`.
    pub fn example_details() -> (&'static str, String) {
        let puzzle_input = indoc! {r#"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "#};

        // How light beam bounces around grid:
        // >|<<<\....
        // |v-.\^....
        // .v...|->>>
        // .v...v^.|.
        // .v...v^...
        // .v...v^..\
        // .v../2\\..
        // <->-/vv|..
        // .|<<<2-|.\
        // .v//.|.v..

        let expected_solution = 46;
        (puzzle_input, expected_solution.to_string())
    }
}
