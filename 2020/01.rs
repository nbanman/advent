use advent::prelude::*;

fn default_input() -> (Vec<i64>, HashSet<i64>) {
    let entries = get_numbers(include_input!(2020 / 01));
    let entry_set: HashSet<_> = entries.iter().map(|n| *n).collect();
    (entries, entry_set)
}

fn part1((_, entry_set): (Vec<i64>, HashSet<i64>)) -> i64 {
    for entry in entry_set.iter() {
        let complement = 2020 - entry;
        if entry_set.contains(&complement) {
            return entry * complement
        }
    }
    -1
}

fn part2((entries, entry_set): (Vec<i64>, HashSet<i64>)) -> i64 {
    for (first, second) in entries.iter().dropping_back(1)
        .cartesian_product(entries.iter().dropping(1)) {

        let complement = 2020 - first - second;
        if entry_set.contains(&complement) {
            return first * second * complement
        }
    }
    -1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

/*#[test]
fn example() {
    let input = vec![1721, 979, 366, 299, 675, 1456];
    assert_eq!(part1(input.clone()), 514579);
    assert_eq!(part2(input), 241861950);
}*/

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1015476);
    assert_eq!(part2(input), 200878544);
}
