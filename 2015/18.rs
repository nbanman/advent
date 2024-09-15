use advent::prelude::*;
fn parse_input(input: &str) -> (usize, Vec<bool>) {
    let input = input.as_bytes();
    let width = input.iter().position(|c| *c == b'\n').unwrap();
    let lights = input.iter()
        .filter(|c| **c != b'\n')
        .map(|c| *c == b'#')
        .collect();
    (width, lights)
}

fn default_input() -> (usize, Vec<bool>) {
    parse_input(include_input!(2015 / 18))
}

fn get_corners_and_neighbors(lights: &Vec<bool>, width: usize) -> (Vec<usize>, Vec<isize>) {
    let corners = vec![0, width - 1, lights.len() - width, lights.len() - 1];
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
    lights: &Vec<bool>,
    corners: &Vec<usize>,
    neighbors: &Vec<isize>,
    width: usize,
    corners_stuck: bool
) -> Vec<bool> {
    let next: Vec<bool> = lights.into_iter().enumerate()
        .map(|(idx, lit)| {
            if corners_stuck && corners.contains(&idx) {
                true
            } else {
                let lit_neighbors = neighbors.iter()
                    .filter_map(|&offset| {
                        let pos = idx as isize + offset;
                        lights.try_get(pos)
                    })
                    .filter(|x| **x == true)
                    .count();
                lit_neighbors == 3 || (lit_neighbors == 2 && *lit)
            }
        })
        .collect();
    println!("{}", next.iter().filter(|it| **it == true).count());
    next
}

fn solve(lights: Vec<bool>, width: usize, corners_stuck: bool) -> usize {
    let (corners, neighbors) = get_corners_and_neighbors(&lights, width);
    println!("{}", lights.iter().filter(|it| **it == true).count());
    iter::successors(Some(lights), |l| {
        Some(iterate(l, &corners, &neighbors, width, corners_stuck))
    })
        .take(101)
        .last()
        .unwrap()
        .into_iter()
        .filter(|&it| it == true)
        .count()
}

fn part1((width, lights): (usize, Vec<bool>)) -> usize {
    solve(lights, width, false)
}

fn part2((width, lights): (usize, Vec<bool>)) -> usize {
    solve(lights, width, true)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1061);
    assert_eq!(part2(input), 1006);
}

#[test]
fn example1() {
    let input = parse_input(".#.#.#
...##.
#....#
..#...
#.#..#
####..");
    assert_eq!(part1(input), 4);
}