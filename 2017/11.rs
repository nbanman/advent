use std::fmt::{Display, Formatter};
use advent::prelude::*;

#[derive(Copy, Clone)]
struct Hexagon {
    q: i64,
    r: i64,
    s: i64,
}

impl Hexagon {
    fn new(q: i64, r: i64) -> Hexagon {
        Hexagon { q, r, s: -q - r }
    }

    fn origin() -> Hexagon { Hexagon { q: 0, r: 0, s: 0, } }

    fn hex_at(&self, step: &str) -> Hexagon {
        match step {
            "n" => Hexagon::new(self.q, self.r - 1),
            "s" => Hexagon::new(self.q, self.r + 1),
            "nw" => Hexagon::new(self.q - 1, self.r),
            "ne" => Hexagon::new(self.q + 1, self.r - 1),
            "sw" => Hexagon::new(self.q - 1, self.r + 1),
            "se" => Hexagon::new(self.q + 1, self.r),
            s => panic!("cannot parse '{}'.", s),
        }
    }

    fn distance(&self, other: &Hexagon) -> usize {
        let v = Hexagon::new(
            self.q - other.q,
            self.r - other.r
        );
        max(max(v.q.abs(), v.r.abs()), v.s.abs()) as usize
    }
}

impl Display for Hexagon {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hexagon(q: {}, r: {}, s: {})", self.q, self.r, self.s)
    }
}
fn parse_input(input: &str) -> Vec<Hexagon> {
    input.trim_end().split(',')
        .scan(Hexagon::origin(), |state, direction| {
            *state = state.hex_at(direction);
            Some(*state)
        })
        .collect()
}

fn default_input() -> Vec<Hexagon> {
    parse_input(include_input!(2017 / 11))
}

fn part1(path: Vec<Hexagon>) -> usize {
    path.last().unwrap().distance(&Hexagon::origin())
}

fn part2(path: Vec<Hexagon>) -> usize {
    let origin = Hexagon::origin();
    path.iter()
        .map(|hex| hex.distance(&origin))
        .max()
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 747);
    assert_eq!(part2(input), 1544);
}
