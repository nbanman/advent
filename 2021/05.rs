use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(Vector2, Vector2)> {
    input
        .get_numbers()
        .array_chunked()
        .map(|[x1, y1, x2, y2]| (vector![x1, y1], vector![x2, y2]))
        .collect()
}

fn default_input() -> Vec<(Vector2, Vector2)> {
    parse_input(include_input!(2021 / 05))
}

fn straight_range(
    (a, b): (Vector2, Vector2), include_diagonals: bool,
) -> Vec<Vector2> {
    let mut range = Vec::new();
    if a.x == b.x {
        let (&small, &large) = min_max(&[a.y, b.y]);
        for y in small..=large {
            range.push(vector![a.x, y]);
        }
    } else if a.y == b.y {
        let (&small, &large) = min_max(&[a.x, b.x]);
        for x in small..=large {
            range.push(vector![x, a.y]);
        }
    } else if include_diagonals {
        let y_increment = if a.y < b.y {
            1i64
        } else {
            -1i64
        };
        if a.x < b.x {
            for (idx, x) in (a.x..=b.x).enumerate() {
                range.push(vector!(x, a.y + idx as i64 * y_increment))
            }
        } else {
            for (idx, x) in (b.x..=a.x).rev().enumerate() {
                range.push(vector!(x, a.y + idx as i64 * y_increment))
            }
        };
    }
    range
}

fn solve(input: impl Iterator<Item=(Vector2, Vector2)>, include_diagonals: bool) -> usize {
    let mut counter: HashMap<Vector2, usize> = HashMap::new();
    input
        .flat_map(|line| straight_range(line, include_diagonals))
        .for_each(|pos| {
            let current = counter.get(&pos).unwrap_or(&0) + 1;
            counter.insert(pos, current);
        });
    counter.values().filter(|&v| v > &1).count()
}

fn part1(input: Vec<(Vector2, Vector2)>) -> usize {
    solve(input.into_iter(), false)
}

fn part2(input: Vec<(Vector2, Vector2)>) -> usize {
    solve(input.into_iter(), true)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
    );
    assert_eq!(part1(input.clone()), 5);
    assert_eq!(part2(input), 12);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 5774);
    assert_eq!(part2(input), 18423);
}
