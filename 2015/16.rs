use once_cell::sync::Lazy;
use regex::Regex;
use advent::prelude::*;

static RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([a-z]+): (\d+)").unwrap()
});
fn build_sue(ticker: &'static str) -> HashMap<&'static str, usize> {
    RX.captures_iter(ticker)
        .map(|captures| {
            let mut cap_iter = captures.iter();
            cap_iter.next();
            let item = cap_iter.next().unwrap().unwrap().as_str();
            let amt = cap_iter.next().unwrap().unwrap().as_str().parse().unwrap();
            (item, amt)
        })
        .collect()
}
fn parse_input(input: &'static str) -> (HashMap<&'static str, usize>, Vec<HashMap<&'static str, usize>>) {
    let ticker_tape = r"children: 3
        cats: 7
        samoyeds: 2
        pomeranians: 3
        akitas: 0
        vizslas: 0
        goldfish: 5\n
        trees: 3\n
        cars: 2\n
        perfumes: 1";

    let aunt_sue = build_sue(ticker_tape);

    let other_sues = input.lines()
        .map(|ticker| build_sue(ticker))
        .collect();

    (aunt_sue, other_sues)
}

fn default_input() -> (HashMap<&'static str, usize>, Vec<HashMap<&'static str, usize>>) {
    parse_input(include_input!(2015 / 16))
}

fn part1((aunt_sue, sues): (HashMap<&'static str, usize>, Vec<HashMap<&'static str, usize>>)) -> usize {
    sues.into_iter().enumerate()
        .find(|(_, sue)| {
            aunt_sue.iter().all(|(item, amt)| amt == sue.get(item).unwrap_or(&amt))
        })
        .unwrap()
        .0 + 1
}

fn part2((aunt_sue, sues): (HashMap<&'static str, usize>, Vec<HashMap<&'static str, usize>>)) -> usize {
    sues.into_iter().enumerate()
        .find(|(_, sue)| {
            aunt_sue.iter().all(|(item, amt)| {
                if let Some(sue_amt) = sue.get(item) {
                    match *item {
                        "cats" | "trees" => sue_amt > amt,
                        "pomeranians" | "goldfish" => sue_amt < amt,
                        _ => sue_amt == amt,
                    }
                } else { true }
            })
        })
        .unwrap()
        .0 + 1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 40);
    assert_eq!(part2(input), 241);
}
