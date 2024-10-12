use advent::prelude::*;

fn parse_input(input: &str) -> &str {
    input
}

fn default_input() -> &'static str {
    parse_input(include_input!(2023 / 07))
}

struct Hand {
    cards: String,
    bid: usize,
}

impl Hand {
    fn hand_strength(&self) -> usize {
        let jokers = self.cards.chars()
            .filter(|c| *c == 'ĵ')
            .count();
        let hand_type_strength = if jokers == 5 {
            10
        } else {
            let groups: Vec<usize> = self.cards.chars()
                .filter(|c| *c != 'ĵ')
                .counts()
                .into_values()
                .sorted_by(|a, b| b.cmp(a))
                .collect();
            // println!("cards {}, groups {:?}", self.cards, groups);
            (*groups.first().unwrap() + jokers) * 2 + *(groups.get(1).unwrap_or(&0))
        };
        // println!("cards: {}, jokers: {}, hand_type: {}", self.cards, jokers, hand_type_strength);
        self.cards.chars().fold(hand_type_strength, |acc, card| {
            (acc << 4) + "ĵ23456789TJQKA".find(card).unwrap()
        })
    }
}

fn solve<F>(input: &str, swap_jokers: F) -> usize
    where F: Fn(&str) -> String
{
    input.lines().filter_map(|line| {
        let (cards, bid) = line.split_once(' ')?;
        let cards = swap_jokers(cards);
        let hand = Hand {
            cards,
            bid: bid.parse::<usize>().ok()?,
        };
        Some(hand)
    }).sorted_by_cached_key(|hand| hand.hand_strength())
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum()
}


fn part1(input: &str) -> usize {
    let swap_jokers = |cards: &str| -> String { cards.to_string() };
    solve(input, swap_jokers)
}

fn part2(input: &str) -> usize {
    let swap_jokers = |cards: &str| -> String {
        cards.replace('J', "ĵ")
    };
    solve(input, swap_jokers)
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
    assert_eq!(part1(input), 253866470);
    assert_eq!(part2(input), 254494947);
}
