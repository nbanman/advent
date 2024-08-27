use std::collections::HashMap;
use std::ops::RangeInclusive;

use advent::prelude::*;

#[derive(Debug, Clone)]
struct Rule {
    category: usize,
    amount: usize,
    comparison: String,
    destination: String,
}

fn parse_input(input: &str) -> (Vec<Vec<usize>>, HashMap<String, Vec<Rule>>) {
    let (work_stanza, part_stanza) = input.split_once("\n\n").unwrap();
    let mut workflows = HashMap::new();
    let rule_pattern = regex!(r"(?:([xmas])([<>])(\d+):)?(\w+)");
    work_stanza.lines().for_each(|line| {
        let name: String = line.chars().take_while(|&it| it.is_ascii_alphabetic()).collect();
        let mut iter = rule_pattern.captures_iter(line);
        let name = iter.next().unwrap().get(0).unwrap().as_str().to_string();
        let rules: Vec<Rule> = iter
            .map(|m| {
                let category = if let Some(category) = m.get(1) {
                    match category.as_str() {
                        "x" => 0,
                        "m" => 1,
                        "a" => 2,
                        "s" => 3,
                        _ => 0,
                    }
                } else {
                    0
                };
                let amount: usize = if let Some(amount) = m.get(3) {
                    amount.as_str().parse().unwrap_or(0)
                } else {
                    0
                };
                let comparison = if let Some(comparison) = m.get(2) {
                    comparison.as_str().to_string()
                } else {
                    String::new()
                };
                Rule {
                    category,
                    amount,
                    comparison,
                    destination: m[4].to_string(),
                }
            })
            .collect();
        workflows.insert(name, rules);
    });


    let parts_pattern = regex!(r"-?\d+");

    let parts = parts_pattern.find_iter(part_stanza)
        .map(|m| m.as_str().parse::<usize>().unwrap())
        .chunks(4)
        .into_iter()
        .map(|chunk| chunk.collect::<Vec<usize>>())
        .collect::<Vec<Vec<usize>>>();

    (parts, workflows)
}

fn default_input() -> (Vec<Vec<usize>>, HashMap<String, Vec<Rule>>) {
    parse_input(include_input!(2023 / 19))
}

fn sort<'a>(workflows: &'a HashMap<String, Vec<Rule>>, name: &str, part: &Vec<usize>) -> &'a str {
    let workflow = workflows.get(name).unwrap();
    for rule in workflow {
        let result = if rule.comparison.as_str() == ">" {
            if part[rule.category] > rule.amount { Some(&rule.destination) } else { None }
        } else if rule.comparison.as_str() == "<" {
            if part[rule.category] < rule.amount { Some(&rule.destination) } else { None }
        } else {
            Some(&rule.destination)
        };

        if let Some(result) = result {
            let result = result.as_str();
            return if result == "A" || result == "R" {
                result
            } else {
                sort(workflows, result, part)
            };
        }
    }
    ""
}

fn part1((parts, workflows): (Vec<Vec<usize>>, HashMap<String, Vec<Rule>>)) -> usize {
    parts.iter()
        .filter(|xmas| {
            let mut name = "in";
            while name.len() > 1 {
                name = sort(&workflows, name, *xmas);
            }
            name == "A"
        })
        .map(|xmas| xmas.iter().sum::<usize>())
        .sum()
}

fn part2((parts, workflows): (Vec<Vec<usize>>, HashMap<String, Vec<Rule>>)) -> usize {
    let mut accepted = Vec::new();
    let mut remaining = vec![("in", vec![1usize..=4000; 4])];
    while !remaining.is_empty() {
        let next = remaining.iter()
            .flat_map(|(name, part_ranges)| {
                route(workflows.get(name).unwrap())
            });
    }
    3
}

fn route(
    workflow: Vec<Rule>,
    part_ranges: Vec<RangeInclusive<usize>>,
) -> Vec<(String, Vec<RangeInclusive<usize>>)> {
    let mut remaining = Some(part_ranges);
    let mut route = Vec::new();
    for rule in workflow {
        if let Some(remaining) = remaining {
            let (pass, fail) = split(remaining);
        } else {
            return route;
        }
    }
}

fn split(rule: Rule, part_ranges: Vec<RangeInclusive<usize>>) -> Vec<Vec<RangeInclusive<usize>>> {
    let comparison = rule.comparison.as_str();
    match comparison {
        ">" => {
            let breakpoint = rule.amount;
            let pass = breakpoint + 1..=part_ranges[rule.category].last().unwrap();
            let fail = *part_ranges[rule.category].start()..=breakpoint;
            make_splits(rule.category, pass, fail)
        }
    }
}

fn make_splits(
    category: usize,
    pass: RangeInclusive<usize>,
    fail: RangeInclusive<usize>,
) -> Vec<Vec<RangeInclusive<usize>>> {
    vec![]
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";
    assert_eq!(part1(parse_input(input)), 19114);
    assert_eq!(part2(parse_input(input)), 167409079868000);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 449531);
    assert_eq!(part2(input), 122756210763577);
}
