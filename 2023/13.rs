use advent::prelude::*;

fn parse_input(input: &str) -> Vec<String> {
    input.split("\n\n").map(|str| String::from(str)).collect()
}

fn default_input() -> Vec<String> {
    parse_input(include_input!(2023 / 13))
}

fn find_seams(patterns: Vec<String>) -> Vec<(String, usize)> {
    patterns.iter()
        .filter_map(|pattern| {
            let width = pattern.find('\n')? + 1;
            let height = (pattern.len() + 1) / width;
            let value = if let Some(x) = find_seam(pattern, width, 1, 1) {
                x
            } else if let Some(x) = find_seam(pattern, height, width, 100) {
                x
            } else {
                panic!("No seam found in pattern '{}'.", pattern)
            };
            Some((String::from(pattern), value))
        }).collect()
}

fn find_seam(pattern: &str, length: usize, advance: usize, factor: usize) -> Option<usize> {
    let mut stack = Vec::new();
    let chars = pattern.as_bytes();
    stack.push(chars[0]);
    let mut potential_seam: Option<usize> = None;
    let mut location = 1usize;
    'outer: while location < length { //todo <=?
        let next = chars[location + advance];
        if next != *stack.last().unwrap() {
            stack.push(next);
            location += 1;
        } else {
            potential_seam = Some(location);
            'inner: loop {
                stack.pop();
                if stack.is_empty() { break 'outer; }
                location += 1;
                let inner_next = if let Some(x) = chars.get(location) { x } else { break 'outer; };
                if stack.last().unwrap() != inner_next {
                    if let Some(potential_seam) = potential_seam {
                        location = potential_seam - (location - potential_seam);
                        while location != potential_seam + 1 {
                            stack.push(chars[location + advance]);
                            location += 1;
                        }
                    }
                    potential_seam = None;
                    break 'inner;
                }
            }
        }
    }
    potential_seam.map(|seam| seam * factor)
}

fn part1(patterns: Vec<String>) -> usize {
    find_seams(patterns).iter().map(|(_, value)| value).sum()
}

fn part2(patterns: Vec<String>) -> usize {
    todo!("part 2")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";
    assert_eq!(part1(parse_input(input)), 405);
    // assert_eq!(part2(parse_input(input)), 400);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 27505);
    assert_eq!(part2(input), 22906);
}
