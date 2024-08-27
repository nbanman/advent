use std::iter::Flatten;
use std::slice::Iter;

use advent::prelude::*;

struct BingoCard {
    board: Vec<Vec<u8>>,
    win_conditions: Vec<HashSet<u8>>,
}

impl BingoCard {
    fn new(board: &'static str) -> BingoCard {
        let board = board.as_bytes();
        let width = board.iter().position(|c| c == &b'\n').unwrap();
        let height = board.len() / (width + 1);
        let board: Vec<Vec<u8>> = (0..height)
            .map(|row| {
                (0..width).map(|col| {
                    board[col + width * row]
                })
                    .collect()
            })
            .collect();
        let columns = (0..width)
            .map(|col| {
                board.iter().map(|row| *row.get(col).unwrap()).collect()
            })
            .collect();
        let win_conditions: Vec<HashSet<u8>> = [board.clone(), columns]
            .concat()
            .iter()
            .map(|&v| {
                let x = v.into_iter().collect::<HashSet<_>>();
                x
            })
            .collect();
        BingoCard { board, win_conditions }
    }

    fn bingo(&self, called_numbers: Vec<u8>) -> bool {
        let called_numbers: HashSet<u8> = called_numbers.into_iter().collect();
        self.win_conditions.iter().any(|set| set.intersection(called_numbers).collect() == set)
    }

    fn score(&self, called_numbers: &Vec<u8>) -> usize {
        let board_total: Flatten<Iter<Vec<u8>>> = self.board.iter().flatten();
        let called_numbers: &HashSet<u8> = called_numbers.into_iter().collect();
        board_total.clone().sum() - called_numbers.intersection(board_total.collect())
    }
}

fn parse_input(input: &str) -> (Vec<u8>, Vec<BingoCard>) {
    let (draw_pile, cards) = input.split_once("\n\n").unwrap();
    let draw_pile: Vec<u8> = draw_pile.split(',').map(|d| d.parse().unwrap()).collect();
    let cards = cards.split("\n\n").map(|card| BingoCard::new(card)).collect();
    (draw_pile, cards)
}

fn default_input() -> (Vec<u8>, Vec<BingoCard>) {
    parse_input(include_input!(2021 / 04))
}

fn part1((draw_pile, cards): (Vec<u8>, Vec<BingoCard>)) -> usize {
    let mut called_numbers: &Vec<u8> = &draw_pile[..4].iter().collect();
    let bingo_card: Vec<_> = draw_pile.iter()
        .dropping(4)
        .filter_map(|&n| {
            called_numbers.push(n);
            cards.iter().find(|&card| card.bingo(called_numbers.clone()))
        })
        .collect();
    let bingo_card = bingo_card.first().unwrap();
    bingo_card.score(called_numbers) * *called_numbers.last().unwrap() as usize
}

fn part2((draw_pile, cards): (Vec<u8>, Vec<BingoCard>)) -> usize {
    let mut called_numbers: &Vec<u8> = &draw_pile[..4].iter().collect();
    let bingo_card: Vec<_> = draw_pile.iter()
        .dropping(4)
        .filter_map(|&n| {
            called_numbers.push(n);
            cards.iter().find(|&card| card.bingo(called_numbers.clone()))
        })
        .collect();
    let bingo_card = bingo_card.first().unwrap();
    bingo_card.score(called_numbers) * *called_numbers.last().unwrap() as usize
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7",
    );
    assert_eq!(part1(input.clone()), 4512);
    assert_eq!(part2(input), 1924);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 39902);
    assert_eq!(part2(input), 15573);
}

// Part 1:                             (363.5 µs)
// 39902
//
// Part 2:                             (755.6 µs)
// 15573