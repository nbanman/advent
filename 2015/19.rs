use advent::prelude::*;

#[derive(Clone, Eq, Ord, PartialEq, PartialOrd, Debug)]
struct Rule {
    size: usize,
    element: String,
    replacement: String,
}

impl Rule {
    fn new(input: &str) -> Self {
        let mut split = input.split(" => ");
        let element = split.next().unwrap().to_string();
        let replacement = split.next().unwrap().to_string();
        let size = replacement.as_bytes().iter()
            .filter(|it| it.is_ascii_uppercase())
            .count();
        Rule {
            element,
            replacement,
            size,
        }
    }
}
fn parse_input(input: &'static str) -> (&'static str, Vec<Rule>) {
    let mut split = input.split("\n\n");
    let rule_lines = split.next().unwrap();
    let rules = rule_lines.lines()
        .map(|line| Rule::new(line))
        .sorted_by(|a, b| b.cmp(a))
        .collect();
    let molecule = split.next().unwrap().trim_end();
    (molecule, rules)
}

fn default_input() -> (&'static str, Vec<Rule>) {
    parse_input(include_input!(2015 / 19))
}

fn part1((molecule, rules): (&'static str, Vec<Rule>)) -> usize {
    regex!("[A-Z][a-z]?")
        .find_iter(molecule)
        .flat_map(|m| {
            let element = m.as_str();
            rules.iter()
                .filter(move |rule| rule.element == element)
                .map(move |rule| {
                    let mut result = String::new();
                    result.push_str(&molecule[..m.start()]);
                    result.push_str(&rule.replacement);
                    result.push_str(&molecule[m.end()..]);
                    result
                })
        })
        .collect::<HashSet<String>>()
        .len()
}

fn part2((molecule, rules): (&'static str, Vec<Rule>)) -> usize {
    let starts: Vec<_> = rules.iter()
        .filter(|rule| rule.element == "e")
        .map(|rule| rule.replacement.clone())
        .collect();
    iter::successors(Some(molecule.to_string()), |molecule| {
        rules.iter()
            .find(|rule| molecule.contains(&rule.replacement))
            .map(|x| molecule.replacen(&x.replacement, &x.element, 1))
    })
        .enumerate()
        .find(|(_, molecule)| {
            starts.contains(molecule)
        })
        .unwrap()
        .0 + 1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 535);
    assert_eq!(part2(input), 212);
}
