use advent::prelude::*;

fn reallocate(state: &Vec<usize>) -> Vec<usize> {
    let mut new_list = state.clone();
    let (index, alloc) = state.into_iter().enumerate()
        .fold(None, |acc, (index, alloc)| match acc {
            None => Some((index, alloc)),
            Some((max_index, max)) => {
                if alloc > max {
                    Some((index, alloc))
                } else {
                    Some((max_index, max))
                }
            }
        })
        .unwrap();
    new_list[index] = 0;
    for i in 1..=*alloc {
        let new_index = (index + i) % new_list.len();
        new_list[new_index] += 1;
    }
    new_list
}
fn parse_input(input: &str) -> (usize, HashMap<Vec<usize>, usize>, Vec<usize>) {
    let mut state: Vec<usize> = input.trim_end().split('\t').map(|v| v.parse().unwrap()).collect();
    let mut insertion_index = 0usize;
    let mut index = 0usize;
    let mut set = HashMap::new();

    loop {
        set.insert(state.to_owned(), insertion_index);
        state = reallocate(&state);
        index += 1;
        if set.keys().contains(&state) { return (index, set, state) }
        insertion_index += 1;
    }
}

fn default_input() -> (usize, HashMap<Vec<usize>, usize>, Vec<usize>) {
    parse_input(include_input!(2017 / 06))
}

fn part1((index, _, _): (usize, HashMap<Vec<usize>, usize>, Vec<usize>)) -> usize {
    index
}

fn part2((index, set, last): (usize, HashMap<Vec<usize>, usize>, Vec<usize>)) -> usize {
    index - set[&last]
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 12841);
    assert_eq!(part2(input), 8038);
}
