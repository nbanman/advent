use advent::prelude::*;

fn default_input() -> &'static str { include_input!(2017 / 09) }

fn solve(input: &str) -> (usize, usize) {
    let mut in_garbage = false;
    let mut garbage = 0usize;
    let mut depth = 0usize;
    let mut score = 0usize;
    let mut last = ' ';

    for c in input.chars() {
        if in_garbage {
            if c == '>' && last != '!' { in_garbage = false }
            if !("!>".contains(c) || last == '!') { garbage += 1 }
            last = if last == '!' { ' ' } else { c };
        } else {
            match c {
                '<' => in_garbage = true,
                '{' => {
                    depth += 1;
                    score += depth;
                },
                '}' => depth -= 1,
                _ => {},
            }
        }
    }
    (score, garbage)
}
fn part1(input: &str) -> usize {
    solve(input).0
}

fn part2(input: &str) -> usize {
    solve(input).1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 9251);
    assert_eq!(part2(input), 4322);
}
