use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2015 / 17)
}

fn get_containers(input: &str) -> Vec<Vec<usize>> {
    let max_storage = 150;
    let mut combos: Vec<Vec<usize>> = Vec::new();
    for container in input.get_numbers() {
        for index in 0..combos.len() {
            let storage: usize = combos[index].iter().sum::<usize>() + container;
            if storage <= max_storage {
                let mut new_combo = combos[index].clone();
                new_combo.push(container);
                combos.push(new_combo);
            }
        }
        combos.push(vec![container]);
    }
    combos.into_iter()
        .filter(|combo| combo.iter().sum::<usize>() == max_storage)
        .collect()
}

fn part1(input: &str) -> usize {
    let containers = get_containers(input);
    containers.len()
}

fn part2(input: &str) -> usize {
    let containers = get_containers(input);
    let minimum_containers = containers.iter().map(|it| it.len()).min().unwrap();
    containers.into_iter().filter(|it| it.len() == minimum_containers).count()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1638);
    assert_eq!(part2(input), 17);
}
