use derive_new::new;
use indicatif::ParallelProgressIterator;
use nom::{
    bytes::complete::take_while1,
    character::complete::{alpha1, char, digit1, newline},
    combinator::map_res,
    error::Error as NomError,
    multi::separated_list0,
    Finish, IResult,
};
use parse_display::FromStr;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{collections::HashMap, str};
use strum::EnumIs;

#[derive(Debug)]
struct Input {
    workflows: Workflows,
    ratings: Vec<Ratings>,
}

impl Input {
    /// Parse from:
    /// ```
    /// {workflows}
    ///
    /// {ratings}
    /// ```
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut workflows_parser = separated_list0(newline, ParsedWorkflow::parse);
        let mut ratings_parser = separated_list0(newline, Ratings::parse);

        let (input, parsed_workflows) = workflows_parser(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = newline(input)?;
        let (input, ratings) = ratings_parser(input)?;

        let workflows = Workflows(
            parsed_workflows
                .into_iter()
                .map(|wf| (wf.id, Workflow { rules: wf.rules }))
                .collect(),
        );

        Ok((input, Self { workflows, ratings }))
    }
}

// TODO: this is boilerplate code that should be covered by nom
impl str::FromStr for Input {
    type Err = NomError<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s).finish() {
            Ok((_remaining, parsed)) => Ok(parsed),
            Err(NomError { input, code }) => Err(NomError {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug, FromStr, PartialEq, Eq)]
enum Part {
    #[display("x")]
    ExtremelyCoolLooking,

    #[display("m")]
    Musical,

    #[display("a")]
    Aerodynamic,

    #[display("s")]
    Shiny,
}

#[derive(Debug)]
struct Workflows(HashMap<WorkflowId, Workflow>);

impl Ratings {
    fn evaluate(&self, workflows: &Workflows) -> FinalState {
        let starting_workflow_id = WorkflowId("in".to_string());

        let mut cur_workflow_id = &starting_workflow_id;

        loop {
            let workflow = workflows
                .0
                .get(cur_workflow_id)
                .expect("unknown workflow name/id");

            match workflow.rules.evaluate(self) {
                ActionOnPart::Accept => return FinalState::Accepted,
                ActionOnPart::Reject => return FinalState::Rejected,
                ActionOnPart::ForwardTo(next_workflow_id) => cur_workflow_id = next_workflow_id,
            };
        }
    }
}

#[derive(Debug)]
struct Workflow {
    rules: Rules,
}

#[derive(Debug, FromStr, PartialEq, Eq, Hash)]
struct WorkflowId(String);

#[derive(Debug)]
struct ParsedWorkflow {
    id: WorkflowId,
    rules: Rules,
}

impl ParsedWorkflow {
    /// Parse from `{id}{{rules}}`.
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut workflow_id_parser = map_res(alpha1, str::parse::<WorkflowId>);

        let (input, id) = workflow_id_parser(input)?;
        let (input, _) = char('{')(input)?;
        let (input, rules) = Rules::parse(input)?;
        let (input, _) = char('}')(input)?;

        Ok((input, Self { id, rules }))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Rules(Vec<Rule>);

impl Rules {
    /// Try evaluating each of the ratings according to these rules. Return
    /// the action that belongs to the first successful rule evaluation.
    fn evaluate(&self, ratings: &Ratings) -> &ActionOnPart {
        self
            .0
            .iter()
            .flat_map(|rule| ratings.0.iter().map(move |rating| (rule, rating)))
            .find_map(|(rule, rating)| rule.evaluate(rating))
            .expect("problem statement doesn't clarify what should happen if no rule is fulfilled for a certain rating")
    }
}

impl Rules {
    /// Parse from `{rule},{rule},...`.
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut rules_parser = separated_list0(char(','), Rule::parse);

        let (input, rules) = rules_parser(input)?;

        Ok((input, Self(rules)))
    }
}

#[test]
fn test_parse_rules() {
    let expected = Rules(vec![
        Rule::Greater {
            part: Part::Aerodynamic,
            num: 10,
            workflow: ActionOnPart::Accept,
        },
        Rule::Less {
            part: Part::ExtremelyCoolLooking,
            num: 42,
            workflow: ActionOnPart::Reject,
        },
        Rule::Unconditional {
            workflow: ActionOnPart::ForwardTo(WorkflowId("abc".to_string())),
        },
    ]);
    let (_leftover, observed) = Rules::parse("a>10:A,x<42:R,abc").unwrap();
    assert_eq!(expected, observed);
}

// TODO: remove code duplication between `Rule::Greater` and `Rule::Less`

/// Each rule specifies a condition and where to send the part if the
/// condition is true.
#[derive(Debug, FromStr, PartialEq, Eq)]
enum Rule {
    #[display("{part}>{num}:{workflow}")]
    Greater {
        part: Part,
        num: usize,
        workflow: ActionOnPart,
    },

    #[display("{part}<{num}:{workflow}")]
    Less {
        part: Part,
        num: usize,
        workflow: ActionOnPart,
    },

    #[display("{workflow}")]
    Unconditional { workflow: ActionOnPart },
}

impl Rule {
    /// Parse from `{part}={rating}`.
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut rule_parser = map_res(
            // TODO: very hacky, here we capture all chars that are valid in our parse::<Rule>. Instead, make a native nom parser for Rule.
            take_while1(|c: char| c.is_alphanumeric() || c == '<' || c == '>' || c == ':'),
            str::parse::<Rule>,
        );

        let (input, rule) = rule_parser(input)?;

        Ok((input, rule))
    }

    /// If this rule can be applied to this rating, evaluate the rule and
    /// return the next action.
    fn evaluate(&self, rating: &Rating) -> Option<&ActionOnPart> {
        match self {
            Rule::Unconditional { workflow } => Some(workflow),
            Rule::Greater {
                part,
                num,
                workflow,
            } if part == &rating.part && &rating.rating > num => Some(workflow),
            Rule::Less {
                part,
                num,
                workflow,
            } if part == &rating.part && &rating.rating < num => Some(workflow),
            _ => None,
        }
    }
}

/// The action to perform on a part.
#[derive(Debug, FromStr, PartialEq, Eq)]
enum ActionOnPart {
    #[display("A")]
    Accept,

    #[display("R")]
    Reject,

    #[display("{0}")]
    ForwardTo(WorkflowId),
}

/// The final state of a part.
#[derive(Debug, EnumIs)]
enum FinalState {
    Accepted,
    Rejected,
}

// TODO: optimization: make Ratings a [Rating; 4]

#[derive(Debug, PartialEq, Eq)]
struct Ratings(Vec<Rating>);

impl Ratings {
    /// Parse from `{{rating},{rating},...}`.
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut ratings_parser = separated_list0(char(','), Rating::parse);

        let (input, _) = char('{')(input)?;
        let (input, rules) = ratings_parser(input)?;
        let (input, _) = char('}')(input)?;

        Ok((input, Self(rules)))
    }
}

#[test]
fn test_parse_ratings() {
    let expected = Ratings(vec![
        Rating::new(Part::ExtremelyCoolLooking, 42),
        Rating::new(Part::Aerodynamic, 10),
    ]);
    let (_leftover, observed) = Ratings::parse("{x=42,a=10}").unwrap();
    assert_eq!(expected, observed);
}

#[derive(Debug, FromStr, new, PartialEq, Eq)]
#[display("{part}={rating}")]
struct Rating {
    part: Part,
    rating: usize,
}

impl Rating {
    /// Parse from `{part}={rating}`.
    fn parse(input: &str) -> IResult<&str, Self> {
        let mut part_parser = map_res(alpha1, str::parse::<Part>);
        let mut usize_parser = map_res(digit1, str::parse::<usize>);

        let (input, part) = part_parser(input)?;
        let (input, _) = char('=')(input)?;
        let (input, rating) = usize_parser(input)?;

        Ok((input, Self { part, rating }))
    }
}

#[test]
fn test_parse_rating() {
    assert_eq!(Rating::new(Part::Aerodynamic, 10), "a=10".parse().unwrap());
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let input: Input = puzzle_input.parse()?;

    let range = || 1..=4000;

    let all_combinations: Vec<(usize, usize, usize, usize)> = range()
        .flat_map(move |x| {
            range().flat_map(move |m| range().flat_map(move |a| range().map(move |s| (x, m, a, s))))
        })
        .collect();

    let accepted_combinations = all_combinations
        .into_par_iter()
        .progress()
        .map(|(x, m, a, s)| {
            Ratings(vec![
                Rating::new(Part::ExtremelyCoolLooking, x),
                Rating::new(Part::Musical, m),
                Rating::new(Part::Aerodynamic, a),
                Rating::new(Part::Shiny, s),
            ])
        })
        .filter(|ratings| ratings.evaluate(&input.workflows).is_accepted())
        .count();

    Ok(accepted_combinations.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide multiple example details as `[(puzzle input, expected solution)]`.
    pub fn example_details() -> impl Iterator<Item = (&'static str, String)> {
        let puzzle_input = indoc! {"
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        "};
        let expected_solution: usize = 167409079868000;

        [(puzzle_input, expected_solution.to_string())].into_iter()
    }
}
