use std::ops::RangeInclusive;
use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(Vector2, Vector2)> {
    input
        .get_numbers()
        .array_chunked()
        .map(|[px, py, vx, vy]| (vector![px, py], vector![vx, vy]))
        .collect()
}

fn default_input() -> Vec<(Vector2, Vector2)> {
    parse_input(include_input!(2018 / 10))
}

fn to_string(points: &[(Vector2, Vector2)]) -> String {
    let min_x = points.iter().map(|(p, _)| p.x).min().unwrap();
    let min_y = points.iter().map(|(p, _)| p.y).min().unwrap();
    let max_x = points.iter().map(|(p, _)| p.x).max().unwrap();
    let max_y = points.iter().map(|(p, _)| p.y).max().unwrap();
    let set: HashSet<_> = points.iter().map(|(p, _)| *p).collect();
    let mut s = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if set.contains(&vector!(x, y)) {
                s.push('█');
            } else {
                s.push('░');
            }
        }
        s.push('\n');
    }
    s
}

fn min_max_ranges<'a>(vectors: impl Iterator<Item = &'a Vector2>) -> (RangeInclusive<i64>, RangeInclusive<i64>) {
    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    for vector in vectors {
        if vector.x < min_x { min_x = vector.x };
        if vector.x > max_x { max_x = vector.x };
        if vector.y < min_y { min_y = vector.y };
        if vector.y > max_y { max_y = vector.y };
    }

    (min_x..=max_x, min_y..=max_y)
}

fn solve(points: Vec<(Vector2, Vector2)>) -> (usize, Vec<(Vector2, Vector2)>) {
    iter::successors(Some(points), |prev| Some(move_points(prev)))
        .enumerate()
        .find(|(_, points)| {
            let positions = points
                .iter()
                .map(|(pos, _)| pos);
            let (_, y_range) = min_max_ranges(positions);
            y_range.end() - y_range.start() == 9
        })
        .unwrap()
}

fn move_points(points: &Vec<(Vector2, Vector2)>) -> Vec<(Vector2, Vector2)> {
    points.iter().map(|&(pos, vel)| (pos + vel, vel)).collect()
}

fn part1(points: Vec<(Vector2, Vector2)>) -> String {
    let (_, points) = solve(points);
    to_string(&points)
}

fn part2(points: Vec<(Vector2, Vector2)>) -> usize {
    let (secs, _) = solve(points);
    secs
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>",
    );
    assert_eq!(
        part1(input.clone()),
        "\
█░░░█░░███
█░░░█░░░█░
█░░░█░░░█░
█████░░░█░
█░░░█░░░█░
█░░░█░░░█░
█░░░█░░░█░
█░░░█░░███
"
    );
    assert_eq!(part2(input), 3);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(
        part1(input.clone()),
        "\
█░░░░█░░█░░░░░░░██████░░░░██░░░░█░░░░█░░█████░░░░████░░░██████
█░░░░█░░█░░░░░░░░░░░░█░░░█░░█░░░█░░░█░░░█░░░░█░░█░░░░█░░░░░░░█
░█░░█░░░█░░░░░░░░░░░░█░░█░░░░█░░█░░█░░░░█░░░░█░░█░░░░░░░░░░░░█
░█░░█░░░█░░░░░░░░░░░█░░░█░░░░█░░█░█░░░░░█░░░░█░░█░░░░░░░░░░░█░
░░██░░░░█░░░░░░░░░░█░░░░█░░░░█░░██░░░░░░█████░░░█░░░░░░░░░░█░░
░░██░░░░█░░░░░░░░░█░░░░░██████░░██░░░░░░█░░░░█░░█░░███░░░░█░░░
░█░░█░░░█░░░░░░░░█░░░░░░█░░░░█░░█░█░░░░░█░░░░█░░█░░░░█░░░█░░░░
░█░░█░░░█░░░░░░░█░░░░░░░█░░░░█░░█░░█░░░░█░░░░█░░█░░░░█░░█░░░░░
█░░░░█░░█░░░░░░░█░░░░░░░█░░░░█░░█░░░█░░░█░░░░█░░█░░░██░░█░░░░░
█░░░░█░░██████░░██████░░█░░░░█░░█░░░░█░░█████░░░░███░█░░██████
"
    );
    assert_eq!(part2(input), 10656);
}
