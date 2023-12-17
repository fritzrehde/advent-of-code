use derive_new::new;
use grid::Grid;
use itertools::{Itertools, Position};
use parse_display::{Display, FromStr};
use pathfinding::prelude::dijkstra;
use std::{collections::HashMap, fmt, str};
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug)]
struct CityBlockGrid(Grid<CityBlockInGrid>);

impl str::FromStr for CityBlockGrid {
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
                    let material = c.to_string().as_str().parse::<CityBlock>().unwrap();
                    CityBlockInGrid::new(material, PositionInGrid { row, col })
                })
            })
            .collect_vec();

        Ok(Self(Grid::from_vec(grid_vec, cols)))
    }
}

impl fmt::Display for CityBlockGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // To avoid code duplication, use the pretty printer with an empty path.
        write!(f, "{}", CityBlockGridWithPath::new(self, &vec![]))
    }
}

/// An extension to the `CityBlockGrid` that can be printed with the crucible
/// path filled in for nicer visualization.
#[derive(new)]
struct CityBlockGridWithPath<'a> {
    city_block_grid: &'a CityBlockGrid,
    crucible_path: &'a Vec<UltraCrucible>,
}

impl<'a> fmt::Display for CityBlockGridWithPath<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Provide easier/faster (random) access to the crucibles.
        let crucible_map: HashMap<PositionInGrid, &UltraCrucible> = self
            .crucible_path
            .iter()
            .map(|crucible| (crucible.position, crucible))
            .collect();

        for (position, row) in self.city_block_grid.0.iter_rows().with_position() {
            for city_block in row {
                match crucible_map.get(&city_block.position) {
                    Some(crucible) => write!(f, "{}", crucible.facing_direction)?,
                    None => write!(f, "{}", city_block)?,
                };
            }
            if let Position::First | Position::Middle = position {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl CityBlockGrid {
    /// Find the shortest path between a `start` and `end` city block. Return
    /// the path itself, as well as the total heat-loss value.
    fn find_shortest_path(
        &self,
        start: &CityBlockInGrid,
        end: &CityBlockInGrid,
    ) -> Option<(Vec<UltraCrucible>, HeatLoss)> {
        // The crucible could be facing any direction when starting.
        let all_possible_starting_crucibles = Direction::iter()
            .map(|dir| UltraCrucible::new(start.position, dir, 0, start.city_block.heat_loss));

        all_possible_starting_crucibles
            .into_iter()
            .filter_map(|start_crucible| {
                self.find_shortest_path_from_crucible(&start_crucible, end.position)
            })
            .min_by(|(_, cost_a), (_, cost_b)| cost_a.cmp(cost_b))
    }

    /// Find the shortest path starting with a given crucible.
    fn find_shortest_path_from_crucible(
        &self,
        start: &UltraCrucible,
        end_position: PositionInGrid,
    ) -> Option<(Vec<UltraCrucible>, HeatLoss)> {
        // Find next path elements to check.
        let successors = |c: &UltraCrucible| c.next_possible_moves(self);
        // Termination condition.
        let success = |c: &UltraCrucible| {
            // One needs to move a minimum of four blocks in a direction
            // before one can stop at the end.
            c.consecutive_same_direction_moves >= 4 && c.position == end_position
        };

        dijkstra(start, successors, success).map(|(mut path, total_cost)| {
            // `dijkstra`s return value includes the starting node, which we
            // should exclude.
            let _start = path.remove(0);
            (path, total_cost)
        })
    }
}

type GridIndex = usize;

#[derive(Debug, new, Clone, Copy, PartialEq, Eq, Hash)]
struct PositionInGrid {
    row: GridIndex,
    col: GridIndex,
}

/// The object that moves through the grid, aiming to find the path with
/// minimum heat loss.
#[derive(Debug, new, PartialEq, Eq, Hash, Clone)]
struct UltraCrucible {
    position: PositionInGrid,
    facing_direction: Direction,
    /// The number of consecutive moves the crucible has made in the same
    /// direction. Once the crucible changes direction, this is reset to the
    /// minimum value 1.
    consecutive_same_direction_moves: usize,
    cost: HeatLoss,
}

impl UltraCrucible {
    /// Given this crucible object, what are all possible moves it can make?
    fn next_possible_moves(&self, grid: &CityBlockGrid) -> Vec<(UltraCrucible, HeatLoss)> {
        // One cannot reverse direction, one may only:
        [
            // Turn left
            // One needs to move a minimum of four blocks in a direction
            // before one can turn.
            (self.consecutive_same_direction_moves >= 4)
                .then_some((self.facing_direction.left_of(), 1)),
            // Turn right
            // One needs to move a minimum of four blocks in a direction
            // before one can turn.
            (self.consecutive_same_direction_moves >= 4)
                .then_some((self.facing_direction.right_of(), 1)),
            // Continue straight.
            // One can move a maximum of ten consecutive blocks without turning.
            (self.consecutive_same_direction_moves < 10).then_some((
                self.facing_direction,
                self.consecutive_same_direction_moves + 1,
            )),
        ]
        .into_iter()
        .flatten()
        .filter_map(|(next_facing_direction, new_consecutive_straight_moves)| {
            // Only add the next position if it is in the bounds of the grid.
            let next_position = next_facing_direction.translate(&self.position)?;
            let city_block = grid.0.get(next_position.row, next_position.col)?;

            let heat_loss = city_block.city_block.heat_loss;

            Some((
                Self::new(
                    next_position,
                    next_facing_direction,
                    new_consecutive_straight_moves,
                    city_block.city_block.heat_loss,
                ),
                heat_loss,
            ))
        })
        .collect_vec()
    }
}

#[derive(Debug, new, Display, Clone)]
#[display("{city_block}")]
struct CityBlockInGrid {
    city_block: CityBlock,
    position: PositionInGrid,
}

#[derive(Debug, FromStr, Display, Clone)]
struct CityBlock {
    heat_loss: HeatLoss,
}

type HeatLoss = usize;

#[derive(Debug, Display, PartialEq, Eq, Copy, Clone, Hash, EnumIter)]
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

    fn left_of(&self) -> Self {
        self.next_counterclockwise()
    }

    fn right_of(&self) -> Self {
        self.next_clockwise()
    }

    /// Get the next direction, clockwise.
    fn next_clockwise(&self) -> Self {
        // TODO: automate with strum/enum crate: get position of self in Direction::iter(), next clockwise is the next element.
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }

    /// Get the next direction, counter-clockwise.
    fn next_counterclockwise(&self) -> Self {
        // TODO: automate with strum/enum crate: get position of self in Direction::iter(), next counter-clockwise is the previous element.
        match self {
            Self::North => Self::West,
            Self::West => Self::South,
            Self::South => Self::East,
            Self::East => Self::North,
        }
    }
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let city_block_grid: CityBlockGrid = puzzle_input.parse()?;

    let mut grid_iter = city_block_grid.0.iter();
    let start = grid_iter.next().expect("grid contains no top left element");
    let end = grid_iter
        .last()
        .expect("grid contains no bottom right element");

    let (shortest_path, min_heat_loss) = city_block_grid
        .find_shortest_path(start, end)
        .expect("no path found");

    println!(
        "{}",
        CityBlockGridWithPath::new(&city_block_grid, &shortest_path)
    );

    Ok(min_heat_loss.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide the example details as `(puzzle input, expected solution)`.
    pub fn example_details() -> impl Iterator<Item = (&'static str, String)> {
        let puzzle_input_1 = indoc! {"
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        "};

        // Shortest path:
        // 2>>>>>>>>1323
        // 32154535v5623
        // 32552456v4254
        // 34465858v5452
        // 45466578v>>>>
        // 143859879845v
        // 445787698776v
        // 363787797965v
        // 465496798688v
        // 456467998645v
        // 122468686556v
        // 254654888773v
        // 432267465553v

        let expected_solution_1 = 94;

        let puzzle_input_2 = indoc! {"
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991
        "};

        // Shortest path:
        // 1>>>>>>>1111
        // 9999999v9991
        // 9999999v9991
        // 9999999v9991
        // 9999999v>>>>

        let expected_solution_2 = 71;

        [
            (puzzle_input_1, expected_solution_1.to_string()),
            (puzzle_input_2, expected_solution_2.to_string()),
        ]
        .into_iter()
    }
}
