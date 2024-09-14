use advent::prelude::*;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Point(i64, i64);

impl Point {
    fn moove(&self, dir: &u8) -> Self {
        let mut x = self.0;
        let mut y = self.1;
        match dir {
            &b'^' => { y -= 1 },
            &b'v' => { y += 1 },
            &b'<' => { x -= 1 },
            &b'>' => { x += 1 },
            _ => {},
        }
        Point(x, y)
    }
}
fn default_input() -> &'static str {
    include_input!(2015 / 03)
}

fn deliver(input: &str) -> HashSet<Point> {
    input.as_bytes().into_iter()
        .scan(Point(0, 0), |state, dir| {
            *state = state.moove(dir);
            Some(*state)
        })
        .collect()
}

fn collate(s: &str, threads: usize) -> Vec<String> {
    let mut partitions = vec![String::new(); threads];
    for (index, c) in s.chars().enumerate() {
        partitions[index % threads].push(c);
    }
    partitions
}
fn part1(input: &str) -> usize {
    deliver(input).len()
}

fn part2(input: &str) -> usize {
    collate(input, 2)
        .iter()
        .map(|s| deliver(s))
        .reduce(|state, x| state.union(&x).cloned().collect())
        .unwrap()
        .len()

}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2081);
    assert_eq!(part2(input), 2341);
}
