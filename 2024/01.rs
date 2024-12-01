use advent::prelude::*;

fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    input.get_numbers::<usize>().enumerate().partition_map(|(idx, n)| {
        if idx & 1 == 0 {
            Either::Left(n)
        } else {
            Either::Right(n)
        }
    })
}

fn default_input() -> (Vec<usize>, Vec<usize>) {
    parse_input(include_input!(2024 / 01))
}

fn part1((mut a, mut b): (Vec<usize>, Vec<usize>)) -> usize {
    a.sort_unstable();
    b.sort_unstable();
    a.into_iter().zip(b.into_iter())
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn part2((a, b): (Vec<usize>, Vec<usize>)) -> usize {
    let mut freq = HashMap::new();
    for n in b {
        freq.entry(n)
            .and_modify(|count| { *count += 1 })
            .or_insert(1);
    }
    a.into_iter().map(|n| n * freq.get(&n).unwrap_or(&0)).sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1);
    assert_eq!(part2(input), 2);
}
