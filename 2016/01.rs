use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2016 / 01)
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(usize)]
enum Nsew {
    North,
    East,
    South,
    West,
}

impl Nsew {
    fn left(&self) -> Nsew {
        if let Some(dir) = Nsew::from_usize((((*self) as isize - 1).rem_euclid(4)) as usize) {
            dir
        } else {
            self.clone()
        }
    }

    fn right(&self) -> Nsew {
        if let Some(dir) = Nsew::from_usize((((*self) as usize + 1).rem_euclid(4))) {
            dir
        } else {
            self.clone()
        }
    }

    fn from_usize(value: usize) -> Option<Nsew> {
        match value {
            0 => Some(Nsew::North),
            1 => Some(Nsew::East),
            2 => Some(Nsew::South),
            3 => Some(Nsew::West),
            _ => None,
        }
    }
}

trait Manhattan {
    fn manhattan_distance(&self, other: Self) -> usize;
}

impl Manhattan for Vector2 {
    fn manhattan_distance(&self, other: Self) -> usize {
        (self.x - other.x).abs() as usize + (self.y - other.y).abs() as usize
    }
}

fn part1(input: &'static str) -> usize {
    let mut dir = Nsew::North;
    let mut pos = Vector2::zero();
    for instruction in input.split(", ") {
        let times = instruction.get_numbers().next().unwrap();
        dir = if instruction.chars().next() == Some('L') { dir.left() } else { dir.right() };
        for i in 0..times {
            pos += match dir {
                Nsew::North => vector!(0, -1),
                Nsew::East => vector!(1, 0),
                Nsew::South => vector!(0, 1),
                Nsew::West => vector!(-1, 0),
            };
        }
    }
    pos.manhattan_distance(Vector2::zero())
}

fn part2(input: &'static str) -> usize {
    let mut visited = HashSet::new();
    let mut dir = Nsew::North;
    let mut pos = Vector2::zero();
    for instruction in input.split(", ") {
        let times = instruction.get_numbers().next().unwrap();
        dir = if instruction.chars().next() == Some('L') { dir.left() } else { dir.right() };
        for i in 0..times {
            pos += match dir {
                Nsew::North => vector!(0, -1),
                Nsew::East => vector!(1, 0),
                Nsew::South => vector!(0, 1),
                Nsew::West => vector!(-1, 0),
            };
            if !visited.insert(pos) {
                return pos.manhattan_distance(Vector2::zero())
            }
        }
    }
    unreachable!()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 226);
    assert_eq!(part2(input), 79);
}
