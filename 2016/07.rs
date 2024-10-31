use advent::prelude::*;


fn parse_input(input: &str) -> Vec<[Vec<&str>; 2]> {
    input
        .lines()
        .map(|line| {
            let mut buckets = [Vec::new(), Vec::new()];
            for (idx, token) in line.split(&['[', ']']).enumerate() {
                buckets[idx & 1].push(token);
            }
            buckets
        })
        .collect()
}
fn default_input() -> Vec<[Vec<&'static str>; 2]> {
    parse_input(include_input!(2016 / 07))
}

fn abba(s: &str) -> bool {

    s.chars()
        .tuple_windows()
        .any(|(a, b, c, d)| a == d && b == c && a != b)
}

fn aba(s: &str) -> Vec<String> {
    s.chars()
        .tuple_windows()
        .filter(|(a, b, c)| a == c && a != b)
        .map(|(a, b, _)| format!("{}{}{}", b, a, b))
        .collect()
}

fn part1(ips: Vec<[Vec<&str>; 2]>) -> usize {
    ips.into_iter()
        .filter(|[supernets, hypernets]| {
            supernets.into_iter().any(|it| abba(it)) &&
                !hypernets.into_iter().any(|it| abba(it))
        })
        .count()
}

fn part2(ips: Vec<[Vec<&str>; 2]>) -> usize {
    ips.into_iter()
        .filter(|[supernets, hypernets]| {
            supernets.into_iter()
                .flat_map(|supernet| aba(supernet))
                .any(|aba| hypernets.iter().any(|it| it.contains(aba.as_str())))
        })
        .count()
}


fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 118);
    assert_eq!(part2(input), 260);
}
