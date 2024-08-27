use advent::prelude::*;

enum Dir {
    U,
    D,
    L,
    R,
}

fn parse_input(input: &str) -> Vec<&str> {
    input.trim_end().lines().collect()
}

fn default_input() -> Vec<&'static str> {
    parse_input(include_input!(2023 / 18))
}

fn solve(plans: Vec<(Dir, i64)>) -> i64 {
    let (mut y, mut area, mut perimeter) = (0i64, 0i64, 0i64);
    for (dir, dist) in plans {
        match dir {
            Dir::U => y -= dist,
            Dir::D => y += dist,
            Dir::L => area -= y * dist,
            Dir::R => area += y * dist,
        }
        perimeter += dist;
    }
    perimeter / 2 + 1 + area.unsigned_abs() as i64
}

fn part1(input: Vec<&str>) -> i64 {
    let plans: Vec<(Dir, i64)> = input.iter()
        .map(|s| {
            let (dir_str, rest) = s.split_once(' ').unwrap();
            let (dist_str, _) = rest.split_once(' ').unwrap();
            let dir = match dir_str {
                "U" => Dir::U,
                "D" => Dir::D,
                "L" => Dir::L,
                "R" => Dir::R,
                x => panic!("String must begin with [UDLR], but begins with {}.", x),
            };
            let dist: i64 = dist_str.parse().unwrap();
            (dir, dist)
        })
        .collect();
    solve(plans)
}

fn part2(input: Vec<&str>) -> i64 {
    let plans: Vec<(Dir, i64)> = input.iter()
        .filter_map(|s| {
            let (_, color) = s.split_once('#')?;
            let dir = &color[5..color.len() - 1];
            let dir = match dir {
                "0" => Dir::R,
                "1" => Dir::D,
                "2" => Dir::L,
                "3" => Dir::U,
                x => panic!("String must begin with [UDLR], but begins with {}.", x),
            };
            let dist = i64::from_str_radix(&color[..color.len() - 2], 16).ok()?;
            Some((dir, dist))
        })
        .collect();
    solve(plans)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";
    assert_eq!(part1(parse_input(input)), 62);
    assert_eq!(part2(parse_input(input)), 952408144115);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 50746);
    assert_eq!(part2(input), 70086216556038);
}
