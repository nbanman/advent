use advent::prelude::*;

fn parse_input(input: &'static str) -> HashMap<&'static str, HashMap<&'static str, i64>> {
    let mut arrangements = HashMap::new();
    let pattern =
        regex!(r"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).");
    for captures in pattern.captures_iter(input) {
        let groups: Vec<&str> = captures.iter().skip(1)
            .map(|m| m.unwrap().as_str())
            .collect();
        let (p1, gain_or_lose, units, p2) =
            (groups[0], groups[1], groups[2], groups[3]);
        let mut happiness_units: i64 = units.parse().unwrap();
        if gain_or_lose == "lose" {
            happiness_units *= -1;
        }
        let entry = arrangements.entry(p1).or_insert(HashMap::new());
        (*entry).insert(p2, happiness_units);
    }
    arrangements
}
fn get_permutations(elements: Vec<&'static str>) -> Vec<Vec<&'static str>> {
    let length = elements.len();
    let mut combos: Vec<Vec<&str>> = elements
        .iter()
        .map(|person| vec![*person])
        .collect();
    for _ in 1..length {
        combos = combos
            .iter()
            .flat_map(|combo| {
                elements.clone()
                    .into_iter()
                    .filter_map(|person| {
                        if combo.contains(&person) {
                            None
                        } else {
                            let mut new_combo = combo.clone();
                            new_combo.push(person);
                            Some(new_combo)
                        }
                    })
            })
            .collect();
    }
    combos
}
fn default_input() -> HashMap<&'static str, HashMap<&'static str, i64>> {
    parse_input(include_input!(2015 / 13))
}

fn solve(
    arrangements: HashMap<&'static str, HashMap<&'static str, i64>>,
    mut people: Vec<&'static str>
) -> i64 {
    let first = people.pop().expect("people is empty");
    let mut permutations = get_permutations(people);

    for combo in permutations.iter_mut() {
        combo.push(&first)
    }
    permutations.into_iter()
        .map(|combo| {
            let last = *combo.first().unwrap();
            let mut pairs: Vec<(_, _)> = combo.into_iter().tuple_windows().collect();
            pairs.push((&first, last));
            pairs.into_iter()
                .map(|(left, right)| {
                    if let Some(left_to_right) = arrangements
                        .get(left)
                        .and_then(|it| it.get(right)) {

                        let right_to_left = arrangements
                            .get(right)
                            .and_then(|it| it.get(left))
                            .expect("name not found");
                        let total = left_to_right + right_to_left;
                        total
                    } else {
                        0
                    }
                })
                .sum()
        })
        .max()
        .expect("permutations is empty")
}

fn part1(input: HashMap<&'static str, HashMap<&'static str, i64>>) -> i64 {
    let people: Vec<_> = input.keys().cloned().collect();
    solve(input, people)
}

fn part2(input: HashMap<&'static str, HashMap<&'static str, i64>>) -> i64 {
    let mut people: Vec<_> = input.keys().cloned().collect();
    people.push("me");
    solve(input, people)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 664);
    assert_eq!(part2(input), 640);
}

#[test]
fn example1() {
    let input = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.
";
    assert_eq!(part1(parse_input(input)), 330);
}