use advent::prelude::*;

fn parse_input(input: &str) -> (Vec<VecDeque<u8>>, Vec<Move>) {
    let (stack_str, move_str) = input.split_once("\n\n").unwrap();

    let mut stacks: Vec<VecDeque<u8>> = Vec::with_capacity(9);
    for _ in 0..9 {
        stacks.push(VecDeque::new());
    }

    stack_str.lines()
        .dropping_back(1)
        .for_each(|line| {
            line
                .chars()
                .collect::<Vec<char>>()
                .chunks(4)
                .enumerate()
                .for_each(|(idx, chunk)| {
                    let i = chunk[1];
                    if i != ' ' {
                        stacks[idx].push_back(i as u8)
                    }
                })
        });

    let moves = move_str
        .lines()
        .map(|line| {
            let parts = line
                .split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect::<Vec<_>>();
            Move {
                quantity: parts[0],
                from: parts[1] - 1,
                to: parts[2] - 1,
            }
        }).collect();

    (stacks, moves)
}

fn default_input() -> (Vec<VecDeque<u8>>, Vec<Move>) {
    parse_input(include_input!(2022 / 05))
}

#[derive(Debug, Clone)]
struct Move {
    quantity: usize,
    from: usize,
    to: usize,
}

fn top(stacks: &[VecDeque<u8>]) -> String {
    let bs: Vec<_> = stacks.iter().filter_map(|s| s.front().copied()).collect();
    String::from_utf8(bs).unwrap()
}

fn part1((mut stacks, instructions): (Vec<VecDeque<u8>>, Vec<Move>)) -> String {
    for Move { quantity, from, to } in instructions {
        for _ in 0..quantity {
            if let Some(crater) = stacks[from].pop_front() {
                stacks[to].push_front(crater)
            }
        }
    }
    top(&stacks)
}

fn part2((mut stacks, instructions): (Vec<VecDeque<u8>>, Vec<Move>)) -> String {
    let mut temp = VecDeque::new();
    for Move { quantity, from, to } in instructions {
        for _ in 0..quantity {
            if let Some(crater) = stacks[from].pop_front() {
                temp.push_front(crater)
            }
        }
        while !temp.is_empty() {
            if let Some(crater) = temp.pop_front() {
                stacks[to].push_front(crater)
            }
        }
    }
    top(&stacks)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
    );
    assert_eq!(part1(input.clone()), "CMZ");
    assert_eq!(part2(input), "MCD");
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), "ZSQVCCJLL");
    assert_eq!(part2(input), "QZFJRWHGS");
}
