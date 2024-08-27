use std::hash::Hash;

use advent::prelude::*;

struct Display {
    wires: Vec<HashSet<u8>>,
    display: Vec<HashSet<u8>>,
    digit_map: HashMap<HashSet<u8>, u8>,
    output_value: usize,
}

impl Display {
    fn new(s: &str) -> Display {
        let sides: Vec<Vec<HashSet<u8>>> = s
            .split(" | ")
            .map(|s| {
                s.as_bytes().split(|&it| it == b' ')
                    .map(|&it| {
                        it.iter().collect::<HashSet<u8>>()
                    })
                    .collect()
            })
            .collect();
        let &wires = sides.get(0).unwrap();
        let &display = sides.get(1).unwrap();

        let mut digit_map = HashMap::new();
        let wire_groups: HashMap<u8, Vec<HashSet<u8>>> = wires.iter()
            .group_by(|&wire| wire.len())
            .collect();

        digit_map.insert(wire_groups.get(&2).unwrap()[0].clone(), 1);
        digit_map.insert(wire_groups.get(&4).unwrap()[0].clone(), 4);
        digit_map.insert(wire_groups.get(&7).unwrap()[0].clone(), 7);
        digit_map.insert(wire_groups.get(&7).unwrap()[0].clone(), 8);

        digit_map.insert(
            wire_groups.get(&6).unwrap().iter().find(|&&w| w.intersection(digit_map.get(&1)).count() == 1).unwrap().clone(),
            6,
        );
        digit_map.insert(
            wire_groups.get(&6).unwrap().iter().find(|&&w| w.intersection(digit_map.get(&4)).count() == 4).unwrap().clone(),
            9,
        );
        digit_map.insert(
            wire_groups.get(&5).unwrap().iter().find(|&&w| w.intersection(digit_map.get(&6)).count() == 5).unwrap().clone(),
            5,
        );
        digit_map.insert(
            wire_groups.get(&5).unwrap().iter().find(|&&w| w.intersection(digit_map.get(&5)).count() == 3).unwrap().clone(),
            2,
        );
        digit_map.insert(
            wire_groups.get(&5).unwrap().iter().find(|&&w| w.intersection(digit_map.get(&5)).count() == 4).unwrap().clone(),
            3,
        );
        digit_map.insert(
            wire_groups.get(&6).unwrap().iter().find(|&&w| w.intersection(digit_map.get(&5)).count() == 4).unwrap().clone(),
            0,
        );

        let digit_map: HashMap<_, _> = wire_groups.into_iter()
            .map(|(k, v)| (v, k))
            .collect();

        let output_value: Vec<_> = display.iter()
            .map(|wires| {
                *digit_map.get(wires).unwrap() as char
            })
            .collect();

        let output_value = output_value.parse::<usize>().unwrap();

        Display {
            wires,
            display,
            digit_map,
            output_value,
        }
    }
}

fn parse_input(input: &str) -> Vec<Display> {
    input
        .lines()
        .map(|line| Display::new(line))
        .collect()
}

fn default_input() -> Vec<Display> {
    parse_input(include_input!(2021 / 08))
}

fn part1(displays: Vec<Display>) -> usize {
    displays
        .iter()
        .flat_map(|display| display.display)
        .filter(|display| display.len() < 5 || display.len() > 6)
        .count()
}

fn part2(displays: Vec<Display>) -> usize {
    displays
        .iter()
        .flat_map(|display| display.display)
        .filter(|display| display.len() < 5 || display.len() > 6)
        .count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce",
    );
    assert_eq!(part1(input.clone()), 26);
    assert_eq!(part2(input), 61229);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 397);
    assert_eq!(part2(input), 1027422);
}

// Part 1:                             (10.90 µs)
// 397
//
// Part 2:                             (55.80 µs)
// 1027422