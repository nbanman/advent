use advent::prelude::*;

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

fn default_input() -> Vec<usize> {
    parse_input(include_input!(2018 / 08))
}

fn parse(mut data: &[usize]) -> (&[usize], usize, usize) {
    let children = data[0];
    let meta_entries = data[1];
    data = &data[2..];

    let mut total = 0;
    let mut values = Vec::new();

    // build children recursively
    for _ in 0..children {
        let (child_data, child_total, child_value) = parse(data);
        data = child_data; // this advances the data beyond the amount used by the inner parse
        total += child_total;
        values.push(child_value);
    }

    let metadata = &data[..meta_entries];
    data = &data[meta_entries..];

    let sum = metadata.iter().sum();
    total += sum;

    if children == 0 {
        (data, total, sum)
    } else {
        let value = metadata.iter()
            .filter_map(|i| values.get(i - 1)).sum();
        (data, total, value)
    }
}

fn part1(data: Vec<usize>) -> usize {
    let (_, total, _) = parse(&data);
    total
}

fn part2(data: Vec<usize>) -> usize {
    let (_, _, value) = parse(&data);
    value
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2");
    assert_eq!(part1(input.clone()), 138);
    assert_eq!(part2(input), 66);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 36027);
    assert_eq!(part2(input), 23960);
}
