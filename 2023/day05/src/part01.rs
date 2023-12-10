use derive_more::From;
use derive_new::new;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, newline, not_line_ending, space1},
    combinator::map_res,
    multi::{many1, separated_list0},
    sequence::terminated,
    IResult,
};
use parse_display::FromStr;
use std::ops::Range;

#[derive(Debug, From)]
struct Seeds(Vec<Seed>);

impl Seeds {
    /// Parse from:
    /// ```
    /// seeds: 79 14 55 13
    /// ```
    fn parse(input: &str) -> IResult<&str, Seeds> {
        let seed_parser = map_res(digit1, str::parse::<Seed>);
        let mut seeds_parser = separated_list0(space1, seed_parser);

        let (input, _) = tag("seeds: ")(input)?;
        let (input, seeds) = seeds_parser(input)?;
        let (input, _) = newline(input)?;

        Ok((input, Seeds::from(seeds)))
    }
}

/// The number with which each quantity (seed, soil etc.) is identified.
type QuantityId = usize;

/// A seed, identified by an `Id`.
#[derive(Debug, FromStr, From)]
struct Seed(QuantityId);

impl Seed {
    /// Find the location number corresponding to this seed by traversing all
    /// maps until the end (the final map maps to locations).
    fn find_corresponding_location(&self, all_maps: &AllMaps) -> QuantityId {
        let initial_seed = self.0;
        let number = all_maps
            .0
            .iter()
            // Use previous `value` as key to find next value in map.
            .fold(initial_seed, |value, map| map.get(&value));

        number
    }
}

/// All maps of one quantity to the next quantity. The ordering of the maps is
/// important, but the names of individual maps are not, which is why they are
/// not stored. This means that e.g. the `seed-to-soil` and `soil-to-fertilizer`
/// maps only differ in their position in the underlying vector (0th and 1st,
/// respectively).
#[derive(Debug, From)]
struct AllMaps(Vec<Map>);

type Src = QuantityId;
type Dst = QuantityId;

/// A range-based hashmap-like structure that maps a `Src` quantity to a `Dst`
/// quantity, and provides `get(Src) -> Dst` for all any possible `Src` key.
/// Very large range lengths in the puzzle input make it infeasible to use
/// a regular `HashMap`.
#[derive(Debug)]
struct Map(Vec<MapRange>);

impl Map {
    /// Get the `Dst` value that belongs to the `Src` key. This will always
    /// succeed, because if the `Src->Dst` mapping is not stored in this `Map`
    /// directly, the `Src` should be returned.
    fn get(&self, src: &Src) -> Dst {
        for map_range in self.0.iter() {
            if let Some(dst) = map_range.get(src) {
                return dst;
            }
        }
        // TODO: maybe use Cow (copy on write), since we could also return src, which is &Src, here.
        *src
    }

    /// Parse from:
    /// ```
    /// {map_name} map:
    /// 50 98 2
    /// 52 50 48
    /// ...
    /// ```
    fn parse<'a>(map_name: &'a str, input: &'a str) -> IResult<&'a str, Self> {
        let parsed_map_parser = map_res(not_line_ending, str::parse::<MapParsed>);

        let (input, _) = tag(format!("{} map:", map_name).as_str())(input)?;
        let (input, _) = newline(input)?;
        let (input, parsed_maps) = many1(terminated(parsed_map_parser, line_ending))(input)?;

        Ok((input, Map::from(parsed_maps)))
    }
}

impl<I> From<I> for Map
where
    I: IntoIterator<Item = MapParsed>,
{
    fn from(parsed_maps: I) -> Self {
        let new_range = |start, len| start..(start + len);

        let map_ranges = parsed_maps
            .into_iter()
            .map(|map_parsed| {
                let src_range = new_range(map_parsed.src_range_start, map_parsed.range_len);
                let dst_first = map_parsed.dst_range_start;
                MapRange::new(src_range, dst_first)
            })
            .collect();

        Self(map_ranges)
    }
}

/// Map a range of `Src` values to a range of `Dst` values. Both ranges have
/// equal lengths.
#[derive(Debug, new)]
struct MapRange {
    /// The `Src` range.
    src_range: Range<Src>,
    /// The first value in the `Dst` range.
    dst_first: Dst,
}

impl MapRange {
    /// Get the `Dst` value that belongs to the `Src` key, if it exists.
    fn get(&self, src: &Src) -> Option<Dst> {
        self.src_range.contains(src).then(|| {
            let idx_into_src_range = src - self.src_range.start;
            self.dst_first + idx_into_src_range
        })
    }
}

#[derive(FromStr)]
#[display("{dst_range_start} {src_range_start} {range_len}")]
struct MapParsed {
    dst_range_start: usize,
    src_range_start: usize,
    range_len: usize,
}

/// Parse `Seeds` and `AllMaps` from:
/// ```
/// seeds: 79 14 55 13
///
/// seed-to-soil map:
/// 50 98 2
/// 52 50 48
/// ...
///
/// soil-to-fertilizer map:
/// 0 15 37
/// ...
///
/// fertilizer-to-water map:
/// 49 53 8
/// ...
///
/// water-to-light map:
/// 88 18 7
/// ...
///
/// light-to-temperature map:
/// 45 77 23
/// ...
///
/// temperature-to-humidity map:
/// 0 69 1
/// ...
///
/// humidity-to-location map:
/// 60 56 37
/// ...
/// ```
fn parse(input: &str) -> IResult<&str, (Seeds, AllMaps)> {
    let (input, seeds) = Seeds::parse(input)?;
    let (input, _) = newline(input)?;

    // TODO: remove this code duplication (instead, just iterate over a list of map names)

    let (input, map1) = Map::parse("seed-to-soil", input)?;
    let (input, _) = newline(input)?;

    let (input, map2) = Map::parse("soil-to-fertilizer", input)?;
    let (input, _) = newline(input)?;

    let (input, map3) = Map::parse("fertilizer-to-water", input)?;
    let (input, _) = newline(input)?;

    let (input, map4) = Map::parse("water-to-light", input)?;
    let (input, _) = newline(input)?;

    let (input, map5) = Map::parse("light-to-temperature", input)?;
    let (input, _) = newline(input)?;

    let (input, map6) = Map::parse("temperature-to-humidity", input)?;
    let (input, _) = newline(input)?;

    let (input, map7) = Map::parse("humidity-to-location", input)?;

    let all_maps: AllMaps = vec![map1, map2, map3, map4, map5, map6, map7].into();

    Ok((input, (seeds, all_maps)))
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    // TODO: remove unwrap (replacing with ? leads to a borrow-checking issue related to nom's result type)
    let (_leftover, (seeds, all_maps)) = parse(puzzle_input).unwrap();

    let lowest_location_number: usize = seeds
        .0
        .into_iter()
        .map(|seed| seed.find_corresponding_location(&all_maps))
        .min()
        .expect("there should be at least one seed");

    Ok(lowest_location_number.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide the example details as `(puzzle input, expected solution)`.
    pub fn example_details<'a>() -> (&'a str, String) {
        let puzzle_input = indoc! {"
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        "};
        let expected_solution = 35;
        (puzzle_input, expected_solution.to_string())
    }
}
