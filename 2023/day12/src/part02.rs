use anyhow::Result;
use derive_more::From;
use derive_new::new;
use indicatif::ParallelProgressIterator;
use itertools::{izip, Itertools, Position};
use memoize::memoize;
use parse_display::{Display, FromStr};
use rayon::prelude::*;
use std::{cell::RefCell, collections::HashMap, fmt, iter, rc::Rc, str};
use strum::EnumIs;

/// The factor with which to expand/unfold each `Spring`.
// const UNFOLDING_FACTOR: usize = 5;
const UNFOLDING_FACTOR: usize = 2;

#[derive(Debug, FromStr, Display, new, Hash, Clone, PartialEq, Eq)]
#[display("{springs} {damaged_spring_groups}")]
struct Line {
    springs: Springs,
    damaged_spring_groups: DamagedSpringGroups,
}

#[memoize]
fn all_valid_configurations_builder(line: Line, idx: usize, mut cur_config: Springs) -> usize {
    let recurse = |next_spring, mut new_cur_config: Springs| {
        new_cur_config.0.push(next_spring);
        // TODO: inefficient clone
        all_valid_configurations_builder(line.clone(), idx + 1, new_cur_config)
    };

    // println!("{}", &cur_config);

    // Early exit if the start of the built-up configuration doesn't
    // match the grouping pattern.
    if !cur_config.start_is_valid(&line.damaged_spring_groups) {
        return 0;
    }

    match line.springs.0.get(idx) {
        Some(next_spring) => match next_spring {
            Spring::Unknown => {
                // Recurse on all possibilities.
                recurse(Spring::Damaged, cur_config.clone())
                    + recurse(Spring::Operational, cur_config)
            }
            known => recurse(known.clone(), cur_config),
        },
        None => {
            if cur_config.is_valid(&line.damaged_spring_groups) {
                // println!("found valid config: {}", cur_config);
                1
            } else {
                0
            }
        }
    }
}

// Spring => [(is_damaged, group_len)]
type GroupCacheType = HashMap<Springs, Vec<(bool, usize)>>;
type GroupCache = Rc<RefCell<GroupCacheType>>;

impl Line {
    /// Get number of valid configurations of these springs. This means every
    /// `Spring::Unknown` is replaced with the other possibilities, and all
    /// combinations of this replacing are captured. A valid configuration is
    /// one where the `Springs` match the `DamagedSpringGroups`.
    fn all_valid_configurations(self) -> usize {
        // println!("getting all valid configs for: {}", self);

        let mut group_cache: GroupCache = Rc::new(RefCell::new(HashMap::new()));

        // let num_valid_configs =
        //     self.all_valid_configurations_builder(0, Springs::default(), group_cache);

        let num_valid_configs = all_valid_configurations_builder(self, 0, Springs::default());

        // println!("valid configs len: {}", valid_configs.len());

        num_valid_configs
    }

    /// A recursive helper for `all_valid_configurations`.
    fn all_valid_configurations_builder(
        &self,
        idx: usize,
        mut cur_config: Springs,
        group_cache: GroupCache,
    ) -> usize {
        let recurse = |next_spring, mut new_cur_config: Springs| {
            new_cur_config.0.push(next_spring);
            self.all_valid_configurations_builder(idx + 1, new_cur_config, group_cache.clone())
        };

        // println!("{}", &cur_config);

        // Early exit if the start of the built-up configuration doesn't
        // match the grouping pattern.
        if !cur_config.start_is_valid(&self.damaged_spring_groups) {
            return 0;
        }

        match self.springs.0.get(idx) {
            Some(next_spring) => match next_spring {
                Spring::Unknown => {
                    // Recurse on all possibilities.
                    recurse(Spring::Damaged, cur_config.clone())
                        + recurse(Spring::Operational, cur_config)
                }
                known => recurse(known.clone(), cur_config),
            },
            None => {
                if cur_config.is_valid(&self.damaged_spring_groups) {
                    // println!("found valid config: {}", cur_config);
                    1
                } else {
                    0
                }
            }
        }
    }
}

#[derive(Debug, From, Clone, Hash, PartialEq, Eq)]
struct DamagedSpringGroups(Vec<DamagedSpringGroup>);

impl str::FromStr for DamagedSpringGroups {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let folded_groups: Vec<_> = s
            .split(',')
            .map(|c| c.parse::<DamagedSpringGroup>())
            .collect::<Result<_, _>>()?;

        let groups = iter::repeat(folded_groups)
            .take(UNFOLDING_FACTOR)
            .flatten()
            .collect_vec();

        Ok(Self(groups))
    }
}

impl fmt::Display for DamagedSpringGroups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (position, group) in self.0.iter().with_position() {
            write!(f, "{}", group)?;
            if let Position::First | Position::Middle = position {
                write!(f, ",")?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, FromStr, Display, PartialEq, Eq, Clone, PartialOrd, Ord, Hash)]
struct DamagedSpringGroup(usize);

#[derive(Debug, From, Clone, Default, Hash, PartialEq, Eq)]
struct Springs(Vec<Spring>);

impl str::FromStr for Springs {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let folded_springs: Vec<_> = s
            .chars()
            .map(|c| c.to_string().as_str().parse::<Spring>())
            .collect::<Result<_, _>>()?;

        let springs = iter::repeat(folded_springs)
            .take(UNFOLDING_FACTOR)
            .intersperse(vec![Spring::Unknown])
            .flatten()
            .collect_vec();

        Ok(Self(springs))
    }
}

impl fmt::Display for Springs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for spring in self.0.iter() {
            write!(f, "{}", spring)?;
        }
        Ok(())
    }
}

impl Springs {
    /// Check if the springs match the damaged groups pattern.
    fn is_valid(&self, damaged_spring_groups: &DamagedSpringGroups) -> bool {
        // Springs that contain `Spring::Unknown` can never be valid.
        if self.0.contains(&Spring::Unknown) {
            // dbg!("early return because of unknown");
            return false;
        }

        // TODO: remove unnecessary collect into vec, only used because comparing between &DamagingSpringGroup and DamagingSpringGroup is impossible
        let damaged_spring_groups_iter = self
            .0
            .iter()
            // Group by predicate `Spring::is_damaged`:
            // e.g. ###..## => [(damaged, count == 3), (not_damaged, count == 2), (damaged, count == 2))]
            .group_by(|spring| spring.is_damaged())
            .into_iter()
            .filter_map(|(is_damaged, group)| is_damaged.then_some(group))
            .map(|group| DamagedSpringGroup(group.count()))
            .collect_vec();

        itertools::equal(
            damaged_spring_groups_iter.iter(),
            damaged_spring_groups.0.iter(),
        )
    }

    /// Check if start of some springs match the damaged groups pattern.
    fn start_is_valid(&mut self, damaged_spring_groups: &DamagedSpringGroups) -> bool {
        // TODO: inefficient check, could easily optimize out by using the type system better (i.e. making another spring that can never be unknown)
        // Springs that contain `Spring::Unknown` can never be valid.
        if self.0.contains(&Spring::Unknown) {
            // dbg!("early exit due to unknown element");
            unreachable!();
            // return false;
        }

        // let group_cache_borrow = group_cache.borrow_mut();

        // let last = self.0.pop();
        // match group_cache_borrow.get(&self) {
        //     Some(cached) => {
        //         match last {
        //             Some(last) => cached.last_mut().map(|(is_damaged, count)| match last {
        //                 Spring::Damaged => {
        //                     if *is_damaged {
        //                         *count += 1
        //                     } else {
        //                         cached.push((true, 1))
        //                     }
        //                 }
        //                 Spring::Operational => {
        //                     if *is_damaged {
        //                         cached.push((false, 1))
        //                     } else {
        //                         *count += 1
        //                     }
        //                 }
        //                 Spring::Unknown => unreachable!(),
        //             }),
        //             None => todo!(),
        //         };
        //         // Push last element back.
        //         if let Some(last) = last {
        //             self.0.push(last);
        //         }

        //         //
        //         todo!();
        //     }
        //     None => todo!(),
        // }

        let groups = self
            .0
            .iter()
            // Group by predicate `Spring::is_damaged`:
            // e.g. ###..## => [(damaged, count == 3), (not_damaged, count == 2), (damaged, count == 2))]
            .group_by(|spring| spring.is_damaged());

        let damaged_spring_groups_iter = groups
            .into_iter()
            .filter_map(|(is_damaged, group)| is_damaged.then_some(group))
            .map(|group| DamagedSpringGroup(group.count()));

        // General approach. Doesn't scale well, because checks for non-last
        // elements are duplicated many times.
        izip!(damaged_spring_groups_iter, damaged_spring_groups.0.iter())
            .with_position()
            .all(|(position, (a, b))| match position {
                // The last damaged group may still be incomplete.
                Position::Last | Position::Only => &a <= b,
                _ => &a == b,
            })

        // Assume that all non-last groups were checked in previous calls to
        // this function, so we only have to analyze the last element.
        // let last = izip!(damaged_spring_groups_iter, damaged_spring_groups.0.iter()).last();
        // match last {
        //     Some((a, b)) => &a <= b,
        //     None => true,
        // }
    }
}

#[test]
fn test_springs_matches_groups_pattern() {
    // #.#.###
    let springs: Springs = vec![
        Spring::Damaged,
        Spring::Operational,
        Spring::Damaged,
        Spring::Operational,
        Spring::Damaged,
        Spring::Damaged,
        Spring::Damaged,
    ]
    .into();

    // 1,1,3
    let damaged_spring_groups: DamagedSpringGroups = vec![
        DamagedSpringGroup(1),
        DamagedSpringGroup(1),
        DamagedSpringGroup(3),
    ]
    .into();

    assert!(springs.is_valid(&damaged_spring_groups));

    // #.#.#.#
    let springs: Springs = vec![
        Spring::Damaged,
        Spring::Operational,
        Spring::Damaged,
        Spring::Operational,
        Spring::Damaged,
        Spring::Operational,
        Spring::Damaged,
    ]
    .into();

    // 1,1,3
    let damaged_spring_groups: DamagedSpringGroups = vec![
        DamagedSpringGroup(1),
        DamagedSpringGroup(1),
        DamagedSpringGroup(3),
    ]
    .into();

    assert!(!springs.is_valid(&damaged_spring_groups));
}

#[derive(Debug, FromStr, Display, EnumIs, PartialEq, Eq, Clone, Hash)]
enum Spring {
    #[display(".")]
    Operational,

    #[display("#")]
    Damaged,

    #[display("?")]
    Unknown,
}

enum KnownSpring {
    Operational,
    Damaged,
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    // Collected so .progress() can be called on a fixed size iterator.
    let lines = puzzle_input
        .lines()
        .map(|line| line.parse::<Line>().unwrap())
        .collect_vec();

    let sum: usize = lines
        .into_par_iter()
        .progress()
        // .inspect(|v| {
        //     // dbg!(v);
        //     println!("{}", v);
        // })
        .map(Line::all_valid_configurations)
        .sum();

    Ok(sum.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide the example details as `(puzzle input, expected solution)`.
    pub fn example_details() -> (&'static str, String) {
        let puzzle_input = indoc! {"
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        "};
        let expected_solution = 525152;
        (puzzle_input, expected_solution.to_string())
    }
}
