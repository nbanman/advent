use advent::prelude::*;

fn parse_input(input: &'static str) -> usize {
    input.trim_end().parse().unwrap()
}

fn default_input() -> usize { parse_input(include_input!(2015 / 20)) }

fn part1(input: usize) -> usize {
    solve(input, 10, |_, _| true)
}

fn part2(input: usize) -> usize {
    solve(input, 11, |house_number: usize, elf: usize| elf * 50 > house_number)
}

fn solve<F>(minimum_presents: usize, multiplier: usize, predicate: F) -> usize
where
    F: Fn(usize, usize) -> bool
{
    (1..).into_iter()
        .find(|house_number| {
            let elves: Vec<_> =
                expand_factors(prime_factors(house_number), vec![1]).into_iter()
                    .filter(|factor| predicate(*house_number, *factor))
                    .collect();
            let presents = elves.iter().fold(0, |acc, i| acc + i * multiplier);
            presents >= minimum_presents
        })
        .unwrap()
}

fn expand_factors(prime_factors: Vec<usize>, factors: Vec<usize>) -> Vec<usize> {
    if prime_factors.is_empty() {
        factors
    } else {
        let first = *prime_factors.first().unwrap();

        let latest: Vec<_> = prime_factors.iter()
            .take_while(|it| **it == first)
            .scan(1, |state, factor| {
                *state *= factor;
                Some(*state)
            }).collect();

        let mut new_factors = Vec::new();
        for factor in factors {
            new_factors.push(factor);
            for recent in latest.iter() {
                new_factors.push(factor * recent);
            }
        }
        let latest: HashSet<_> = latest.into_iter().collect();
        let reduced_primes = prime_factors.into_iter()
            .filter(|x| !latest.contains(x))
            .collect();
        expand_factors(reduced_primes, new_factors)
    }
}

fn prime_factors(n: &usize) -> Vec<usize> {
    let mut factors = Vec::new();
    let mut n = *n;
    while n & 1 == 0 {
        factors.push(2);
        n /= 2
    }
    for i in (3..=(n as f64).sqrt() as usize).step_by(2) {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
    }
    if n > 2 { factors.push(n) }
    factors
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 776160);
    assert_eq!(part2(input), 786240);
}

#[test]
fn example1() {
    let input = "210";
    assert_eq!(part1(parse_input(input)), 12);
}