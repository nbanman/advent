use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    input
        .get_numbers()
        .chunks(3)
        .into_iter()
        .map(|chunk| chunk.sorted().collect())
        .collect()
}

fn default_input() -> Vec<Vec<u64>> {
    parse_input(include_input!(2015 / 02))
}

fn part1(input: Vec<Vec<u64>>) -> u64 {
    input
        .into_iter()
        .map(|boxy| {
            let [a, b, c] = boxy.as_slice() else { panic!("invalid parse") };
            let surface_area = 2 * a * b + 2 * a * c + 2 * b * c;
            let smallest_side_area = a * b;
            surface_area + smallest_side_area
        })
        .sum()
}

fn part2(input: Vec<Vec<u64>>) -> u64 {
    input
        .into_iter()
        .map(|boxy| {
            let [a, b, c] = boxy.as_slice() else { panic!("invalid parse") };
            let cubic_volume = a * b * c;
            let ribbon_to_wrap = 2 * (a + b);
            cubic_volume + ribbon_to_wrap
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1588178);
    assert_eq!(part2(input), 3783758);
}
