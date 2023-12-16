use derive_new::new;
use grid::Grid;
use itertools::{chain, Itertools, Position};
use parse_display::{Display, FromStr};
use std::{collections::HashSet, fmt, str};
use strum::EnumIs;

#[derive(Debug, Clone)]
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

enum AxisIndex {
    Row(GridIndex),
    Column(GridIndex),
}

impl CaveFloorGrid {
    /// Simulate how an initial beam (placed anywhere in the grid) spreads.
    fn beam_enters(&mut self, starting_beam: Beam) {
        let mut beams = vec![starting_beam];

        while let Some(next_beam) = beams.pop() {
            let next_beams = next_beam.move_once(self);
            beams.extend(next_beams);
        }
    }

    /// Get all possible starting beams. Beams can start from any edge,
    /// moving/facing away from that edge.
    fn all_starting_edge_beams(&self) -> impl Iterator<Item = Beam> + '_ {
        let last_row = self.0.rows().saturating_sub(1);
        let last_col = self.0.cols().saturating_sub(1);

        let north_edge = self.edge_beam(AxisIndex::Row(0), Direction::North.invert());
        let south_edge = self.edge_beam(AxisIndex::Row(last_row), Direction::South.invert());
        let west_edge = self.edge_beam(AxisIndex::Column(0), Direction::West.invert());
        let east_edge = self.edge_beam(AxisIndex::Column(last_col), Direction::East.invert());

        chain!(north_edge, east_edge, south_edge, west_edge)
    }

    /// Get all possible beams starting from a specified edge according to an
    /// axis, and moving/facing away from that edge.
    fn edge_beam(
        &self,
        selected_axis: AxisIndex,
        facing_direction: Direction,
    ) -> impl Iterator<Item = Beam> + '_ {
        self.0
            .indexed_iter()
            .filter(move |((row, col), _)| match selected_axis {
                AxisIndex::Row(selected_row) => &selected_row == row,
                AxisIndex::Column(selected_col) => &selected_col == col,
            })
            .map(move |((row, col), _)| Beam::new(PositionInGrid { row, col }, facing_direction))
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

#[derive(Debug, new, Clone)]
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

#[derive(Debug, FromStr, Display, EnumIs, Clone)]
enum TileType {
    #[display(".")]
    Empty,

    #[display("{0}")]
    Mirror(Mirror),

    #[display("{0}")]
    Splitter(Splitter),
}

#[derive(Debug, FromStr, Display, Clone)]
enum Mirror {
    #[display("/")]
    SouthWestToNorthEast,

    #[display(r#"\"#)]
    NorthWestToSouthEast,
}

#[derive(Debug, FromStr, Display, Clone)]
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
            _ => vec![*going_in_direction],
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

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Display)]
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
    let cave_floor_grid: CaveFloorGrid = puzzle_input.parse()?;

    let (max_energized_tile_count, max_cave_floor_grid) = cave_floor_grid
        .all_starting_edge_beams()
        .map(|starting_beam| {
            let mut cave_floor_grid_cloned = cave_floor_grid.clone();
            cave_floor_grid_cloned.beam_enters(starting_beam);
            let energized_tile_count: usize = cave_floor_grid_cloned.energized_tiles().count();

            (energized_tile_count, cave_floor_grid_cloned)
        })
        .max_by(|(count_a, _), (count_b, _)| count_a.cmp(count_b))
        .expect("expected there to be at least one possible starting beam");

    println!("{}", max_cave_floor_grid);

    Ok(max_energized_tile_count.to_string())
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

        // How light beam bounces around grid in best starting position:
        //
        // start (downwards)
        //    |
        //    V
        // .|<2<\....
        // |v-v\^....
        // .v.v.|->>>
        // .v.v.v^.|.
        // .v.v.v^...
        // .v.v.v^..\
        // .v.v/2\\..
        // <-2-/vv|..
        // .|<<<2-|.\
        // .v//.|.v..

        let expected_solution = 51;
        (puzzle_input, expected_solution.to_string())
    }
}
