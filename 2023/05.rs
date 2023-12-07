use advent::prelude::*;

#[derive(Clone, Copy, Debug)]
struct Listing {
    source_start: u64,
    offset: u64,
    len: u64,
}

impl Listing {
    fn source_end(&self) -> u64 {
        self.source_start + self.len - 1
    }
}

fn default_input() -> (Vec<u64>, Vec<Vec<Listing>>) {
    parse_input(include_input!(2023 / 05))
}

fn parse_input(input: &str) -> (Vec<u64>, Vec<Vec<Listing>>) {
    let stanzas: Vec<Vec<u64>> = input.split("\n\n")
        .map(|stanza| {
            let number_strings: String = stanza.chars()
                .filter(|c| c.is_ascii_digit() || c.is_whitespace())
                .collect();
            number_strings.split_whitespace()
                .map(|number_string| number_string.parse::<u64>().unwrap())
                .collect()
        }).collect();

    let seeds = &stanzas[0];
    let conversions = stanzas[1..].iter()
        .map(|map_numbers| {
            map_numbers.chunks(3)
                .map(|chunk| {
                    Listing {
                        source_start: chunk[0],
                        offset: chunk[0] - chunk[1],
                        len: chunk[2]
                    }
                })
                .sorted_by_key(|listing| listing.source_start)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (seeds.to_owned(), conversions)
}

fn solve(conversions: &[Vec<Listing>], seed_ranges: Vec<(u64, u64)>) -> u64 {
    seed_ranges.iter().map(|seed_range| {
        println!("{:?}", seed_range);
        let sub_ranges = conversions
            .iter()
            .fold(vec![*seed_range], |ranges, listings| {
                println!("ranges: {:?}", ranges);
                let internal = ranges.iter().flat_map(|(start, end)| {
                    let mut sub_sub_ranges: Vec<(u64, u64)> = Vec::new();
                    let last = listings.iter().fold(*start, |next, listing| {
                        if end >= &listing.source_start && next <= listing.source_end() {
                            if next < listing.source_start {
                                sub_sub_ranges.push((next, listing.source_start));
                            }
                            let map_end = min(*end, listing.source_end());
                            map_end + 1
                        } else {
                            next
                        }
                    });
                    if last <= *end {
                        sub_sub_ranges.push((last, *end));
                    }
                    println!("Sub-sub-ranges: {:?}", sub_sub_ranges);
                    sub_sub_ranges
                }).collect::<Vec<(u64, u64)>>();
                println!("Internal: {:?}", internal);
                internal
            });
        let test = sub_ranges.iter().map(|(first, _)| first).min().unwrap();
        *test
    })
    .min().unwrap().to_owned()
}

fn part1((seeds, conversions): (Vec<u64>, Vec<Vec<Listing>>)) -> u64 {
    let seed_ranges: Vec<(u64, u64)> = seeds.iter().map(|seed| (*seed, *seed)).collect();
    solve(&conversions, seed_ranges)
}

fn part2((seeds, conversions): (Vec<u64>, Vec<Vec<Listing>>)) -> u64 {
    todo!("part 2")
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
    // assert_eq!(part2(parse_input(input)), 46);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 379811651);
    assert_eq!(part2(input), 27992443);
}
