use advent::prelude::*;

/// Takes input and converts it to a Vec of coordinates, one for each movement of the original
/// rope knot.
fn parse_input(input: &str) -> Vec<Vector2> {
    input
        .lines()
        // flattens out a vec containing vectors repeated several times
        .flat_map(|line| {
            // for each line, separate the direction (UDLR) and the distance (#), then convert the
            // direction to a vector and the distance to usize
            let (dir, dist) = line.split_once(' ').unwrap();
            let dir = match dir {
                "U" => vector![0, -1],
                "D" => vector![0, 1],
                "L" => vector![-1, 0],
                "R" => vector![1, 0],
                dir => panic!("unknown direction `{}`", dir),
            };
            let dist = dist.parse::<usize>().unwrap();
            vec![dir; dist]
        })
        // takes the flattened directions and uses it to plot the movement of the original knot
        .scan(Vector2::zero(), |pos, dir| {
            *pos = *pos + dir;
            Some(*pos)
        })
        .collect()
}

fn default_input() -> Vec<Vector2> { parse_input(include_input!(2022 / 09)) }

fn solve(first_knot: Vec<Vector2>, knots: usize) -> usize {
    (1..knots)
        .fold(first_knot, |prev_rope, _| {
            let mut pos = Vector2::zero();
            let mut rope = vec![pos];
            for prev_pos in prev_rope {
                let diff = prev_pos - pos;
                if diff.x.abs() > 1 || diff.y.abs() > 1 {
                    pos.x += diff.x.signum();
                    pos.y += diff.y.signum();
                    rope.push(pos);  // note pos is copy so no clone() needed
                }
            }
            rope
        })
        .into_iter()
        .collect::<HashSet<_>>()
        .len()
}

fn part1(first_rope: Vec<Vector2>) -> usize {
    solve(first_rope, 2)
}

fn part2(first_rope: Vec<Vector2>) -> usize {
    solve(first_rope, 10)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2",
    );
    assert_eq!(part1(input), 13);
}

#[test]
fn example2() {
    let input = parse_input(
        "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20",
    );
    assert_eq!(part2(input), 36);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 6175);
    assert_eq!(part2(input), 2578);
}
