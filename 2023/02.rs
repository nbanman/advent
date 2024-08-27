use std::collections::HashMap;

use advent::prelude::*;

fn parse_input(input: &str) -> Vec<HashMap<String, u32>> {
    let pattern = regex!(r"(\d+) (\w+)");

    input
        .lines()
        .map(|line| {
            let mut game: HashMap<String, u32> = HashMap::new();
            pattern.captures_iter(line).for_each(|m| {
                let amt = m.get(1).map_or(0, |amt| {
                    amt.as_str().parse::<u32>().unwrap()
                });
                let color = m.get(2).map_or("unknown", |color| {
                    color.as_str()
                });
                match game.get(color) {
                    Some(n) => if n < &amt {
                        game.insert(color.to_string(), amt);
                    },
                    None => {
                        game.insert(color.to_string(), amt);
                    }
                }
                if let Some(n) = game.get(color) {
                    if n < &amt {
                        game.insert(color.to_string(), amt);
                    }
                }
            });
            game
        }).collect()
}

fn default_input() -> Vec<HashMap<String, u32>> {
    parse_input(include_input!(2023 / 02))
}

fn part1(input: Vec<HashMap<String, u32>>) -> usize {
    let mut standard_bag: HashMap<String, u32> = HashMap::new();
    standard_bag.insert(String::from("red"), 12);
    standard_bag.insert(String::from("green"), 13);
    standard_bag.insert(String::from("blue"), 14);

    input
        .iter()
        .enumerate()
        .filter(|(_, game)| {
            game.iter().all(|(color, amt)| {
                standard_bag.get(color).unwrap_or(&0) >= amt
            })
        }).map(|(index, _)| index + 1)
        .sum()
}

fn part2(input: Vec<HashMap<String, u32>>) -> u32 {
    input
        .iter()
        .map(|game| {
            game.values().product::<u32>()
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let test_input = "\
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let input = parse_input(test_input);
    assert_eq!(part1(input.clone()), 8);
    assert_eq!(part2(input), 2286);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2377);
    assert_eq!(part2(input), 71220);
}
