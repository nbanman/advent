use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2021 / 10)
}

struct Maps {
    counterparts: HashMap<u8, u8>,
    syntax_error_score: HashMap<u8, usize>,
    point_value: HashMap<u8, u8>
}

impl Maps {
    fn new() -> Maps {
        let counterparts = [(b'(', b')'), (b'[', b']'), (b'{', b'}'), (b'<', b'>')]
            .into_iter()
            .collect();

        let syntax_error_score = [(b')', 3), (b']', 57), (b'}', 1197), (b'>', 25137)]
            .into_iter()
            .collect();

        let point_value = [(b')', 1), (b']', 2), (b'}', 3), (b'>', 4)]
            .into_iter()
            .collect();

        Maps {
            counterparts,
            syntax_error_score,
            point_value,
        }
    }

    fn score(&self, stack: &Vec<u8>) -> usize {
        stack.iter().rev().
            fold(0, |acc, c| acc * 5 + *self.point_value.get(&c).unwrap() as usize)
    }
}

fn parse_string<F, G>(s: &str, maps: &Maps, on_corrupt: F, on_finish: G) -> Option<usize>
where
    F: Fn(u8) -> Option<usize>,
    G: Fn(Vec<u8>) -> Option<usize>,
{
    let mut stack = Vec::new();
    for &candidate in s.as_bytes().iter() {
        if maps.counterparts.values().contains(&candidate) {
            if candidate != stack.pop().unwrap() { return on_corrupt(candidate) }
        } else {
            stack.push(*maps.counterparts.get(&candidate).unwrap())
        }
    }
    on_finish(stack)
}

fn part1(input: &str) -> usize {
    let maps = Maps::new();
    input
        .lines()
        .filter_map(|line| {
            parse_string(
                line,
                &maps,
                |it| maps.syntax_error_score.get(&it).map(|it| *it),
                |_| None,
            )
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let maps = Maps::new();
    let scores: Vec<_> = input
        .lines()
        .filter_map(|line| {
            parse_string(
                line,
                &maps,
                |_| None,
                |it| Some(maps.score(&it))
            )
        })
        .sorted()
        .collect();
    *scores.get(scores.len() / 2).unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = "\
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
    assert_eq!(part1(input), 26397);
    assert_eq!(part2(input), 288957);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 167379);
    assert_eq!(part2(input), 2776842859);
}
