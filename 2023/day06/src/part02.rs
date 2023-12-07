use derive_new::new;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1, newline},
    combinator::map_res,
    error::Error as NomError,
    multi::many1,
    sequence::preceded,
    Finish, IResult,
};
use std::str;

// TODO: it would be cool to use dedicated `Milliseconds`, `Millimeters` and `Millimeters/Milliseconds` types instead of usize, to ensure type safety

#[derive(Debug, new)]
#[cfg_attr(test, derive(Eq, PartialEq))]
struct Race {
    /// The total length in time of the race, in milliseconds.
    total_time: usize,
    /// The record distance in this race, in millimeters.
    record_distance: usize,
}

/// Parse a string with arbitary whitespace between digits into a usize,
/// e.g. `7  15   30` into `71530`.
fn parse_whitespace_separated_number(input: &str) -> IResult<&str, usize> {
    let digits_parser = many1(preceded(multispace0, digit1));
    map_res(digits_parser, |digits: Vec<&str>| {
        digits.concat().parse::<usize>()
    })(input)
}

impl Race {
    /// Parse from:
    /// ```
    /// Time:      7  15   30
    /// Distance:  9  40  200
    /// ```
    /// into Race { time: 71530, distance: 940200 }
    fn parse(input: &str) -> IResult<&str, Self> {
        let (input, _) = tag("Time:")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, total_time) = parse_whitespace_separated_number(input)?;
        let (input, _) = newline(input)?;
        let (input, _) = tag("Distance:")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, record_distance) = parse_whitespace_separated_number(input)?;

        Ok((
            input,
            Race {
                total_time,
                record_distance,
            },
        ))
    }
}

// TODO: this is boilerplate code that should be covered by nom
impl str::FromStr for Race {
    type Err = NomError<String>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match Self::parse(s).finish() {
            Ok((_remaining, race)) => Ok(race),
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

    let race = Race::new(71530, 940200);
    let s = indoc! {"
        Time:      7  15   30
        Distance:  9  40  200
    "};
    assert_eq!(race, s.parse().expect("should parse"));
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
    let race: Race = puzzle_input.parse()?;
    let record_beating_possibilities: usize = race.record_beating_possibilities();
    Ok(record_beating_possibilities.to_string())
}
