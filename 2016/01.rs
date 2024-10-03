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
        if let Some(dir) = Nsew::from_usize(((*self) as isize - 1).rem_euclid(4) as usize) {
            dir
        } else {
            self.clone()
        }
    }

    fn right(&self) -> Nsew {
        if let Some(dir) = Nsew::from_usize(((*self) as usize + 1).rem_euclid(4)) {
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

fn moves(input: &str) -> impl Iterator<Item = Vector2> + '_ {
    let mut dir = Nsew::North;
    input
        .split(", ")
        .flat_map(move |instruction| {
            let times = instruction.get_numbers().next().unwrap();
            dir = if instruction.chars().next() == Some('L') { dir.left() } else { dir.right() };
            (0..times).map(move |_| dir)
        })
        .scan(Vector2::zero(), |state, dir| {
            *state = *state + match dir {
                Nsew::North => vector!(0, -1),
                Nsew::East => vector!(1, 0),
                Nsew::South => vector!(0, 1),
                Nsew::West => vector!(-1, 0),
            };
            Some(*state)
        })
}
fn part1(input: &'static str) -> usize {
    let moves = moves(input);
    moves.last().unwrap().manhattan_distance(Vector2::zero())
}

fn part2(input: &'static str) -> usize {
    let mut moves = moves(input);
    let mut visited = HashSet::new();
    moves.find(|pos| !visited.insert(*pos)).unwrap().manhattan_distance(Vector2::zero())
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
