use advent::prelude::*;

#[derive(Clone)]
struct Hand {
    cards: Vec<u8>, // convert chars into u8s ordered by card strength
    bid: usize,
}

impl Hand {
    fn new(s: &str) -> Self {
        let (cards, bid) = s.split_once(' ').unwrap();
        let cards = cards.as_bytes().iter()
            .map(|&c| {
                match c {
                    b'T' => 10,
                    b'J' => 11,
                    b'Q' => 12,
                    b'K' => 13,
                    b'A' => 14,
                    num => num - 48,
                }
            })
            .collect();
        let bid = bid.parse().unwrap();
        Self { cards, bid }
    }
}

fn parse_input(input: &str) -> Vec<Hand> {
    input.lines().map(Hand::new).collect()
}

fn default_input() -> Vec<Hand> {
    parse_input(include_input!(2023 / 07))
}

impl Hand {
    // returns an array used for comparing hand strength. Each element in the array is compared, and
    // if tied, the next element is compared.
    fn hand_strength(&self, jacks_are_jokers: bool) -> [u8; 7] {
        // number of jokers gets added to the most numerous of the other cards to make the most powerful hand
        let jokers = if jacks_are_jokers {
            self.cards.iter()
                .filter(|&&c| c == 11)
                .count() as u8
        } else {
            0
        };

        // groups cards together, for use in determining the two most frequently occurring cards.
        let mut groups = [0u8; 15];
        for &card in self.cards.iter() {
            if !jacks_are_jokers || card != 11 {
                groups[card as usize] += 1;
            }
        }

        // get the frequency of the two most frequently occurring cards
        let (first, second) = groups.into_iter()
            .sorted_by(|a, b| b.cmp(a))
            .take(2)
            .collect_tuple()
            .unwrap();

        let mut strength = [0u8; 7];

        strength[0] = first + jokers; // # of most frequently occurring card, plus any jokers
        strength[1] = second; // # of second most frequently occurring card

        // remainder of array is filled with the strength of each individual card in the hand
        for (index, &card) in self.cards.iter().enumerate() {
            strength[index + 2] = if jacks_are_jokers && card == 11 { 0 } else { card }
        }
        strength
    }
}

// takes the hands, sorts by the hand strength as defined by each puzzle part, assigns points using rank and
// bid amount, then returns sum of all points
fn solve(hands: Vec<Hand>, jacks_are_jokers: bool) -> usize {
    hands.iter()
        .sorted_by_cached_key(|hand| hand.hand_strength(jacks_are_jokers))
        .enumerate()
        .map(|(index, hand)| (index + 1) * hand.bid)
        .sum()
}

fn part1(hands: Vec<Hand>) -> usize { solve(hands, false) }
fn part2(hands: Vec<Hand>) -> usize { solve(hands, true) }

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
