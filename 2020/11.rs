use advent::prelude::*;
use crate::Seat::{EmptySpace, Occupied, Unoccupied};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Seat {
    Occupied,
    Unoccupied,
    EmptySpace,
}

impl Seat {
    fn from(c: u8) -> Seat {
        match c {
            b'#' => Occupied,
            b'L' => Unoccupied,
            b'.' => EmptySpace,
            c    => panic!("Input contains unrecognized char: {}", c as char)
        }
    }
}
type Grid = (Vec<Seat>, usize);

fn parse_input(input: &str) -> Grid {
    let input = input.as_bytes();
    let width = input.iter().position(|c| c == &b'\n').unwrap();
    let layout = input
        .iter()
        .filter(|&&c| c != b'\n')
        .map(|&c| Seat::from(c))
        .collect();
    (layout, width)
}

fn default_input() -> Grid {
    parse_input(include_input!(2020 / 11))
}

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn solve<F>(grid: Grid, tolerance: usize, get_neighbors: F) -> usize
where
    F: Fn(&Grid, usize) -> usize,
{
    let (new_grid, _) = iter::successors(Some(grid), |acc| {
        let layout = acc.0
            .iter()
            .enumerate()
            .map(|(idx, &seat)| {
                if matches!(seat, EmptySpace) {
                    EmptySpace
                } else {
                    let is_occupied = matches!(seat, Occupied);
                    let neighbors = get_neighbors(acc, idx);
                    if is_occupied && neighbors < tolerance || !is_occupied && neighbors == 0 {
                        Occupied
                    } else {
                        Unoccupied
                    }
                }
            })
            .collect();
        Some((layout, acc.1))
    })
        .tuple_windows()
        .find(|(prev, next)| prev == next)
        .unwrap();

    new_grid.0.iter().filter(|seat| matches!(seat, Occupied)).count()
}

fn part1(grid: Grid) -> usize {
    let get_neighbors = |grid: &Grid, idx: usize| {
        let width = grid.1 as isize;
        let height = grid.0.len() as isize / width;
        let x = idx as isize % width;
        let y = idx as isize / width;
        DIRECTIONS
            .iter()
            .filter_map(|(dx, dy)| {
                let new_x = *dx + x;
                if new_x >= 0 && new_x < width {
                    let new_y = *dy + y;
                    if new_y >= 0 && new_y < height {
                        let pos = new_y * width + new_x;
                        Some(pos)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .filter(|pos| matches!(grid.0[*pos as usize], Occupied))
            .count()
    };
    solve(grid, 4, get_neighbors)
}

fn part2(grid: Grid) -> usize {
    let get_neighbors = |grid: &Grid, idx: usize| {
        let width = grid.1 as isize;
        let height = grid.0.len() as isize / width;
        let x = idx as isize % width;
        let y = idx as isize / width;
        let count = DIRECTIONS
            .iter()
            .filter(|(slope_x, slope_y)| {
                let successors: Vec<_> = iter::successors(Some((x, y)), |(dx, dy)| {
                    let new_x = *dx + slope_x;
                    if new_x >= 0 && new_x < width {
                        let new_y = *dy + slope_y;
                        if new_y >= 0 && new_y < height {
                            let pos = (new_y * width + new_x) as usize;
                            if matches!(grid.0[pos], Unoccupied) {
                                None
                            } else {
                                Some((new_x, new_y))
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                    .map(|(new_x, new_y)| {
                        let pos = (new_y * width + new_x) as usize;
                        grid.0[pos]
                    })
                    .collect();
                successors.into_iter().dropping(1).any(|seat| matches!(seat, Occupied))
            })
            .count();
        count
    };
    solve(grid, 5, get_neighbors)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
    );
    assert_eq!(part1(input.clone()), 37);
    assert_eq!(part2(input), 26);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2243);
    assert_eq!(part2(input), 2027);
}
