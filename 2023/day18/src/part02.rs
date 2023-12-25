use derive_new::new;
use grid::Grid;
use itertools::{Itertools, Position};
use parse_display::{Display, FromStr};
use std::{fmt, str};
use strum::{EnumIs, EnumIter, IntoEnumIterator};
use vec1::vec1;

#[derive(Debug)]
struct DigPlan(Vec<DigInstruction>);

impl str::FromStr for DigPlan {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dig_insts = s
            .lines()
            .map(|line| {
                line.parse::<DigInstructionParsed>()
                    .map(DigInstruction::from)
            })
            .collect::<Result<_, _>>()?;
        Ok(Self(dig_insts))
    }
}

impl DigPlan {
    /// Starting with the `starting_hole`, execute all instructions in the
    /// `DigPlan`, and then dig out the interior terrain.
    fn trench_grid(&self, starting_hole: SignedPositionInGrid) -> TerrainGrid {
        let trenches = self.dig_trenches(starting_hole);

        let mut terrain_grid = self.grid(trenches);
        terrain_grid.dig_interior();

        terrain_grid
    }

    /// Starting with the `starting_hole`, execute all instructions in the
    /// `DigPlan` and return the resulting trench positions.
    fn dig_trenches(&self, starting_hole: SignedPositionInGrid) -> Vec<SignedPositionInGrid> {
        let mut trench_positions = vec1![starting_hole];

        for dig_inst in self.0.iter() {
            let next_positions = dig_inst.execute(trench_positions.last());
            trench_positions.extend(next_positions);
        }

        trench_positions.into()
    }

    /// Convert the positions of the trenches into a `TerrainGrid`.
    fn grid(&self, trenches: Vec<SignedPositionInGrid>) -> TerrainGrid {
        let cmp_row = |p1: &&SignedPositionInGrid, p2: &&SignedPositionInGrid| p1.row.cmp(&p2.row);
        let cmp_col = |p1: &&SignedPositionInGrid, p2: &&SignedPositionInGrid| p1.col.cmp(&p2.col);

        let first_row = trenches.iter().min_by(cmp_row).map_or(0, |pos| pos.row);
        let last_row = trenches.iter().max_by(cmp_row).map_or(0, |pos| pos.row);
        let first_col = trenches.iter().min_by(cmp_col).map_or(0, |pos| pos.col);
        let last_col = trenches.iter().max_by(cmp_col).map_or(0, |pos| pos.col);

        let rows = last_row.abs_diff(first_row) + 1;
        let cols = last_col.abs_diff(first_col) + 1;

        // Build up an empty grid filled with `Terrain::GroundLevel`.
        let grid_vec = (0..rows)
            .flat_map(|row| {
                (0..cols).map(move |col| TerrainInGrid::new(PositionInGrid { row, col }))
            })
            .collect_vec();
        let mut grid = Grid::from_vec(grid_vec, cols);

        // Insert the trenches into the grid.
        for trench_pos in trenches {
            let pos = trench_pos.to_unsigned(first_row, first_col);
            if let Some(terrain_mut) = grid.get_mut(pos.row, pos.col) {
                terrain_mut.terrain = Terrain::Trench;
            }
        }

        TerrainGrid(grid)
    }
}

#[derive(Debug)]
struct TerrainGrid(Grid<TerrainInGrid>);

impl fmt::Display for TerrainGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (position, row) in self.0.iter_rows().with_position() {
            for terrain in row {
                write!(f, "{}", terrain)?;
            }
            if let Position::First | Position::Middle = position {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl TerrainGrid {
    /// Dig out all terrain that is inside the trenches. Upon completion, all
    /// terrain will be marked as either `Terrain::Trench`,
    /// `Terrain::InsideTrenches` or `Terrain::OutsideTrenches`.
    fn dig_interior(&mut self) {
        self.mark_terrain_outside_trench();

        // This is also valid if there is no exterior terrain at all.
        self.mark_remaining_terrain_as_inside_trench();
    }

    /// Mark all terrain that is outside the trench loop as
    /// `Terrain::OutsideTrenches`.
    fn mark_terrain_outside_trench(&mut self) {
        let is_ground_level_terrain = |row, col| {
            self.0.get(row, col).and_then(|terrain| {
                terrain
                    .terrain
                    .is_ground_level()
                    .then_some(terrain.position)
            })
        };

        for initial_exterior_terrain_pos in self.search_edges(is_ground_level_terrain) {
            self.flood_fill_outside_trenches(&initial_exterior_terrain_pos);
        }
    }

    /// Mark all remaining unmarked terrain (`Terrain::GroundLevel`)
    /// as `Terrain::InsideTrenches`.
    fn mark_remaining_terrain_as_inside_trench(&mut self) {
        self.0.iter_mut().for_each(|terrain_mut| {
            if terrain_mut.terrain.is_ground_level() {
                terrain_mut.terrain = Terrain::InsideTrenches
            }
        });
    }

    /// Search for all elements that satisify the `predicate` in the outer edges
    /// (in every direction) of the grid.
    fn search_edges<P>(&self, predicate: P) -> Vec<PositionInGrid>
    where
        P: Fn(GridIndex, GridIndex) -> Option<PositionInGrid>,
    {
        let top_row = 0;
        let bottom_row = self.0.rows().saturating_sub(1);
        let left_col = 0;
        let right_col = self.0.cols().saturating_sub(1);

        // TODO: try not to unnecessarily collect to vector. Maybe we can use an iterator instead somehow?
        let mut positions = vec![];

        // TODO: remove code duplication for searching each edge. make more generic.

        // Search top edge.
        for col in left_col..=right_col {
            if let Some(pos) = predicate(top_row, col) {
                positions.push(pos);
            }
        }

        // Search bottom edge.
        for col in (left_col..=right_col).rev() {
            if let Some(pos) = predicate(bottom_row, col) {
                positions.push(pos);
            }
        }

        // Search right edge.
        for row in top_row..=bottom_row {
            if let Some(pos) = predicate(row, right_col) {
                positions.push(pos);
            }
        }

        // Search left edge.
        for row in (top_row..=bottom_row).rev() {
            if let Some(pos) = predicate(row, left_col) {
                positions.push(pos);
            }
        }

        positions
    }
}

#[derive(Debug, new, Display)]
#[display("{terrain}")]
struct TerrainInGrid {
    #[new(default)]
    terrain: Terrain,
    position: PositionInGrid,
}

impl TerrainGrid {
    /// Given the position of a `Terrain::OutsideTrenches` object, use the
    /// `flood fill` algorithm to fill all touching terrain outside the trench
    /// loop with `Terrain::OutsideTrenches`.
    fn flood_fill_outside_trenches(&mut self, terrain_pos: &PositionInGrid) {
        match self.0.get_mut(terrain_pos.row, terrain_pos.col) {
            Some(terrain_mut) => match terrain_mut.terrain {
                Terrain::GroundLevel => terrain_mut.terrain = Terrain::OutsideTrenches,
                Terrain::Trench | Terrain::OutsideTrenches => return,
                Terrain::InsideTrenches => {
                    unreachable!("inside trenches were found during outside trenches flood fill")
                }
            },
            None => return,
        }

        for dir in Direction::iter() {
            if let Some(unvalidated_next_pos) = dir.translate(terrain_pos) {
                if let Some(next_pos) = self
                    .0
                    .get(unvalidated_next_pos.row, unvalidated_next_pos.col)
                    .map(|terrain| terrain.position)
                {
                    self.flood_fill_outside_trenches(&next_pos);
                }
            }
        }
    }
}

#[derive(Debug, Display, Default, EnumIs)]
enum Terrain {
    #[default]
    #[display(".")]
    GroundLevel,

    #[display("#")]
    Trench,

    #[display("o")]
    OutsideTrenches,

    #[display("i")]
    InsideTrenches,
}

#[derive(Debug, new, PartialEq, Eq)]
struct DigInstruction {
    direction: Direction,
    steps: isize,
}

impl str::FromStr for DigInstruction {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: use nom for better parsing, this is very error prone and would throw terrible error messages
        let steps = isize::from_str_radix(&s[1..6], 16)?;
        let direction = match s[6..=6].parse::<usize>()? {
            0 => Direction::East,
            1 => Direction::South,
            2 => Direction::West,
            3 => Direction::North,
            dir_index => anyhow::bail!("Unexpected direction index: {}", dir_index),
        };
        Ok(Self { direction, steps })
    }
}

#[test]
fn test_parsing_color() {
    assert_eq!(
        DigInstruction::new(Direction::East, 461937),
        "#70c710".parse().unwrap()
    );
    assert_eq!(
        DigInstruction::new(Direction::South, 56407),
        "#0dc571".parse().unwrap()
    );
    assert_eq!(
        DigInstruction::new(Direction::East, 356671),
        "#5713f0".parse().unwrap()
    );
}

impl DigInstruction {
    /// Given a starting position, execute this instruction, and return all
    /// visited positions.
    fn execute(
        &self,
        start_position: &SignedPositionInGrid,
    ) -> impl Iterator<Item = SignedPositionInGrid> {
        self.direction.translate_steps(start_position, self.steps)
    }
}

#[derive(Debug, FromStr)]
#[display("{_direction} {_steps} ({color})")]
struct DigInstructionParsed {
    _direction: Direction,
    _steps: isize,
    color: DigInstruction,
}

impl From<DigInstructionParsed> for DigInstruction {
    fn from(dig_inst_parsed: DigInstructionParsed) -> Self {
        dig_inst_parsed.color
    }
}

type GridIndex = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new)]
struct PositionInGrid {
    row: GridIndex,
    col: GridIndex,
}

type SignedGridIndex = isize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, new)]
struct SignedPositionInGrid {
    row: SignedGridIndex,
    col: SignedGridIndex,
}

impl SignedPositionInGrid {
    /// Convert this position to a representation where the row and column
    /// grid indexes start at 0.
    fn to_unsigned(self, first_row: SignedGridIndex, first_col: SignedGridIndex) -> PositionInGrid {
        let row = first_row.abs_diff(self.row);
        let col = first_col.abs_diff(self.col);
        PositionInGrid { row, col }
    }
}

#[derive(Debug, FromStr, PartialEq, Eq, Copy, Clone, Hash, EnumIter)]
enum Direction {
    #[display("U")]
    North,

    #[display("R")]
    East,

    #[display("D")]
    South,

    #[display("L")]
    West,
}

impl Direction {
    /// Translate moving a certain number of `steps` into this direction in a
    /// grid to changes to the row and column indexes. Return all newly visited
    /// positions in the grid, where the last iterator element is the furthest
    /// away from the initial `pos`. Note that the grid is not expected to have
    /// fixed margins, so returning negative indexes is valid.
    fn translate_steps(
        &self,
        pos: &SignedPositionInGrid,
        steps: isize,
    ) -> impl Iterator<Item = SignedPositionInGrid> {
        // TODO: unnecessary collecting into vec, was done because .rev() returns different type
        let no_step = |idx| (idx..=idx).collect_vec();
        let step_forward = |idx| ((idx + 1)..=(idx + steps)).collect_vec();
        let step_backward = |idx| ((idx - steps)..idx).rev().collect_vec();

        let (row_range, col_range) = match self {
            Self::North => (step_backward(pos.row), no_step(pos.col)),
            Self::South => (step_forward(pos.row), no_step(pos.col)),
            Self::East => (no_step(pos.row), step_forward(pos.col)),
            Self::West => (no_step(pos.row), step_backward(pos.col)),
        };

        row_range.into_iter().flat_map(move |row| {
            col_range
                .clone()
                .into_iter()
                .map(move |col| SignedPositionInGrid { row, col })
        })
    }

    /// Translate moving in this direction in a grid to changes to the row and
    /// column indexes. This will never return the same position back. If going
    /// into some direction would be outside the grid bounds, return `None`.
    fn translate(&self, pos: &PositionInGrid) -> Option<PositionInGrid> {
        let no_step = |idx| idx;
        let step_forward = |idx| idx + 1;
        let step_backward = |idx: usize| idx.checked_sub(1);

        let (row, col) = match self {
            Self::North => (step_backward(pos.row)?, no_step(pos.col)),
            Self::South => (step_forward(pos.row), no_step(pos.col)),
            Self::East => (no_step(pos.row), step_forward(pos.col)),
            Self::West => (no_step(pos.row), step_backward(pos.col)?),
        };

        Some(PositionInGrid { row, col })
    }
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let dig_plan: DigPlan = puzzle_input.parse()?;

    let starting_hole = SignedPositionInGrid::new(0, 0);

    let terrain_grid = dig_plan.trench_grid(starting_hole);

    println!("{}", terrain_grid);

    let inside_trench_count: usize = terrain_grid
        .0
        .iter()
        .filter(|terrain| matches!(terrain.terrain, Terrain::InsideTrenches | Terrain::Trench))
        .count();

    Ok(inside_trench_count.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide multiple example details as `[(puzzle input, expected solution)]`.
    pub fn example_details() -> impl Iterator<Item = (&'static str, String)> {
        let puzzle_input = indoc! {"
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        "};

        // Just the trench line:
        // sadly way too large to display :(

        let expected_solution: usize = 952408144115;

        [(puzzle_input, expected_solution.to_string())].into_iter()
    }
}
