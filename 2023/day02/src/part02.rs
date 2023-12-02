use anyhow::{Error, Result};
use derive_more::From;
use derive_new::new;
use parse_display::{FromStr, ParseError};
use std::str;

/// Each game is listed with its ID number followed by a semicolon-separated
/// list of subsets of cubes that were revealed from the bag.
#[derive(FromStr, new)]
#[display("Game {_id}: {cube_subsets}")]
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
struct Game {
    _id: usize,
    cube_subsets: CubePickingSubsets,
}

impl Game {
    /// Find the `Bag` with the fewest number of cubes of each color that could
    /// have made this game possible.
    fn find_minimum_possible_bag(&self) -> Bag {
        self.cube_subsets
            .0
            .iter()
            .fold(Bag::new(0, 0, 0), |mut bag, cube_picking_subset| {
                if let Some(cube_picking) = &cube_picking_subset.red_cube {
                    bag.red_cubes = std::cmp::max(cube_picking.occurences, bag.red_cubes);
                }
                if let Some(cube_picking) = &cube_picking_subset.green_cube {
                    bag.green_cubes = std::cmp::max(cube_picking.occurences, bag.green_cubes);
                }
                if let Some(cube_picking) = &cube_picking_subset.blue_cube {
                    bag.blue_cubes = std::cmp::max(cube_picking.occurences, bag.blue_cubes);
                }
                bag
            })
    }
}

/// A bag of cubes that contains a certain amount of `red_cubes`, `green_cubes`
/// and `blue_cubes`.
#[derive(new)]
struct Bag {
    red_cubes: usize,
    green_cubes: usize,
    blue_cubes: usize,
}

impl Bag {
    /// The power of a set of cubes is equal to the numbers of red, green, and
    /// blue cubes multiplied together.
    fn power(&self) -> usize {
        self.red_cubes * self.green_cubes * self.blue_cubes
    }
}

#[test]
fn test_parsing_game() {
    let game = Game {
        _id: 1,
        cube_subsets: vec![
            CubePickingSubset::new(
                Some(CubePicking::new(4, CubeColor::Red)),
                None,
                Some(CubePicking::new(3, CubeColor::Blue)),
            ),
            CubePickingSubset::new(
                Some(CubePicking::new(1, CubeColor::Red)),
                Some(CubePicking::new(2, CubeColor::Green)),
                Some(CubePicking::new(6, CubeColor::Blue)),
            ),
            CubePickingSubset::new(None, Some(CubePicking::new(2, CubeColor::Green)), None),
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

/// A comma-separated list of cube-pickings, that contains at most one red,
/// green and blue cube.
#[derive(From, Default, new)]
#[cfg_attr(test, derive(PartialEq, Eq, Debug))]
struct CubePickingSubset {
    red_cube: Option<CubePicking>,
    green_cube: Option<CubePicking>,
    blue_cube: Option<CubePicking>,
}

impl str::FromStr for CubePickingSubset {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cube_picking_subset = s.split(", ").map(str::parse::<CubePicking>).try_fold(
            CubePickingSubset::default(),
            |mut acc, cube_picked_res| {
                let cube_picked = cube_picked_res?;

                // Set new `CubePicked`.
                let acc_color_cube = match cube_picked.color {
                    CubeColor::Red => &mut acc.red_cube,
                    CubeColor::Green => &mut acc.green_cube,
                    CubeColor::Blue => &mut acc.blue_cube,
                };
                *acc_color_cube = Some(cube_picked);

                Ok::<_, ParseError>(acc)
            },
        )?;

        Ok(cube_picking_subset)
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
    let set_power_sum: usize = puzzle_input
        .lines()
        .map(|line| line.parse::<Game>().unwrap())
        .map(|game| game.find_minimum_possible_bag())
        .map(|bag| bag.power())
        .sum();
    Ok(set_power_sum.to_string())
}
