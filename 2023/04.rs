use advent::prelude::*;

fn parse_input(input: &str) -> Vec<usize> {
    input.lines()
        .filter_map(|line| {
            let numbers = &line[line.find(':')? + 1..]
                .split_whitespace()
                .collect::<Vec<_>>();
            Some(
                numbers.len() - numbers.iter().unique().count()
            )
        }).collect()
}

fn default_input() -> Vec<usize> {
    parse_input(include_input!(2023 / 04))
}

fn part1(cards: Vec<usize>) -> usize {
    cards.iter().map(|count| 1 << count >> 1).sum()
}

fn part2(cards: Vec<usize>) -> usize {
    let mut card_count = vec![1; cards.len()];
    cards.iter().enumerate().for_each(|(index, count)| {
        let range = index + 1..=index + count;
        let number_of_cards = card_count[index];
        range.for_each(|i| card_count[i] += number_of_cards)
    });
    card_count.iter().sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part1(parse_input(input)), 13);
    assert_eq!(part2(parse_input(input)), 30);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 23750);
    assert_eq!(part2(input), 13261850);
}
