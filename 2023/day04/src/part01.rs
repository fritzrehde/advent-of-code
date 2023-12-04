use anyhow::Result;
use derive_more::From;
use itertools::{EitherOrBoth, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, multispace0, multispace1},
    combinator::map_res,
    error::Error as NomError,
    multi::separated_list0,
    sequence::preceded,
    Finish, IResult,
};
use std::str;

#[derive(Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct ScratchCard {
    _id: usize,
    winning_numbers: Numbers,
    picked_numbers: Numbers,
}

impl ScratchCard {
    /// Parse from `Card  {id}:{winning_numbers} |{picked_numbers}`.
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Card")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, id) = map_res(digit1, str::parse::<usize>)(input)?;
        let (input, _) = char(':')(input)?;
        let (input, winning_numbers) = Numbers::parse(input)?;
        let (input, _) = tag(" |")(input)?;
        let (input, picked_numbers) = Numbers::parse(input)?;

        Ok((
            input,
            ScratchCard {
                _id: id,
                winning_numbers,
                picked_numbers,
            },
        ))
    }
}

// TODO: this is boilerplate code that should be covered by nom
impl str::FromStr for ScratchCard {
    type Err = NomError<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s).finish() {
            Ok((_remaining, scatchcard)) => Ok(scatchcard),
            Err(NomError { input, code }) => Err(NomError {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[test]
fn test_parsing_scratchcard() {
    let scratchcard = ScratchCard {
        _id: 1,
        winning_numbers: vec![41, 48, 83, 86, 17].into(),
        picked_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53].into(),
    };
    assert_eq!(
        scratchcard,
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
            .parse()
            .expect("should parse")
    );
    assert_eq!(
        scratchcard,
        // The can be variable-length whitespace between "Card" and its id.
        "Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
            .parse()
            .expect("should parse")
    );
}

impl ScratchCard {
    /// Calculate how many points a card is worth.
    fn points_worth(&self) -> usize {
        let shared_numbers = self.shared_numbers().count();
        if shared_numbers == 0 {
            0
        } else {
            // The first match makes the card worth one point and each match
            // after the first doubles the point value of that card.
            let exp = shared_numbers - 1;
            // TODO: remove unwrap
            2usize.pow(u32::try_from(exp).unwrap())
        }
    }

    /// Calculate how many shared numbers there are between the
    /// `winning_numbers` and the `picked_numbers`.
    fn shared_numbers(&self) -> impl Iterator<Item = &usize> {
        // Sort both iters to ensure `merge_join_by` works.
        let winning_numbers_iter = self.winning_numbers.0.iter().sorted();
        let picked_numbers_iter = self.picked_numbers.0.iter().sorted();
        winning_numbers_iter
            .merge_join_by(picked_numbers_iter, |a, b| Ord::cmp(a, b))
            .filter_map(|either| match either {
                EitherOrBoth::Both(a, _) => Some(a),
                _ => None,
            })
    }
}

#[test]
fn test_shared_numbers() {
    let scratchcard = ScratchCard {
        _id: 1,
        winning_numbers: vec![41, 48, 83, 86, 17].into(),
        picked_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53].into(),
    };
    let shared_numbers = [48, 83, 86, 17].iter();
    // Sort both iters so only the content is compared, not the order.
    itertools::assert_equal(
        shared_numbers.sorted(),
        scratchcard.shared_numbers().sorted(),
    );
}

#[derive(From, Debug)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct Numbers(Vec<usize>);

impl Numbers {
    /// Parse from ` 3 86  6 31 17  9 48 53` (numbers with variable-length
    /// whitespace separators, and possible whitespace at the beginning).
    fn parse(input: &str) -> IResult<&str, Self> {
        let number_parser = map_res(digit1, str::parse::<usize>);
        let numbers_parser = separated_list0(multispace1, number_parser);
        let remove_leading_whitespace_parser = preceded(multispace0, numbers_parser);
        let mut parser = remove_leading_whitespace_parser;

        let (input, numbers) = parser(input)?;

        Ok((input, Numbers(numbers)))
    }
}

#[test]
fn test_parsing_numbers() {
    let numbers: Numbers = vec![3, 86, 6, 31, 17, 9, 48, 53].into();
    assert_eq!(
        ("", numbers),
        Numbers::parse(" 3 86  6 31 17  9 48 53").expect("should parse")
    );
}

pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let points_worth: usize = puzzle_input
        .lines()
        .map(|line| line.parse::<ScratchCard>().unwrap())
        .map(|scratch_card| scratch_card.points_worth())
        .sum();

    Ok(points_worth.to_string())
}
