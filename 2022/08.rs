use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<Vector2, u8> {
    parse_map(input, |c| c.to_digit(10).unwrap() as u8)
}

fn default_input() -> HashMap<Vector2, u8> {
    parse_input(include_str!("input/08.txt"))
}

const NORTH: Vector2 = vector![0, 1];
const EAST: Vector2 = vector![1, 0];
const SOUTH: Vector2 = vector![0, -1];
const WEST: Vector2 = vector![-1, 0];
const CARDINALS: [Vector2; 4] = [NORTH, EAST, SOUTH, WEST];

fn part1(map: HashMap<Vector2, u8>) -> usize {
    map.iter()
        .filter(|(&center, &height)| {
            CARDINALS.iter().any(|d| {
                iter::successors(Some(center + d), |p| Some(p + d))
                    .map_while(|p| map.get(&p).copied())
                    .all(|h| h < height)
            })
        })
        .count()
}

fn part2(map: HashMap<Vector2, u8>) -> usize {
    map.iter()
        .map(|(&center, &height)| {
            CARDINALS
                .iter()
                .map(|d| {
                    // FIXME: The scan could be avoided if there existed a
                    // `take_while_inclusive` iterator method.
                    // E.g.
                    //     iter::successors(Some(center + d), |p| Some(p + d))
                    //         .map_while(|p| map.get(&p).copied())
                    //         .take_while_inclusive(|h| h < height)
                    //         .count()
                    iter::successors(Some(center + d), |p| Some(p + d))
                        .map_while(|p| map.get(&p).copied())
                        .scan(false, |done, h| {
                            (!*done).some_with(|| {
                                if h >= height {
                                    *done = true;
                                }
                                h
                            })
                        })
                        .count()
                })
                .product()
        })
        .max()
        .unwrap()
}

fn main() {
    let mut run = advent::start();
    run.part(|| part1(default_input()));
    run.part(|| part2(default_input()));
    run.finish();
}

#[test]
fn example() {
    let input = parse_input(
        "30373
25512
65332
33549
35390",
    );
    assert_eq!(part1(input.clone()), 21);
    assert_eq!(part2(input), 8);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1845);
    assert_eq!(part2(input), 230112);
}