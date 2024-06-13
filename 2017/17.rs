use advent::prelude::*;

fn parse_input(input: &str) -> usize {
    input.trim_end().parse().unwrap()
}

fn default_input() -> usize {
    parse_input(include_input!(2017 / 17))
}

fn part1(input_num: usize) -> usize {
    let mut buffer = vec![0];
    let mut curr_pos = 0;

    for n in 1..=2017 {
        curr_pos = (curr_pos + input_num) % buffer.len() + 1;
        buffer.insert(curr_pos, n);
    }

    buffer[(curr_pos + 1) % buffer.len()]
}

fn part2(input_num: usize) -> usize {
    let mut curr_pos = 0;
    let mut result = 0;
    let mut n = 0;
    let limit = 50_000_000;

    while n < limit {
        if curr_pos == 1 {
            result = n;
        }

        let fits = (n - curr_pos) / input_num;
        n += fits + 1;
        curr_pos = (curr_pos + (fits + 1) * (input_num + 1) - 1) % n + 1;
    }

    result
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1547);
    assert_eq!(part2(input), 31154878);
}
