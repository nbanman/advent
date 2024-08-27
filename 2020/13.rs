use advent::prelude::*;

#[derive(Clone)]
struct Bus {
    id: i64,
    offset: i64,
}

struct State {
    time: i64,
    bus_id: i64,
}

fn modular_inverse(ni: i64, modulus: i64) -> i64 {
    iter::successors(Some(1), |i| Some(i + 1))
        .find(|&n| (ni % modulus * n) % modulus == 1)
        .unwrap()
}

fn crt(buses: Vec<Bus>) -> Bus {
    let n = buses
        .iter()
        .fold(1, |acc, bus| acc * bus.id);
    let big_phase: i64 = buses
        .iter()
        .map(|bus| {
            let ni = n / bus.id;
            bus.offset * ni * modular_inverse(ni, bus.id)
        })
        .sum();
    Bus { id: n, offset: big_phase % n }
}


fn parse_input(input: &str) -> (i64, Vec<Bus>) {
    if let Some((start, buses)) = input.split_once('\n') {
        let start = start.parse().unwrap();
        let buses: Vec<_> = buses
            .split(',')
            .enumerate()
            .filter_map(|(idx, s)| {
                if s == "x" {
                    None
                } else {
                    let id = s.trim_end().parse().unwrap();
                    Some(Bus { id, offset: idx as i64 })
                }
            })
            .collect();
        (start, buses)
    } else {
        panic!("Invalid input!")
    }
}

fn default_input() -> (i64, Vec<Bus>) {
    parse_input(include_input!(2020 / 13))
}

fn part1((start, buses): (i64, Vec<Bus>)) -> i64 {
    let time_sequence =
        iter::successors(Some(State { time: start, bus_id: 0 }), |state| {
            let time = state.time + 1;
            let bus_id = buses.iter()
                .find(|bus| time % bus.id == 0)
                .map(|bus| bus.id)
                .unwrap_or(0);
            Some(State { time, bus_id })
        });
    if let Some(state) = time_sequence
        .into_iter()
        .find(|state| state.bus_id != 0) {
        state.bus_id * (state.time - start)
    } else {
        panic!("No bus found!")
    }
}

fn part2((_, buses): (i64, Vec<Bus>)) -> i64 {
    let bus = crt(buses);
    bus.id - bus.offset
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("939\n7,13,x,x,59,x,31,19");
    assert_eq!(part1(input.clone()), 295);
    assert_eq!(part2(input), 1068781);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 115);
    assert_eq!(part2(input), 756261495958122);
}
