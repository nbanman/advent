use advent::prelude::*;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

fn parse_input(input: &str) -> Graph<'_> {
    input
        .lines()
        .map(|line| line.split_once('-').unwrap())
        .fold(HashMap::new(), |mut graph, (a, b)| {
            graph.entry(a).or_default().push(b);
            if a != "start" && b != "end" { graph.entry(b).or_default().push(a) }
            graph
        })
}

fn default_input() -> Graph<'static> {
    parse_input(include_input!(2021 / 12))
}

#[derive(Eq, PartialEq)]
struct State<'a> {
    visits: Vec<&'a str>,
    visited_twice: bool,
}

fn solve(edges: &Graph<'_>, allow_one_extra: bool) -> usize {
    let mut count = 0;
    let mut q = VecDeque::new();
    q.push_back(State { visits: vec!["start"], visited_twice: false });

    while let Some(state) = q.pop_front() {
        if state.visits.last() == Some(&"end") {
            count += 1;
        } else {
            if let Some(neighbors) = edges.get(state.visits.last().unwrap()) {
                for &name in neighbors {
                    if name.as_bytes()[0].is_ascii_uppercase() || !state.visits.contains(&name) {
                        let mut new_visits = state.visits.clone();
                        new_visits.push(name);
                        let next_state = State {
                            visits: new_visits,
                            visited_twice: state.visited_twice,
                        };
                        q.push_back(next_state);
                    } else if allow_one_extra && !state.visited_twice {
                        let mut new_visits = state.visits.clone();
                        new_visits.push(name);
                        let next_state = State {
                            visits: new_visits,
                            visited_twice: true,
                        };
                        q.push_back(next_state);
                    }
                }
            }
        }
    }
    count
}

fn part1(graph: Graph<'_>) -> usize {
    solve(&graph, false)
}

fn part2(graph: Graph<'_>) -> usize {
    solve(&graph, true)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = parse_input(
        "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end",
    );
    assert_eq!(part1(input.clone()), 10);
    assert_eq!(part2(input), 36);
}

#[test]
fn example2() {
    let input = parse_input(
        "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc",
    );
    assert_eq!(part1(input.clone()), 19);
    assert_eq!(part2(input), 103);
}

#[test]
fn example3() {
    let input = parse_input(
        "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW",
    );
    assert_eq!(part1(input.clone()), 226);
    assert_eq!(part2(input), 3509);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 4104);
    assert_eq!(part2(input), 119760);
}
