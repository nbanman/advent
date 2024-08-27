use std::ops::RangeInclusive;

use advent::prelude::*;

type Ticket = Vec<i64>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Rule<'a> {
    name: &'a str,
    low: RangeInclusive<i64>,
    high: RangeInclusive<i64>,
}

impl Rule<'_> {
    fn valid_for(&self, value: &i64) -> bool {
        self.low.contains(value) || self.high.contains(value)
    }
}

fn parse_input(input: &str) -> (Vec<Rule<'_>>, Vec<Ticket>) {
    let (rules, tickets) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|line| {
            let (name, ranges) = line.split_once(':').unwrap();
            let [low, high] = &ranges
                .get_numbers()
                .array_chunked()
                .map(|[start, end]| start..=end)
                .collect::<Vec<_>>()[..];
            Rule { name, low: low.to_owned(), high: high.to_owned() }
        }).collect();

    let tickets: Vec<_> = tickets
        .lines()
        .filter_map(|line| {
            let numbers = get_numbers(line);
            if numbers.len() == 0 {
                None
            } else {
                Some(numbers)
            }
        })
        .collect();
    (rules, tickets)
}

fn default_input() -> (Vec<Rule<'static>>, Vec<Ticket>) {
    parse_input(include_input!(2020 / 16))
}


fn part1((rules, tickets): (Vec<Rule<'_>>, Vec<Ticket>)) -> i64 {
    tickets
        .into_iter()
        .flatten()
        .filter(|value| {
            rules.iter().all(|rule| !rule.valid_for(value))
        })
        .sum()
}

fn part2((rules, tickets): (Vec<Rule<'_>>, Vec<Ticket>)) -> i64 {
    let valid_tickets: Vec<_> = tickets
        .into_iter()
        .filter(|ticket| {
            ticket.into_iter().all(|value| {
                rules.iter().any(|rule| rule.valid_for(value))
            })
        })
        .collect();

    let width = valid_tickets
        .iter()
        .map(|ticket| ticket.len())
        .min()
        .unwrap();
    let height = valid_tickets.len();

    let value_list: Vec<Vec<_>> = (0..width)
        .map(|w| {
            (0..height)
                .map(|h| {
                    valid_tickets[h][w]
                })
                .collect()
        })
        .collect();

    let mut sorter: Vec<(i64, Vec<&Rule>)> = value_list
        .into_iter()
        .enumerate()
        .map(|index, values| {
            let rules: Vec<_> = rules
                .iter()
                .filter(|rule| values.all(|value| rule.valid_for(value)))
                .collect();
            (index, rules)
        })
        .collect();

    let mut register = HashMap::new();
    while !sorter.is_empty() {
        let temp_sorter = sorter
            .iter()
            .filter(|(_, rule)| rule.len() == 1);
        for (index, rule) in temp_sorter {
            register.insert(rule, index);
            sorter.remove((index, rule));
            for mut x in sorter.iter() {
                x.1.remove(x.1.first().unwrap())
            }
        }
    }

    // First we find all the tickets that match any rule correctly.
    let valid: Vec<_> = nearby
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|v| rules.iter().any(|rule| rule.valid_for(v)))
        })
        .collect();

    // Now construct a map of column to set of rules that might apply.
    let mut rule_sets: Vec<_> = (0..your.len())
        .map(|col| {
            // The column values for the valid tickets.
            let set: Vec<_> = valid.iter().map(|ticket| ticket[col]).collect();
            // The set of rules that match all the column values.
            let rules: HashSet<_> = rules
                .iter()
                .filter(|rule| set.iter().all(|v| rule.valid_for(v)))
                .collect();
            (col, rules)
        })
        .sorted_unstable_by_key(|(_, rules)| Reverse(rules.len()))
        .collect();

    let mut result = 1;
    while let Some((col, rules)) = rule_sets.pop() {
        assert_eq!(rules.len(), 1);
        let rule = rules.into_iter().next().unwrap();
        for (_, rule_set) in rule_sets.iter_mut() {
            rule_set.remove(&rule);
        }
        if rule.name.starts_with("departure") {
            result *= your[col]
        }
    }
    result
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12",
    );
    assert_eq!(part1(input.clone()), 71);
    assert_eq!(part2(input), 1);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 29019);
    assert_eq!(part2(input), 517827547723);
}
