use advent::prelude::*;

fn default_input() -> usize {
    include_input!(2018 / 11).trim().parse().unwrap()
}

const LENGTH: usize = 300;

struct Square {
    x: i64,
    y: i64,
    size: usize,
    power: i64,
}

fn solve(serial: usize, smallest: usize, largest: usize) -> Square {
    let cells: Vec<Vec<_>> = (0..LENGTH)
        .map(|y| {
            (0..LENGTH)
                .map(|x| {
                    let rack_id = x + 10;
                    ((((rack_id * y + serial) * rack_id) % 1000) / 100) as i64 - 5
                })
                .collect()
        })
        .collect();

    let mut grid = cells.clone();
    let mut max = Square {
        x: 0,
        y: 0,
        size: 0,
        power: 0,
    };

    for size in 1..=largest {
        for y in 0..=LENGTH - size {
            if size >= smallest {
                let mut power = grid[y].iter().take(size).sum();
                for x in size..LENGTH {
                    if power > max.power {
                        max = Square {
                            x: x as i64 - size as i64,
                            y: y as i64,
                            size,
                            power,
                        };
                    }
                    power += grid[y][x] - grid[y][x - size]
                }
            }
            if y < LENGTH - size {
                for x in 0..LENGTH {
                    grid[y][x] += cells[y + size][x];
                }
            }
        }
    }
    max
}

fn part1(serial: usize) -> String {
    let square = solve(serial, 3, 3);
    format!("{},{}", square.x, square.y)
}

fn part2(serial: usize) -> String {
    let square = solve(serial, 1, 300);
    format!("{},{},{}", square.x, square.y, square.size)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    for (input, exp) in [
        (18, String::from("33,45")),
        (42, String::from("232, 251, 12")),
    ] {
        assert_eq!(part1(input), exp);
        assert_eq!(part2(input), exp);
    }
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), String::from("235,48"));
    assert_eq!(part2(input), String::from("285,113,11"));
}
