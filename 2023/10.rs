use advent::prelude::*;

// Somehow 3x slower than Kotlin version!

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    N,
    E,
    S,
    W,
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

    let start_dir = if "7|F".contains(field.chars().nth(move_along_pipe(&start_pos, &Direction::N, &field_width)).unwrap()) {
        Direction::N
    } else if "7-J".contains(field.chars().nth(move_along_pipe(&start_pos, &Direction::E, &field_width)).unwrap()) {
        Direction::E
    } else {
        Direction::S
    };

    let move_direction = |(pos, dir): (usize, &Direction)| {
        let next_pos: usize = move_along_pipe(&pos, dir, &field_width);
        let neighbor_dir = direction_map
            .get(&(field.chars().nth(next_pos).unwrap(), dir.clone()))
            .unwrap();
        (next_pos, neighbor_dir)
    };

    let mut directions = vec![start_dir];

    let mut pos = start_pos;
    let mut direction = start_dir.clone();
    loop {
        let (new_pos, new_dir) = move_direction((pos, &direction));
        if let Some(x) = field.chars().nth(new_pos) {
            if x == 'S' { break };
        }
        pos = new_pos;
        direction = new_dir.clone();
        directions.push(direction);
    }

    directions
}

fn move_along_pipe(pos: &usize, dir: &Direction, width: &usize) -> usize {
    match dir {
        Direction::N => pos - (width + 1),
        Direction::E => pos + 1,
        Direction::S => pos + (width + 1),
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
    let pipe = get_pipe(field);
    let (area, _) = pipe
        .iter()
        .fold((0isize, 0isize), |(sum, d), dir| {
            match dir {
                Direction::N => (sum, d + 1),
                Direction::S => (sum, d - 1),
                Direction::W => (sum - d, d),
                Direction::E => (sum + d, d),
            }
        });

    area.abs() as usize - (pipe.len() / 2) + 1
}


fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";
    assert_eq!(part1(input), 22);
    assert_eq!(part2(input), 5);
}
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 7086);
    assert_eq!(part2(input), 317);
}
