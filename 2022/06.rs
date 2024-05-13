use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2022 / 06)
}

fn solve(input: &str, n: usize) -> usize {
    let mut index_map = [0usize; 26];
    let mut duplicate_index = 0usize;
    let mut index = 0usize;
    input.as_bytes().iter()
        .position(|c| {
            let c = *c as usize - 97;
            let last_seen = index_map[c];
            index_map[c] = index;
            duplicate_index = max(duplicate_index, last_seen);
            index += 1;
            index - 1 - duplicate_index >= n
        })
        .unwrap() + 1
}

fn part1(input: &str) -> usize {
    solve(input, 4)
}

fn part2(input: &str) -> usize {
    solve(input, 14)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
}

#[test]
fn example2() {
    assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1361);
    assert_eq!(part2(input), 3263);
}
