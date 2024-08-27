use std::collections::HashMap;

use advent::prelude::*;

fn parse_input(input: &'static str) -> (&str, HashMap<String, (String, String)>) {
    let (directions, net_str) = input.split_once("\n\n").unwrap();
    let mut network: HashMap<String, (String, String)> = HashMap::new();
    for line in net_str.lines() {
        let (node, to) = line.split_once(" = (").unwrap();
        let (left, to) = to.split_once(", ").unwrap();
        let (right, _) = to.split_once(')').unwrap();
        network.insert(node.to_string(), (left.to_string(), right.to_string()));
    }
    (directions, network)
}

fn default_input() -> (&'static str, HashMap<String, (String, String)>) {
    parse_input(include_input!(2023 / 08))
}

fn traverse<F>(
    directions: &str,
    network: &HashMap<String, (String, String)>,
    start_node: &str,
    end_condition: F) -> usize
    where F: Fn(&str) -> bool
{
    directions
        .chars()
        .cycle()
        .scan(start_node, |node, dir| {
            let new_node = network
                .get(&node.to_string())
                .map(|(left, right)| if dir == 'L' {
                    left.as_str()
                } else {
                    right.as_str()
                })
                .unwrap();
            *node = new_node;
            Some(new_node)
        })
        .enumerate()
        .find(|(_, node)| {
            end_condition(&node)
        })
        .unwrap()
        .0 + 1
}

// fn part1(dir: &str, network: &HashMap<String, (String, String)>) -> usize {
fn part1(
    (directions, network): (&str, HashMap<String, (String, String)>)
) -> usize {
    let end_condition = |end: &str| end == "ZZZ";
    traverse(directions, &network, "AAA", end_condition)
}

fn part2(
    (directions, network): (&str, HashMap<String, (String, String)>)
) -> usize {
    let end_condition = |end: &str| end.ends_with('Z');
    network.keys()
        .filter(|node| node.ends_with('A'))
        .map(|node| traverse(directions, &network, node, end_condition))
        .reduce(|acc, cycle_length| lcm(acc as i64, cycle_length as i64) as usize)
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part1(parse_input(input)), 2);
}

#[test]
fn example2() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part1(parse_input(input)), 6);
}

#[test]
fn example3() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
    assert_eq!(part2(parse_input(input)), 6);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 19241);
    assert_eq!(part2(input), 9606140307013);
}
