use derive_more::From;
use derive_new::new;
use itertools::izip;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1, newline},
    combinator::map_res,
    error::{make_error, Error as NomError, ErrorKind},
    multi::separated_list0,
    sequence::preceded,
    Finish, IResult,
};
use std::str;

#[derive(Debug, From)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct Races(Vec<Race>);

impl Races {
    /// Parse from:
    /// ```
    /// Time:      7  15   30
    /// Distance:  9  40  200
    /// ```
    fn parse(input: &str) -> IResult<&str, Self> {
        let number_parser = map_res(digit1, str::parse::<usize>);
        let numbers_parser = separated_list0(multispace1, number_parser);
        let remove_leading_whitespace_parser = preceded(multispace0, numbers_parser);
        let mut numbers_parser = remove_leading_whitespace_parser;

        // TODO: avoid collecting `times` and `distances` into Vec's, since they're being iterated over afterwards anyways

        let (input, _) = tag("Time:")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, times) = numbers_parser(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = tag("Distance:")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, distances) = numbers_parser(input)?;

        if times.len() != distances.len() {
            // TODO: fix error reporting
            // bail!("Expected input to have equal time and distance elements, but times: {}, distances: {}", times.len(), distances.len());
            return Err(nom::Err::Error(make_error(input, ErrorKind::Count)));
        }

        let races = izip!(times, distances)
            .map(|(total_time_ms, record_distance_mm)| Race::new(total_time_ms, record_distance_mm))
            .collect();

        Ok((input, Races(races)))
    }
}

// TODO: this is boilerplate code that should be covered by nom
impl str::FromStr for Races {
    type Err = NomError<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s).finish() {
            Ok((_remaining, races)) => Ok(races),
            Err(NomError { input, code }) => Err(NomError {
                input: input.to_string(),
                code,
            }),
        }
    }
}

#[test]
fn test_parsing_races() {
    use indoc::indoc;

    let races: Races = vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)].into();
    let s = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};
    assert_eq!(races, s.parse().expect("should parse"));
}

// TODO: it would be cool to use dedicated `Milliseconds`, `Millimeters` and `Millimeters/Milliseconds` types instead of usize, to ensure type safety

#[derive(Debug, new)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct Race {
    /// The total length in time of the race, in milliseconds.
    total_time: usize,
    /// The record distance in this race, in millimeters.
    record_distance: usize,
}

impl Race {
    /// Calculate the number of record beating race possibilities.
    fn record_beating_possibilities(&self) -> usize {
        self.race_possibilities()
            .filter(|possible_race| possible_race.beats_record(self.record_distance))
            .count()
    }

    /// Get all possible configurations/outcomes of a race.
    fn race_possibilities(&self) -> impl Iterator<Item = PossibleRace> + '_ {
        let min_holding_time_ms = 0;
        let max_holding_time_ms = self.total_time;

        // All possible different `holding_time`s.
        (min_holding_time_ms..=max_holding_time_ms)
            .map(|holding_time_ms| PossibleRace::new(holding_time_ms, self.total_time))
    }
}

/// There are different possibilities of how a race can go.
/// For each whole millisecond you spend at the beginning of the race holding
/// down the button (`holding_time`), the boat's speed increases by one
/// millimeter per millisecond.
#[derive(new)]
struct PossibleRace {
    holding_time: usize,
    total_race_time: usize,
}

impl PossibleRace {
    /// Check if this race possibility beats the record distance.
    fn beats_record(&self, record_distance: usize) -> bool {
        self.distance_covered() > record_distance
    }

    /// Calculate the total distance covered in a `total_time` amount of time.
    fn distance_covered(&self) -> usize {
        let moving_speed = self.speed();
        let moving_time = self.total_race_time - self.holding_time;
        moving_time * moving_speed
    }

    /// Calculate the speed the boat will reach if it is held according to the
    /// `holding_time`.
    fn speed(&self) -> usize {
        self.holding_time
    }
}

pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let races: Races = puzzle_input.parse()?;
    let product: usize = races
        .0
        .iter()
        .map(Race::record_beating_possibilities)
        .product();
    Ok(product.to_string())
}
