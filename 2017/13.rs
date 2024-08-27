use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .get_numbers::<usize>()
        .array_chunked()
        .map(|[depth, range]| (depth, range - 1))
        .collect()
}

fn default_input() -> Vec<(usize, usize)> {
    parse_input(include_input!(2017 / 13))
}

fn severity(depth: usize, range: usize) -> usize {
    if depth % (range * 2) == 0 {
        depth * (range + 1)
    } else {
        0
    }
}

fn is_triggered(depth: &usize, range: &usize, offset: &usize) -> bool {
    (depth + offset) % (range * 2) == 0
}


fn part1(layers: Vec<(usize, usize)>) -> usize {
    layers.into_iter().map(|(depth, range)| severity(depth, range)).sum()
}

fn part2(layers: Vec<(usize, usize)>) -> usize {
    let mut offset = 0usize;
    loop {
        if !layers.iter().any(|(depth, range)| is_triggered(depth, range, &offset)) {
            return offset;
        }
        offset += 1;
    }
}


fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1528);
    assert_eq!(part2(input), 3896406);
}
