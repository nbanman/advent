use std::process::exit;
use advent::prelude::*;
use itertools::unfold;
enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    fn flip(&self) -> Direction {
        match self {
            Direction::N => Direction::S,
            Direction::E => Direction::W,
            Direction::S => Direction::N,
            Direction::W => Direction::E,
        }
    }
}
fn default_input() -> &'static str {
    include_input!(2023 / 10)
}

// list of directions taken by the loop
fn get_pipe(field: &str) -> Vec<Direction> {

    let direction_map = get_direction_map();

    // get width of field
    let field_width = field.find('\n').unwrap();

    // start at 'S'
    let start_pos = field.find('S').unwrap();

    // find the initial direction by looking in each direction, and pick the first one that has a pipe
    // fitting that connects back to the start
    let east = field.chars().nth(move_along_pipe(&start_pos, &Direction::E, &field_width));
    let start_dir = if "7-J".contains(field.chars().nth(move_along_pipe(&start_pos, &Direction::E, &field_width)).unwrap()) {
        Direction::N
    } else if "7|F".contains(field.chars().nth(move_along_pipe(&start_pos, &Direction::N, &field_width)).unwrap()) {
        Direction::E
    } else {
        Direction::S
    };

    let move_direction = |(pos, dir): (usize, &Direction)| {
        let next_pos: usize = move_along_pipe(&pos, dir, &field_width);
        let neighbor_dir = direction_map
            .get(&(field.chars().nth(next_pos).unwrap(), *dir.copy()))
            .unwrap();
        (next_pos, neighbor_dir)
    };
    unfold(move_direction((start_pos, &start_dir)), |(next_pos, next_dir) | {
        Some(move_direction((*next_pos, next_dir)))
    })
        .take_while(|(pos, _)| field.chars().nth(*pos).unwrap() == 'S')
        .map(|(_, dir)| dir)
        .collect()
}

fn move_along_pipe(pos: &usize, dir: &Direction, width: &usize) -> usize {
    match dir {
        Direction::N => pos - width,
        Direction::E => pos + 1,
        Direction::S => pos + width,
        Direction::W => pos - 1,
    }
}

fn get_direction_map() -> HashMap<(char, Direction), Direction> {
    [
        (('S', Direction::N), Direction::N),
        (('S', Direction::S), Direction::S),
        (('S', Direction::E), Direction::E),
        (('S', Direction::W), Direction::W),
        (('|', Direction::N), Direction::N),
        (('|', Direction::S), Direction::S),
        (('-', Direction::E), Direction::E),
        (('-', Direction::W), Direction::W),
        (('L', Direction::S), Direction::E),
        (('L', Direction::W), Direction::N),
        (('J', Direction::S), Direction::W),
        (('J', Direction::E), Direction::N),
        (('7', Direction::E), Direction::S),
        (('7', Direction::N), Direction::W),
        (('F', Direction::W), Direction::S),
        (('F', Direction::N), Direction::E),
    ]
        .iter()
        .cloned()
        .collect()
}

fn part1(field: &str) -> usize {
    get_pipe(field).len() / 2
}

fn part2(field: &str) -> usize {
    get_pipe(field).len() / 2
}


fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 7086);
    // assert_eq!(part2(input), 317);
}
