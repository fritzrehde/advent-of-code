use derivative::Derivative;
use derive_more::From;
use derive_new::new;
use itertools::Itertools;
use nom::{
    character::complete::anychar,
    combinator::{all_consuming, map_res},
    error::Error as NomError,
    multi::count,
    Finish, IResult,
};
use parse_display::FromStr;
use std::collections::HashMap;
use std::str;

#[derive(Derivative, Debug, FromStr, Eq, Ord)]
#[derivative(PartialEq, PartialOrd)]
#[display("{hand} {bid}")]
struct HandWithBid {
    hand: Hand,
    #[derivative(PartialEq = "ignore", PartialOrd = "ignore")]
    bid: usize,
}

#[test]
fn test_parse_hand_with_bid() {
    let cards: Cards = [
        Card::Number(3),
        Card::Number(2),
        Card::T,
        Card::Number(3),
        Card::K,
    ]
    .into();
    let hand_type = HandType::OnePair;
    let hand = Hand { cards, hand_type };
    let bid = 765;
    let hand_with_bid = HandWithBid { hand, bid };
    assert_eq!(hand_with_bid, "32T3K 765".parse().expect("should parse"));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, new)]
struct Hand {
    hand_type: HandType,
    cards: Cards,
}

impl str::FromStr for Hand {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Cards = s.parse()?;
        let hand_type: HandType = (&cards).into();
        Ok(Self { cards, hand_type })
    }
}

/// The cards that make up one hand.
#[derive(Debug, From, PartialEq, Eq, PartialOrd, Ord)]
struct Cards([Card; 5]);

impl Cards {
    /// Parse from `{Card}{Card}{Card}{Card}{Card}`.
    fn parse(input: &str) -> IResult<&str, Self> {
        let card_parser = map_res(anychar, |c| c.to_string().as_str().parse::<Card>());
        let card_vec_parser = all_consuming(count(card_parser, 5));
        let mut card_array_parser = map_res(card_vec_parser, Vec::try_into);

        let (input, cards) = card_array_parser(input)?;

        Ok((input, Self(cards)))
    }
}

// TODO: this is boilerplate code that should be covered by nom
impl str::FromStr for Cards {
    type Err = NomError<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s).finish() {
            Ok((_remaining, cards)) => Ok(cards),
            Err(NomError { input, code }) => Err(NomError {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[test]
fn test_ordering_hand_type() {
    assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
}

impl From<&Cards> for HandType {
    fn from(cards: &Cards) -> Self {
        // Count the number of occurences of each card.
        let mut card_counts: HashMap<&Card, usize> = HashMap::new();
        for card in cards.0.iter() {
            *card_counts.entry(card).or_insert(0) += 1;
        }

        let five_of_a_kind = || card_counts.values().any(|&count| count == 5);
        let four_of_a_kind = || card_counts.values().any(|&count| count == 4);
        let full_house = || itertools::equal([&2, &3], card_counts.values().sorted());
        let three_of_a_kind = || card_counts.values().any(|&count| count == 3);
        let two_pair = || card_counts.values().filter(|&count| *count == 2).count() == 2;
        let one_pair = || card_counts.values().filter(|&count| *count == 2).count() == 1;

        return if five_of_a_kind() {
            HandType::FiveOfAKind
        } else if four_of_a_kind() {
            HandType::FourOfAKind
        } else if full_house() {
            HandType::FullHouse
        } else if three_of_a_kind() {
            HandType::ThreeOfAKind
        } else if two_pair() {
            HandType::TwoPair
        } else if one_pair() {
            HandType::OnePair
        } else {
            HandType::HighCard
        };
    }
}

#[derive(FromStr, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display(style = "UPPERCASE")]
enum Card {
    #[display("{0}")]
    Number(usize),
    T,
    J,
    Q,
    K,
    A,
}

#[test]
fn test_parse_card() {
    assert_eq!(Card::A, "A".parse().expect("should parse"));
    assert!("a".parse::<Card>().is_err());

    assert_eq!(Card::Number(9), "9".parse().expect("should parse"));
}

#[test]
fn test_ordering_card() {
    assert!(Card::A > Card::K);
    assert!(Card::Number(9) > Card::Number(8));
}

pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let total_winnings: usize = puzzle_input
        .lines()
        .map(|line| line.parse::<HandWithBid>().unwrap())
        .sorted()
        // Give each hand a rank.
        .enumerate()
        // The rank starts at 1, not 0.
        .map(|(i, hand)| (i + 1, hand))
        .map(|(rank, hand)| hand.bid * rank)
        .sum();

    Ok(total_winnings.to_string())
}
