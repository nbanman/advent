use advent::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Listing {
    source_start: i64,
    offset: i64,
    len: i64,
}

impl Listing {
    fn source_end(&self) -> i64 {
        self.source_start + self.len - 1
    }
}

fn default_input() -> (Vec<i64>, Vec<Vec<Listing>>) {
    parse_input(include_input!(2023 / 05))
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Vec<Listing>>) {
    let stanzas: Vec<Vec<i64>> = input
        .split("\n\n")
        .map(get_numbers)
        .collect();

    let seeds = stanzas[0].to_owned();
    let conversions = stanzas[1..].iter()
        .map(|map_numbers| {
            map_numbers
                .iter()
                .tuples()
                .map(|(destination_start, source_start, length)| {
                    Listing {
                        source_start: *source_start,
                        offset: destination_start - source_start,
                        len: *length,
                    }
                })
                .sorted_by_key(|listing| listing.source_start)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (seeds, conversions)
}

fn solve(conversions: &[Vec<Listing>], seed_ranges: Vec<(i64, i64)>) -> i64 {
    seed_ranges.iter()
        .map(|seed_range| {
            let sub_ranges: Vec<(i64, i64)> = conversions.iter()
                .fold(vec![seed_range.to_owned()], |ranges, listings| {
                    let test: Vec<(i64, i64)> = ranges.iter().flat_map(|range| {
                        let mut sub_ranges: Vec<(i64, i64)> = Vec::new();
                        let (range_first, range_last) = range;
                        let last = listings.iter().fold(*range_first, |next, listing| {
                            if range_last >= &listing.source_start && &next <= &listing.source_end() {
                                if next < listing.source_start {
                                    sub_ranges.push((next.to_owned(), listing.source_start - 1));
                                }
                                let se = listing.source_end();
                                let map_end = min(range_last, &se);
                                sub_ranges.push((next + listing.offset, map_end + listing.offset));
                                *map_end + 1
                            } else {
                                next
                            }
                        });
                        if &last <= range_last {
                            sub_ranges.push((last.to_owned(), range_last.to_owned()));
                        }
                        sub_ranges
                    }).collect();
                    test
                });

            sub_ranges.iter().map(|(first, _)| first.to_owned()).min().unwrap()
        })
        .min()
        .unwrap()
}

fn part1((seeds, conversions): (Vec<i64>, Vec<Vec<Listing>>)) -> i64 {
    let seed_ranges: Vec<(i64, i64)> = seeds.iter().map(|seed| (*seed, *seed)).collect();
    solve(&conversions, seed_ranges)
}

fn part2((seeds, conversions): (Vec<i64>, Vec<Vec<Listing>>)) -> i64 {
    let seed_ranges: Vec<(i64, i64)> = seeds.chunks(2)
        .filter_map(|chunk| {
            if let [start, length] = chunk {
                Some((*start, start + length - 1))
            } else {
                None
            }
        })
        .collect();
    solve(&conversions, seed_ranges)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "seeds: 79 14 55 13

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
56 93 4";
    assert_eq!(part1(parse_input(input)), 35);
    assert_eq!(part2(parse_input(input)), 46);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 379811651);
    assert_eq!(part2(input), 27992443);
}
