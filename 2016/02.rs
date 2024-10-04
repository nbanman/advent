use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 02)
}

fn solve<I, C>(instructions: &str, start: Vector2, in_bounds: I, conversion: C,) -> String
where
    I: Fn(Vector2) -> bool,
    C: Fn(Vector2) -> String,
{
    let pad_traverse = |pos: Vector2, c: char| {
        let new_pos = pos + match c {
            'L' => vector!(-1, 0),
            'R' => vector!(1, 0),
            'U' => vector!(0, -1),
            'D' => vector!(0, 1),
            _ => Vector2::zero(),
        };
        if in_bounds(new_pos) { new_pos } else { pos }
    };

    let process_instruction = |pos: Vector2, instruction: &str| {
        instruction.chars().fold(pos, pad_traverse)
    };

    instructions
        .lines()
        .scan(start, |state, line| {
            *state = process_instruction(*state, line);
            Some(*state)
        })
        .map(conversion)
        .collect::<Vec<_>>()
        .join("")
}
fn part1(input: &str) -> String {
    let start = vector!(1, 1);
    let in_bounds = |pos: Vector2| {
        chebyshev_distance(pos, start) < 2
    };
    let to_numpad = |pos: Vector2| {
        (pos.y * 3 + pos.x + 1).to_string()
    };
    solve(
        input,
        start,
        in_bounds,
        to_numpad,
    )
}

fn chebyshev_distance(a: Vector2, b: Vector2) -> usize {
    max((a.x - b.x).abs(), (a.y - b.y).abs()) as usize
}

fn manhattan_distance(a: Vector2, b: Vector2) -> usize {
    (a.x - b.x).abs() as usize + (a.y - b.y).abs() as usize
}
fn part2(input: &str) -> String {
    let start = vector!(0, 2);
    let in_bounds = |pos: Vector2| { manhattan_distance(pos, vector!(2, 2)) < 3 };
    let to_numpad = |pos: Vector2| {
        format!("{:X}", (5 + pos.x + (pos.y - 2) * 2 + 2 * signum(pos.y - 2)))
    };
    solve(
        input,
        start,
        in_bounds,
        to_numpad,
    )
}

fn signum(n: i64) -> i64 {
    match n {
        _ if n > 0 => 1,
        _ if n < 0 => -1,
        _ => 0,
    }
}
fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), "92435".to_string());
    assert_eq!(part2(input), "C1A88".to_string());
}
