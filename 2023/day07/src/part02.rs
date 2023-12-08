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

#[derive(Derivative, Debug, FromStr)]
#[derivative(PartialEq, Eq, PartialOrd, Ord)]
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

        // TODO: these algorithms could definitely be improved: Currently, we check whether a hand has a certain type separately for each type, which involves some duplication of effort, but presents a clean, isolated solution for each type. This could be improved by using sth like `Lazy` to calculate certain things once, and then reuse them without recalculating. Examples include `non_j_cards()` calls.

        // Get number/count of `Card::J` cards.
        let j_count = card_counts.get(&Card::J).unwrap_or(&0);

        // Get an iterator over all non-`Card::J` cards.
        let non_j_cards = || {
            card_counts
                .iter()
                // Remove all `Card::J` cards.
                .filter_map(|(&card, count)| (card != &Card::J).then_some(count))
        };

        // Check whether the hand contains an n-of-a-kind type,
        // taking into account the special nature of `Card::J`.
        let is_n_of_a_kind = |n: usize| {
            let non_j_count = n - j_count;
            if non_j_count == 0 {
                // We have JJJJJ, so we can make any configuration.
                return true;
            }
            non_j_cards().any(|&count| count == non_j_count)
        };

        let is_five_of_a_kind = || is_n_of_a_kind(5);
        let is_four_of_a_kind = || is_n_of_a_kind(4);
        let is_three_of_a_kind = || is_n_of_a_kind(3);

        let is_full_house = || {
            // All possible patterns:
            // [(5J), (x, (5-x)J), (2, 3, 0J), (2, 2, 1J), (1, 3, 1J), (1, 2, 2J)]
            // where (2, 2, 1J) means there is one `Card::J` card, and
            // two times two other cards, e.g. two `Card::A` and two `Card::K`.
            let non_j_different_card_labels = non_j_cards().count();
            non_j_different_card_labels <= 2
        };

        let is_two_pair = || {
            // All impossible patterns:
            // [(1, 1, 1, 1, 1, 0J), (1, 1, 1, 1, 1J), (1, 1, 1, 2, 0J)]
            let non_j_different_card_labels = non_j_cards().count();
            non_j_different_card_labels <= 3
        };

        let is_one_pair = || {
            // All impossible patterns:
            // [(1, 1, 1, 1, 1, 0J)]
            let non_j_different_card_labels = non_j_cards().count();
            non_j_different_card_labels <= 4
        };

        if is_five_of_a_kind() {
            HandType::FiveOfAKind
        } else if is_four_of_a_kind() {
            HandType::FourOfAKind
        } else if is_full_house() {
            HandType::FullHouse
        } else if is_three_of_a_kind() {
            HandType::ThreeOfAKind
        } else if is_two_pair() {
            HandType::TwoPair
        } else if is_one_pair() {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

#[test]
fn test_hand_type() {
    // FiveOfAKind

    // AAAAA
    let cards: Cards = [Card::A, Card::A, Card::A, Card::A, Card::A].into();
    assert_eq!(HandType::from(&cards), HandType::FiveOfAKind);

    // AAJJA => AAAAA
    let cards: Cards = [Card::A, Card::A, Card::J, Card::J, Card::A].into();
    assert_eq!(HandType::from(&cards), HandType::FiveOfAKind);

    // JJJJJ => EEEEE (where E is any valid Card)
    let cards: Cards = [Card::J, Card::J, Card::J, Card::J, Card::J].into();
    assert_eq!(HandType::from(&cards), HandType::FiveOfAKind);

    // EJJJJ => EEEEE (where E is any valid Card)
    let cards: Cards = [Card::A, Card::J, Card::J, Card::J, Card::J].into();
    assert_eq!(HandType::from(&cards), HandType::FiveOfAKind);

    // FourOfAKind

    // KTTTT
    let cards: Cards = [Card::K, Card::T, Card::T, Card::T, Card::T].into();
    assert_eq!(HandType::from(&cards), HandType::FourOfAKind);

    // KTJJT => KTTTT
    let cards: Cards = [Card::K, Card::T, Card::J, Card::J, Card::T].into();
    assert_eq!(HandType::from(&cards), HandType::FourOfAKind);

    // KTJJJ => KTTTT
    let cards: Cards = [Card::K, Card::T, Card::J, Card::J, Card::J].into();
    assert_eq!(HandType::from(&cards), HandType::FourOfAKind);

    // FullHouse

    // KTKTT
    let cards: Cards = [Card::K, Card::T, Card::K, Card::T, Card::T].into();
    assert_eq!(HandType::from(&cards), HandType::FullHouse);

    // KTKJT => KTK(K|T)T
    let cards: Cards = [Card::K, Card::T, Card::K, Card::J, Card::T].into();
    assert_eq!(HandType::from(&cards), HandType::FullHouse);

    // ThreeOfAKind

    // AAAKT
    let cards: Cards = [Card::A, Card::A, Card::A, Card::K, Card::T].into();
    assert_eq!(HandType::from(&cards), HandType::ThreeOfAKind);

    // AJJKT => AAAKT | AKKKT | ATTKT
    let cards: Cards = [Card::A, Card::J, Card::J, Card::K, Card::T].into();
    assert_eq!(HandType::from(&cards), HandType::ThreeOfAKind);

    // TwoPair

    // AAKKT
    let cards: Cards = [Card::A, Card::A, Card::K, Card::K, Card::T].into();
    assert_eq!(HandType::from(&cards), HandType::TwoPair);

    // NOTE: Impossible to get a `TwoPair` with only one `J`. Any more `J`s
    // will always result in a `ThreeOfAKind`.

    // OnePair

    // AAT9K
    let cards: Cards = [Card::A, Card::A, Card::T, Card::Number(9), Card::K].into();
    assert_eq!(HandType::from(&cards), HandType::OnePair);

    // AKT9J => AKT9A | AKT9K | AKT9T
    let cards: Cards = [Card::A, Card::K, Card::T, Card::Number(9), Card::J].into();
    assert_eq!(HandType::from(&cards), HandType::OnePair);
}

#[derive(FromStr, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[display(style = "UPPERCASE")]
enum Card {
    J,
    #[display("{0}")]
    Number(usize),
    T,
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
