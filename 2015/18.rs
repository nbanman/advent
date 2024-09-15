use advent::prelude::*;
fn default_input() -> &'static str { include_input!(2015 / 18) }

fn get_corners_and_neighbors(lights: &Vec<u8>, width: usize) -> (Vec<usize>, Vec<isize>) {
    let corners = vec![0, width - 2, lights.len() - width, lights.len() - 2];
    let width = width as isize;
    let neighbors = vec![
        -width - 1,
        -width,
        -width + 1,
        -1,
        1,
        width - 1,
        width,
        width + 1,
    ];
    (corners, neighbors)
}
fn iterate(
    lights: &Vec<u8>,
    corners: &Vec<usize>,
    neighbors: &Vec<isize>,
    corners_stuck: bool
) -> Vec<u8> {
    let next: Vec<u8> = lights.into_iter().enumerate()
        .map(|(idx, lit)| {
            if corners_stuck && corners.contains(&idx) {
                b'#'
            } else {
                if *lit == b'\n' {
                    b'\n'
                } else {
                    let lit_neighbors = neighbors.iter()
                        .filter_map(|&offset| {
                            let pos = idx as isize + offset;
                            lights.try_get(pos)
                        })
                        .filter(|x| **x == b'#')
                        .count();
                    if lit_neighbors == 3 || (lit_neighbors == 2 && *lit == b'#') {
                        b'#'
                    } else {
                        b'.'
                    }
                }
            }
        })
        .collect();
    next
}

fn solve(input: &str, corners_stuck: bool) -> usize {
    let input = input.as_bytes();
    let width = input.iter().position(|c| *c == b'\n').unwrap() + 1;
    let lights: Vec<_> = input.into_iter().cloned().collect();
    let (corners, neighbors) = get_corners_and_neighbors(&lights, width);
    iter::successors(Some(lights), |l| {
        Some(iterate(l, &corners, &neighbors, corners_stuck))
    })
        .take(101)
        .last()
        .unwrap()
        .into_iter()
        .filter(|&it| it == b'#')
        .count()
}

fn part1(input: &str) -> usize {
    solve(input, false)
}

fn part2(input: &str) -> usize {
    solve(input, true)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    // assert_eq!(part1(input), 1061);
    assert_eq!(part2(input), 1006);
}

#[test]
fn example1() {
    let input = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";
    assert_eq!(part1(input), 4);
}