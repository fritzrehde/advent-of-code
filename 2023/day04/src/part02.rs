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

#[derive(Debug, Hash, Eq, PartialEq)]
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

/// The resulting metadata extracted from a `ScratchCard`.
#[derive(Debug)]
struct ScratchCardMetadata {
    /// Number of matches in the scratchcard.
    matches: usize,
    /// Number of total instances (original + copies) of this scratchcard that
    /// have been won.
    cards_won: usize,
}

impl ScratchCardMetadata {
    fn new(scratchcard: ScratchCard) -> Self {
        Self {
            matches: scratchcard.shared_numbers().count(),
            cards_won: 1,
        }
    }
}

#[derive(From, Debug, Hash, Eq, PartialEq)]
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
    let mut scratchcards: Vec<ScratchCardMetadata> = puzzle_input
        .lines()
        .map(|line| line.parse::<ScratchCard>().unwrap())
        .map(ScratchCardMetadata::new)
        .collect();

    let len = scratchcards.len();
    for i in 0..len {
        let cards_won = scratchcards[i].cards_won;
        let won_copies = scratchcards[i].matches;

        for card in scratchcards.iter_mut().skip(i + 1).take(won_copies) {
            card.cards_won += cards_won;
        }

        // Less idiomatic, but possibly more performant.
        // for j in (i + 1)..(i + 1 + won_copies) {
        //     if let Some(card) = scratchcards.get_mut(j) {
        //         card.cards_won += cards_won;
        //     }
        // }
    }

    let total_scratchcards: usize = scratchcards
        .iter()
        .map(|ext_scratchcard| ext_scratchcard.cards_won)
        .sum();

    Ok(total_scratchcards.to_string())
}
