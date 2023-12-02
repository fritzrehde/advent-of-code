use anyhow::{Error, Result};
use derive_more::From;
use derive_new::new;
use parse_display::{FromStr, ParseError};
use std::str;

const RED_CUBES_IN_BAG: usize = 12;
const GREEN_CUBES_IN_BAG: usize = 13;
const BLUE_CUBES_IN_BAG: usize = 14;

/// Each game is listed with its ID number followed by a semicolon-separated
/// list of subsets of cubes that were revealed from the bag.
#[derive(FromStr, new)]
#[display("Game {id}: {cube_subsets}")]
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
struct Game {
    id: usize,
    cube_subsets: CubePickingSubsets,
}

impl Game {
    /// Returns whether a `Game` is possible. A `Game` is impossible if any of
    /// the `CubePicking`s are impossible. Note that the cubes are placed back
    /// into the bag after every `CubePicking`, so each cube-picking is
    /// independent.
    fn is_possible(&self) -> bool {
        // TODO: avoid referencing .0, and instead use more idiomatic way of getting underlying type (maybe with deref?)
        for cube_subset in self.cube_subsets.0.iter() {
            for cube_picking in cube_subset.0.iter() {
                if !cube_picking.is_possible() {
                    return false;
                }
            }
        }
        true
    }
}

#[test]
fn test_parsing_game() {
    let game = Game {
        id: 1,
        cube_subsets: vec![
            vec![
                CubePicking::new(3, CubeColor::Blue),
                CubePicking::new(4, CubeColor::Red),
            ]
            .into(),
            vec![
                CubePicking::new(1, CubeColor::Red),
                CubePicking::new(2, CubeColor::Green),
                CubePicking::new(6, CubeColor::Blue),
            ]
            .into(),
            vec![CubePicking::new(2, CubeColor::Green)].into(),
        ]
        .into(),
    };

    assert_eq!(
        game,
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
            .parse()
            .expect("should parse")
    )
}

/// A semicolon-separated list of subsets of cube-pickings.
#[derive(From)]
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
struct CubePickingSubsets(Vec<CubePickingSubset>);

// TODO: this FromStr impl should be automated e.g. by parse_display
impl str::FromStr for CubePickingSubsets {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cube_subsets = s.split("; ").map(str::parse).collect::<Result<_>>()?;
        Ok(Self(cube_subsets))
    }
}

/// A comma-separated list of cube-pickings.
#[derive(From)]
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
struct CubePickingSubset(Vec<CubePicking>);

impl str::FromStr for CubePickingSubset {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cube_grabbed = s
            .split(", ")
            .map(str::parse)
            .collect::<Result<_, ParseError>>()?;
        Ok(Self(cube_grabbed))
    }
}

/// The picking of cube from the bag. A cube that has a `color` and is pulled
/// `occurences` number of times from the bag.
#[derive(FromStr, new)]
#[display("{occurences} {color}")]
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
struct CubePicking {
    occurences: usize,
    color: CubeColor,
}

impl CubePicking {
    /// Returns whether a `CubePicking` is possible. A `CubePicking` is
    /// impossible if the color of the picked cube occurs more often than number
    /// of cubes in that color that are in the bag.
    fn is_possible(&self) -> bool {
        let max_allowed_occurences = match self.color {
            CubeColor::Red => RED_CUBES_IN_BAG,
            CubeColor::Green => GREEN_CUBES_IN_BAG,
            CubeColor::Blue => BLUE_CUBES_IN_BAG,
        };
        self.occurences <= max_allowed_occurences
    }
}

/// The color of a cube.
#[derive(FromStr)]
#[display(style = "lowercase")]
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
enum CubeColor {
    Red,
    Green,
    Blue,
}

pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let ids_sum: usize = puzzle_input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .filter_map(|game| game.is_possible().then_some(game.id))
        .sum();
    Ok(ids_sum.to_string())
}
