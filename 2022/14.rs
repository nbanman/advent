use advent::prelude::*;

fn parse_input(input: &str) -> (HashSet<Vector2>, i64) {
    let cavern: HashSet<Vector2> = input
        .lines()
        .flat_map(|line| {
            line
                .split(" -> ")
                .filter_map(|pt_str| {
                    pt_str
                        .split(',')
                        .filter_map(|xy| xy.parse().ok())
                        .next_array()
                        .map(Vector2::from)
                })
                .array_windows()
                .flat_map(|[p1, p2]| {
                    let diff = (p2 - p1).map(i64::signum);
                    iter::successors(Some(p1), move |&p| (p != p2).some(p + diff))
                })
        })
        .collect();

    let depth = cavern.iter().map(|v| v.y).max().unwrap();

    (cavern, depth)
}

fn default_input() -> (HashSet<Vector2>, i64) {
    parse_input(include_input!(2022 / 14))
}

fn solve<F>(cavern: &mut HashSet<Vector2>, depth: i64, predicate: F) -> usize
    where
        F: Fn(Vector2) -> bool
{
    let mut index = 0usize;
    let options: Vec<Vector2> = vec![vector![0, 1], vector![-1, 1], vector![1, 1]];
    loop {
        let grain = settle(cavern, depth, &options);
        if predicate(grain) { return index; }
        cavern.insert(grain);
        index += 1;
    }
}

fn fall(cavern: &HashSet<Vector2>, grain: &Vector2, options: &Vec<Vector2>) -> Option<Vector2> {
    options
        .into_iter()
        .map(|v| v + grain)
        .find(|v| !cavern.contains(v))
}

fn settle(cavern: &HashSet<Vector2>, depth: i64, options: &Vec<Vector2>) -> Vector2 {
    iter::successors(Some(vector![500, 0]), |grain| fall(cavern, grain, options))
        .take_while(|grain| grain.y <= depth + 1)
        .last()
        .unwrap()
}

fn part1((mut cavern, depth): (HashSet<Vector2>, i64)) -> usize {
    solve(&mut cavern, depth, |v| v.y > depth)
}

fn part2((mut cavern, depth): (HashSet<Vector2>, i64)) -> usize {
    let top = vector!(500, 0);
    solve(&mut cavern, depth, |v| v == top) + 1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9",
    );
    assert_eq!(part1(input.clone()), 24);
    assert_eq!(part2(input), 93);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 825);
    assert_eq!(part2(input), 26729);
}
