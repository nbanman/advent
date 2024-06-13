use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<usize, Vec<usize>> {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.get_numbers();
            (numbers.next().unwrap(), numbers.into_iter().collect())
        })
        .collect()
}

fn default_input() -> HashMap<usize, Vec<usize>> {
    parse_input(include_input!(2017 / 12))
}

fn all_links(start: usize, programs: &HashMap<usize, Vec<usize>>) -> HashSet<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(start);
    while let Some(current) = queue.pop_front() {
        visited.insert(current);
        if let Some(neighbors) = programs.get(&current) {
            for neighbor in neighbors {
                if !visited.contains(neighbor) { queue.push_back(*neighbor) }
            }
        }
    }
    visited
}

fn part1(programs: HashMap<usize, Vec<usize>>) -> usize {
    all_links(0, &programs).len()
}

fn remove_group(program_set: &mut HashSet<usize>, programs: &HashMap<usize, Vec<usize>>) {
    if let Some(&first) = program_set.iter().next() {
        let links = all_links(first, programs);
        program_set.retain(|x| !links.contains(x));
    }
}

fn part2(programs: HashMap<usize, Vec<usize>>) -> usize {
    let mut program_set: HashSet<usize> = programs.keys().cloned().collect();
    let mut count = 0;

    while !program_set.is_empty() {
        remove_group(&mut program_set, &programs);
        count += 1;
    }

    count
}


fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 115);
    assert_eq!(part2(input), 221);
}
