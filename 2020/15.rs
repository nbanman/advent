use advent::prelude::*;

fn default_input() -> Vec<usize> {
    parse_input(include_input!(2020 / 15))
}

fn parse_input(input: &str) -> Vec<usize> {
    get_numbers(input)
}

fn last_number_spoken(start: Vec<usize>, iterations: usize) -> usize {
    let bounds = iterations / 10;
    let mut low = vec![0usize; bounds];
    let mut high = HashMap::with_capacity(bounds / 2);

    for (turn, &n) in start.iter().enumerate() {
        low[n] = turn + 1;
    }

    let mut current = 0;
    for turn in start.len() + 1..iterations {
        if current < bounds {
            let prev = &mut low[current];
            current = if *prev == 0 { 0 } else { turn - *prev };
            *prev = turn;
        } else {
            current = high.insert(current, turn)
                .map(|prev| turn - prev)
                .unwrap_or(0);
        }
    }
    current
}

fn part1(start: Vec<usize>) -> usize {
    last_number_spoken(start, 2020)
}

fn part2(start: Vec<usize>) -> usize {
    last_number_spoken(start, 30000000)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    for (input, p1, p2) in [
        (vec![0, 3, 6], 436, 175594),
        (vec![1, 3, 2], 1, 2578),
        (vec![2, 1, 3], 10, 3544142),
        (vec![1, 2, 3], 27, 261214),
        (vec![2, 3, 1], 78, 6895259),
        (vec![3, 2, 1], 438, 18),
        (vec![3, 1, 2], 1836, 362),
    ] {
        assert_eq!(part1(input.clone()), p1);
        assert_eq!(part2(input), p2);
    }
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 929);
    assert_eq!(part2(input), 16671510);
}
