use advent::prelude::*;

fn parse_input(input: &str) -> Vec<[i32; 2]> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| match c {
                    "A" | "X" => 0,
                    "B" | "Y" => 1,
                    "C" | "Z" => 2,
                    _ => unreachable!(),
                }).next_array()
                .unwrap()
        }).collect()
}

fn default_input() -> Vec<[i32; 2]> {
    parse_input(include_input!(2022 / 02))
}

fn throw_score(my_throw: &i32) -> i32 {
    my_throw + 1
}

fn outcome_score(my_outcome: &i32) -> i32 {
    my_outcome * 3
}

fn my_outcome(my_throw: &i32, opponent_throw: &i32) -> i32 {
    (my_throw - opponent_throw + 1).rem_euclid(3)
}

fn my_throw(my_outcome: &i32, opponent_throw: &i32) -> i32 {
    (my_outcome + opponent_throw - 1).rem_euclid(3)
}

fn part1(rounds: Vec<[i32; 2]>) -> i32 {
    rounds.iter().map(|[opponent_throw, my_throw]| {
        let my_outcome = my_outcome(my_throw, opponent_throw);
        outcome_score(&my_outcome) + throw_score(my_throw)
    }).sum()
}

fn part2(rounds: Vec<[i32; 2]>) -> i32 {
    rounds.iter().map(|[opponent_throw, my_outcome]| {
        let my_throw = my_throw(my_outcome, opponent_throw);
        outcome_score(my_outcome) + throw_score(&my_throw)
    }).sum()
}


fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "A Y
B X
C Z",
    );
    assert_eq!(part1(input.clone()), 15);
    assert_eq!(part2(input), 12);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 9241);
    assert_eq!(part2(input), 14610);
}
