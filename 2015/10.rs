use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 10).trim_end()
}

fn look_and_say(s: String) -> String {
    let mut new = String::new();
    let s = s.as_bytes();
    let mut digit = s[0];
    let mut count = 1;
    for i in 1..s.len() {
        if s[i] == digit {
            count += 1;
        } else {
            new.push_str(&count.to_string());
            new.push_str(&(digit - 48).to_string());
            digit = s[i];
            count = 1;
        }
    }
    new.push_str(&count.to_string());
    new.push_str(&(digit - 48).to_string());
    new
}

fn solve(input: String, n: usize) -> usize {
    let mut s = input;
    for _ in 0..n {
        s = look_and_say(s);
    }
    s.len()
}

fn part1(input: &str) -> usize {
    solve(input.to_string(), 40)
}

fn part2(input: &str) -> usize {
    solve(input.to_string(), 50)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 492982);
    assert_eq!(part2(input), 6989950);
}
