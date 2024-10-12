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
                    num => num - 49,
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
    // strength is an Int that we use to sort with. The most important component of strength  is the strength
    // of the type of hand. After that, the value of the cards, in order. E.g., 98 > 8A, because 9 is greater
    // than 8. We can represent this all as an Int concatenating them all, giving 4 bits for each value.
    fn hand_strength(&self, jacks_are_jokers: bool) -> usize {
        // number of jokers gets added to the most numerous of the other cards to make the most powerful hand
        let jokers = if jacks_are_jokers {
            self.cards.iter()
                .filter(|&&c| c == 11)
                .count()
        } else {
            0
        };
        let hand_type_strength = if jokers == 5 {
            10
        } else {
            // groups cards together, for use in determining handTypeStrength. Sorted because the relative size
            // of the groups is used to determine what kind of hand we have. Then take the two most populous groups
            // in the hand, then deliver ordered ranking of each hand type.
            let mut groups = [0usize; 15];
            for &card in self.cards.iter() {
                if !jacks_are_jokers || card != 11 {
                    groups[card as usize] += 1;
                }
            }
            let (first, second) = groups
                .into_iter()
                .sorted_by(|a, b| b.cmp(a))
                .take(2)
                .collect_tuple()
                .unwrap();

            // this gives a strength, from weakest to strongest, of 3, 5, 6, 7, 8, 9, 10
            (first + jokers) * 2 + second
        };

        // utility function so that if jacks are jokers, the value of the joker is 0 instead of 11
        let value = |c: u8| -> usize {
            if jacks_are_jokers && c == 11 { 0 } else { c as usize }
        };

        // include the strength of cards in the final strength calculation
        self.cards.iter().fold(hand_type_strength, |acc, &card| {
            (acc << 4) + value(card)
        })
    }
}

// takes the hands, sorts by the hand strength as defined by each puzzle part, assigns points using rank and
// bid amount, then returns sum of all points
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
