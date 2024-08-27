use advent::prelude::*;

type Bags<'a> = HashMap<&'a str, Vec<(&'a str, usize)>>;

fn parse_input(input: &str) -> Bags<'_> {
    let bag_rx = regex!(r"(?P<amount>\d+) (?P<bag>\w+ \w+) bag");

    regex!(r"(?P<container>\w+ \w+) bags contain (?P<contained>[^.]+)\.")
        .captures_iter(input)
        .map(|caps| {
            let container = caps.name("container").unwrap().as_str();
            let contained = caps.name("contained").unwrap().as_str();
            let held_bags: Vec<_> = bag_rx.captures_iter(contained)
                .map(|contained_caps| {
                    let amount: usize = contained_caps.name("amount").unwrap().as_str().parse().unwrap();
                    let bag = contained_caps.name("bag").unwrap().as_str();
                    (bag, amount)
                })
                .collect();
            (container, held_bags)
        })
        .collect()
}

fn default_input() -> Bags<'static> {
    parse_input(include_input!(2020 / 07))
}


fn part1(bags: Bags<'_>) -> usize {
    bags.keys()
        .filter(|&&bag| contains_gold(bag, &bags))
        .count() - 1
}

fn contains_gold(bag: &str, bags: &Bags<'_>) -> bool {
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back(bag);
    while let Some(next_bag) = q.pop_front() {
        if next_bag == "shiny gold" { return true; }
        visited.insert(next_bag);
        bags[next_bag].iter()
            .filter(|(held_bag, _)| !visited.contains(held_bag))
            .for_each(|(held_bag, _)| q.push_back(held_bag));
    }
    false
}

fn part2(bags: Bags<'_>) -> usize {
    bags_inside(&bags, "shiny gold")
}

fn bags_inside(bags: &Bags<'_>, bag: &str) -> usize {
    bags[bag]
        .iter()
        .map(|(held_bag, amt)| amt * (1 + bags_inside(bags, held_bag)))
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
"#,
    );
    assert_eq!(part1(input.clone()), 4);
    assert_eq!(part2(input), 32);
}

#[test]
fn example2() {
    let input = parse_input(
        r#"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
"#,
    );
    assert_eq!(part1(input.clone()), 0);
    assert_eq!(part2(input), 126);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 252);
    assert_eq!(part2(input), 35487);
}
