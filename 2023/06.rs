use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 06)
}

fn solve<F>(input: &str, parse_line: F) -> i64
where 
    F: Fn(&str) -> Vec<i64>
{
    let mut lines = input.lines();

    let races: Vec<Vec<i64>> = input
        .lines()
        .map(parse_line)
        .collect();

    let times = races.get(0).unwrap();
    let distances = races.get(1).unwrap();
    let races = times.iter().zip(distances.iter());
    races.map(|(time, distance)| ways_to_win(*time, *distance))
        .product()
}

fn ways_to_win(time: i64, distance: i64) -> i64 {
    let b = time as f64;
    let c = -distance as f64;
    let (root_1, root_2) = quadratic(-1.0, b, c);
    ((root_2 - 1.0).ceil() - (root_1 + 1.0).floor()) as i64 + 1
}

fn quadratic(a: f64, b: f64, c: f64) -> (f64, f64) {
    let determinant = (b * b - 4.0 * a * c).sqrt();
    let root_1 = (-b + determinant) / (2.0 * a);
    let root_2 = (-b - determinant) / (2.0 * a);
    (root_1, root_2)
}

fn part1(input: &str) -> i64 {
    let parse_line = |line: &str| -> Vec<i64> {
        line.split_whitespace().filter_map(|substr| substr.parse().ok()).collect()
    };
    solve(input, parse_line)
}

fn part2(input: &str) -> i64 {
    let parse_line = |line: &str| -> Vec<i64> {
        let digits: String = line.chars().filter(|c| c.is_ascii_digit()).collect();
        let number = digits.parse::<i64>().unwrap();
        vec![number]
    };
    solve(input, parse_line)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 2374848);
    assert_eq!(part2(input), 39132886);
}
