use advent::prelude::*;

const DIRECTIONS: [Vector<i64, 2>; 4] = [
    Vector2::new(0, -1), // N
    Vector2::new(1, 0), // E
    Vector2::new(0, 1), // S
    Vector2::new(-1, 0), // W
];

const DIAGONALS: [Vector<i64, 2>; 8] = [
    Vector2::new(-1, -1),
    Vector2::new(0, -1),
    Vector2::new(1, -1),
    Vector2::new(1, 0),
    Vector2::new(1, 1),
    Vector2::new(0, 1),
    Vector2::new(-1, 1),
    Vector2::new(-1, 0),
];

fn parse_input(input: &str) -> usize {
    input.trim_end().parse().unwrap()
}

fn default_input() -> usize {
    parse_input(include_input!(2017 / 03))
}

fn part1(input: usize) -> usize {
    let square_root = (input as f64).sqrt().ceil() as usize;
    let square_root = if square_root & 1 == 0 { square_root + 1 } else { square_root };
    let furthest = (square_root / 2) * 2;
    let br = square_root.pow(2);
    let diff = (br - input) % furthest;
    furthest - min(diff, furthest / 2) + max(0, diff - furthest / 2)
}

fn part2(input: usize) -> usize {
    let mut dir = 2i8;
    let mut pos = Vector2::zero();
    let mut vel = 0i64;
    let mut space = HashMap::new();
    space.insert(pos.clone(), 1i64);
    loop {
        dir = (dir - 1).rem_euclid(DIRECTIONS.len() as i8);
        if dir == 1 || dir == 3 { vel += 1; }
        for _ in 1..=vel {
            pos = pos + DIRECTIONS[dir as usize];
            let square_val = DIAGONALS.iter()
                .map(|&dir| {
                    let neighbor = pos + dir;
                    space.get(&neighbor).unwrap_or(&0)
                })
                .sum::<i64>() as usize;
            if square_val > input { return square_val; }
            space.insert(pos, square_val as i64);
        }
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 552);
    assert_eq!(part2(input), 330785);
}
