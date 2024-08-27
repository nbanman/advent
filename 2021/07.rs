use std::ops::RangeInclusive;

use itertools::MinMaxResult::MinMax;

use advent::prelude::*;

fn parse_input(input: &str) -> (Vec<usize>, RangeInclusive<usize>) {
    let crabs: Vec<usize> = get_numbers(input);
    if let MinMax(&min, &max) = crabs.iter().minmax() {
        (crabs, min..=max)
    } else {
        unreachable!("Parsing error")
    }
}

fn default_input() -> (Vec<usize>, RangeInclusive<usize>) {
    parse_input(include_input!(2021 / 07))
}

fn optimal_alignment_cost<F>(crabs: &Vec<usize>, range: RangeInclusive<usize>, fuel_cost: F) -> usize
    where
        F: Fn(usize) -> usize,
{
    fn alignment_cost<F>(crabs: &Vec<usize>, position: usize, fuel_cost: F) -> usize
        where
            F: Fn(usize) -> usize,
    {
        crabs.iter()
            .map(|crab| {
                fuel_cost((*crab as isize - position as isize).abs() as usize)
            })
            .sum()
    }

    fn mid_point(range: &RangeInclusive<usize>) -> usize {
        (range.end() - range.start()) / 2 + range.start()
    }

    if range.start() == range.end() {
        alignment_cost(crabs, *range.start(), &fuel_cost)
    } else {
        let midpoint = mid_point(&range);
        let new_range = vec![*range.start()..=midpoint, midpoint..=*range.end()]
            .into_iter()
            .min_by(|a, b| {
                let a = alignment_cost(crabs, mid_point(a), &fuel_cost);
                let b = alignment_cost(crabs, mid_point(b), &fuel_cost);
                a.cmp(&b)
            })
            .unwrap();
        optimal_alignment_cost(crabs, new_range, fuel_cost)
    }
}

fn part1((crabs, crab_range): (Vec<usize>, RangeInclusive<usize>)) -> usize {
    optimal_alignment_cost(&crabs, crab_range, |crab| crab)
}

fn part2((crabs, crab_range): (Vec<usize>, RangeInclusive<usize>)) -> usize {
    optimal_alignment_cost(&crabs, crab_range, |crab| (1..=crab).sum())
}


fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("16,1,2,0,4,2,7,1,2,14");
    assert_eq!(part1(input.clone()), 37);
    assert_eq!(part2(input), 168);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 343468);
    assert_eq!(part2(input), 96086265);
}
