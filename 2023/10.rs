use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 10)
}

// list of directions taken by the loop
fn get_pipe(field: &str) -> Vec<Direction> {
    let field = field.as_bytes();

    // get width of field
    let field_width = field.iter().position(|&x| x == b'\n').unwrap() + 1;

    // start at 'S'
    let start_pos = field.iter().position(|&x| x == b'S').unwrap();

    let start_dir = if "7|F".contains(field[move_along_pipe(&start_pos, Direction::N, &field_width)] as char) {
        Direction::N
    } else if "7-J".contains(field[move_along_pipe(&start_pos, Direction::E, &field_width)] as char) {
        Direction::E
    } else {
        Direction::S
    };

    // lambda for moving along the pipe, taking in the position and direction and returning the next
    // position and direction
    let move_direction = |(pos, dir): (usize, Direction)| {
        // calculate next position
        let next_pos: usize = move_along_pipe(&pos, dir, &field_width);

        // Calculate next direction by looking at the character in the next position.
        // If it's 'L' or '7', that means East/West turns right and North/South turns left.
        // If it's 'J' or 'F', that means East/West turns left and North/South turns right.
        // Otherwise, go straight.
        let neighbor_dir = match field[next_pos] {
            b'L' | b'7' => if dir.ordinal() & 1 == 1 { dir.right() } else { dir.left() },
            b'J' | b'F' => if dir.ordinal() & 1 == 0 { dir.right() } else { dir.left() },
            _ => dir,
        };
        (next_pos, neighbor_dir)
    };

    // move along the pipe recording positions until we hit 'S'
    let mut directions = vec![start_dir];
    let mut pos = start_pos;
    let mut direction = start_dir;
    loop {
        let (new_pos, new_dir) = move_direction((pos, direction));
        if field[new_pos] == b'S' { break }
        pos = new_pos;
        direction = new_dir;
        directions.push(direction);
    }
    directions
}

fn move_along_pipe(pos: &usize, dir: Direction, width: &usize) -> usize {
    match dir {
        Direction::N => pos - width,
        Direction::E => pos + 1,
        Direction::S => pos + width,
        Direction::W => pos - 1,
    }
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
