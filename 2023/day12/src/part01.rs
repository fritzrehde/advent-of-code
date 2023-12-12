use anyhow::Result;
use derive_more::From;
use derive_new::new;
use itertools::Itertools;
use parse_display::FromStr;
use std::str;
use strum::EnumIs;

#[derive(Debug, FromStr, new)]
#[display("{springs} {damaged_spring_groups}")]
struct Line {
    springs: Springs,
    damaged_spring_groups: DamagedSpringGroups,
}

#[derive(Debug, From, Clone)]
struct DamagedSpringGroups(Vec<DamagedSpringGroup>);

// TODO: boilerplate that a crate should generate
impl str::FromStr for DamagedSpringGroups {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let springs = s
            .split(',')
            .map(|c| c.parse::<DamagedSpringGroup>())
            .collect::<Result<_, _>>()?;
        Ok(Self(springs))
    }
}

impl Springs {
    /// Check if the springs match the damaged groups pattern.
    fn matches_pattern(&self, damaged_spring_groups: &DamagedSpringGroups) -> bool {
        // Springs that contain `Spring::Unknown` can never be valid.
        if self.0.contains(&Spring::Unknown) {
            return false;
        }

        // TODO: remove unnecessary collect into vec, only used because comparing between &DamagingSpringGroup and DamagingSpringGroup is impossible
        let damaged_spring_groups_iter = self
            .0
            .iter()
            // Group by being bool `Spring::is_damaged`:
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

    assert!(springs.matches_pattern(&damaged_spring_groups));

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

    assert!(!springs.matches_pattern(&damaged_spring_groups));
}

#[derive(Debug, FromStr, PartialEq, Eq, Clone)]
struct DamagedSpringGroup(usize);

#[derive(Debug, From, Clone, Default)]
struct Springs(Vec<Spring>);

// TODO: boilerplate that a crate should generate
impl str::FromStr for Springs {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let springs = s
            .chars()
            .map(|c| c.to_string().as_str().parse::<Spring>())
            .collect::<Result<_, _>>()?;
        Ok(Self(springs))
    }
}

impl Springs {
    /// Get all possible configurations of these springs. This means every
    /// `Spring::Unknown` is replaced with the other possibilities, and all
    /// combinations of this replacing are captured.
    fn all_possible_configurations(&self) -> Vec<Self> {
        // TODO: optimization: we always know there will be 2^n configurations, so use Vec::with_capacity(2^n) to prevent reallocation
        let mut possible_configs = Vec::new();
        self.all_possible_configurations_builder(0, Self::default(), &mut possible_configs);
        possible_configs
    }

    /// A recursive helper that collects the results into `possible_configs`.
    fn all_possible_configurations_builder(
        &self,
        idx: usize,
        cur_config: Self,
        possible_configs: &mut Vec<Self>,
    ) {
        let mut recurse = |next_spring, mut new_cur_config: Springs| {
            new_cur_config.0.push(next_spring);
            self.all_possible_configurations_builder(idx + 1, new_cur_config, possible_configs)
        };

        match self.0.get(idx) {
            Some(next_spring) => match next_spring {
                Spring::Unknown => {
                    // Recurse on all possibilities.
                    recurse(Spring::Damaged, cur_config.clone());
                    recurse(Spring::Operational, cur_config);
                }
                known => {
                    recurse(known.clone(), cur_config);
                }
            },
            None => possible_configs.push(cur_config),
        }
    }
}

#[derive(Debug, FromStr, EnumIs, PartialEq, Eq, Clone)]
enum Spring {
    #[display(".")]
    Operational,

    #[display("#")]
    Damaged,

    #[display("?")]
    Unknown,
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let sum: usize = puzzle_input
        .lines()
        .map(|line| line.parse::<Line>().unwrap())
        .flat_map(|line| {
            line.springs
                .all_possible_configurations()
                .into_iter()
                .filter(move |possible_springs| {
                    possible_springs.matches_pattern(&line.damaged_spring_groups)
                })
        })
        .count();

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
        let expected_solution = 21;
        (puzzle_input, expected_solution.to_string())
    }
}
