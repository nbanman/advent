use once_cell::sync::Lazy;
use regex::Regex;
use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 09).trim()
}

static MARKER_RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"\(\d+x\d+\)").unwrap()
});
fn decompressed_length(input: &str, recursive: bool) -> usize {
    let mut decomp_length = 0;
    let mut index = 0;
    while index < input.len() {
        let marker = MARKER_RX.find(&input[index..]);
        if marker.is_none() {
            return decomp_length + input[index..].len();
        }
        let marker = marker.unwrap();
        decomp_length += input[index..index + marker.start()].len();
        let (sequence_length, repeats) = marker.as_str()
            .get_numbers::<usize>()
            .collect_tuple()
            .unwrap();
        let sequence = &input[index + marker.end()..index + marker.end() + sequence_length];
        let sequence = if recursive {
            decompressed_length(sequence, recursive)
        } else {
            sequence.len()
        } * repeats;
        decomp_length += sequence;
        index += marker.end() + sequence_length;
    }
    decomp_length
}

fn part1(input: &str) -> usize {
    decompressed_length(input, false)
}

fn part2(input: &str) -> usize {
    decompressed_length(input, true)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 110346);
    assert_eq!(part2(input), 10774309173);
}
