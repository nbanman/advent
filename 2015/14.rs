use advent::prelude::*;

const SECONDS: usize = 2503;

#[derive(Clone, Debug)]
struct Reindeer {
    speed: usize,
    duration: usize,
    rest: usize,
}

impl Reindeer {
    fn distance_raced(&self, seconds: usize) -> usize {
        let interval = self.duration + self.rest;
        let whole_intervals = seconds / interval;
        let remainder = seconds % interval;
        whole_intervals * (self.speed * self.duration) + min(remainder, self.duration) * self.speed
    }
}
fn parse_input(input: &str) -> Vec<Reindeer> {
    input
        .get_numbers()
        .tuples()
        .map(|(speed, duration, rest)| Reindeer { speed, duration, rest })
        .collect()
}

fn default_input() -> Vec<Reindeer> {
    parse_input(include_input!(2015 / 14))
}

fn part1(racers: Vec<Reindeer>) -> usize {
    racers.into_iter()
        .map(|racer| racer.distance_raced(SECONDS))
        .max()
        .unwrap()
}

fn part2(racers: Vec<Reindeer>) -> usize {
    let mut leaderboard = vec![0; racers.len()];
    for t in 1..SECONDS {
        let distances: Vec<_> = racers.iter()
            .map(|racer| racer.distance_raced(t))
            .collect();
        let max_distance = distances.iter().max().unwrap().to_owned();
        for (racer, distance) in distances.into_iter().enumerate() {
            if distance == max_distance { leaderboard[racer] += 1; }
        }
    }
    leaderboard.into_iter().max().unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 2640);
    assert_eq!(part2(input), 1102);
}
