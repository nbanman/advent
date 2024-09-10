use advent::prelude::*;
use fancy_regex::Regex;
use once_cell::sync::Lazy;

fn default_input() -> &'static str {
    include_input!(2015 / 11).trim_end()
}

fn increment(s: &mut String) {
    if let Some(last) = s.pop() {
        let increment = if last == 'z' {
            increment(s);
            'a'
        } else {
            (last as u8 + 1) as char
        };
        s.push(increment);
    }
}

fn has_straight(s: &str) -> bool {
    s.as_bytes()
        .windows(3)
        .any(|w| w[1].checked_sub(w[0]) == Some(1) && w[2].checked_sub(w[1]) == Some(1))
}

fn not_confusing(s: &str) -> bool {
    s.as_bytes().iter().all(|&c| c != b'o' && c != b'l' && c != b'i')
}

static TWO_PAIRS_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([a-z])\1").unwrap()
});

fn two_pairs(s: &str) -> bool {
    let v: Vec<_> = TWO_PAIRS_RX.find_iter(s).collect();
    v.len() >= 2
}

fn valid_password(s: &str) -> bool {
    has_straight(s) && not_confusing(s) && two_pairs(s)
}

fn next_password(mut password: String) -> String {
    loop {
        increment(&mut password);
        if valid_password(&password) { return password }
    }
}

fn part1(input: &str) -> String {
    let password = input.to_string();
    next_password(password)
}

fn part2(input: &str) -> String {
    let password = input.to_string();
    let password = next_password(password);
    next_password(password)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), String::from("hxbxxyzz"));
    assert_eq!(part2(input), String::from("hxcaabcc"));
}
