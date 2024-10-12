use advent::prelude::*;

#[derive(Clone)]
struct Hand {
    cards: Vec<u8>,
    bid: usize,
}
fn parse_input(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();
            let cards = cards.as_bytes().iter()
                .map(|&c| {
                    match c {
                        b'T' => 11,
                        b'J' => 12,
                        b'Q' => 13,
                        b'K' => 14,
                        b'A' => 15,
                        num => num - 48,
                    }
                })
                .collect();
            Hand {
                cards,
                bid: bid.parse::<usize>().unwrap(),
            }
        })
        .collect()
}

fn default_input() -> Vec<Hand> {
    parse_input(include_input!(2023 / 07))
}

impl Hand {
    fn hand_strength(&self, jacks_are_jokers: bool) -> usize {
        let jokers = self.cards.iter()
            .filter(|&&c| jacks_are_jokers && c == 12)
            .count();
        let hand_type_strength = if jokers == 5 {
            10
        } else {
            let groups: Vec<usize> = self.cards.iter()
                .filter(|&&c| !jacks_are_jokers || c != 12)
                .counts()
                .into_values()
                .sorted_by(|a, b| b.cmp(a))
                .collect();
            (*groups.first().unwrap() + jokers) * 2 + *(groups.get(1).unwrap_or(&0))
        };
        let value = |c: u8| -> usize {
            if jacks_are_jokers && c == 12 { 0 } else { c as usize }
        };
        self.cards.iter().fold(hand_type_strength, |acc, &card| {
            (acc << 4) + value(card)
        })
    }
}

fn solve(input: Vec<Hand>, jacks_are_jokers: bool) -> usize {
    input
        .iter()
        .sorted_by_cached_key(|hand| hand.hand_strength(jacks_are_jokers))
            .enumerate()
            .map(|(index, hand)| (index + 1) * hand.bid)
            .sum()
}


fn part1(input: Vec<Hand>) -> usize {
    solve(input, false)
}

fn part2(input: Vec<Hand>) -> usize {
    solve(input, true)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part1(parse_input(input)), 6440);
    assert_eq!(part2(parse_input(input)), 5905);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 253866470);
    assert_eq!(part2(input), 254494947);
}
