use advent::prelude::*;

fn parse_input(input: &str) -> (Vec<u8>, usize) {
    let input = input.as_bytes();
    let width = input.iter().position(|&c| c == b'\n').unwrap();
    let input = input.iter()
        .filter(|&c| c != &b'\n')
        .map(|c| c - b'0')
        .collect();
    (input, width)
}

fn default_input() -> (Vec<u8>, usize) {
    parse_input(include_input!(2021 / 15))
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    id: usize,
    weight: usize,
    h: usize,
}
impl State
{
    fn f(&self) -> usize {
        self.h + self.weight
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f().cmp(&self.f())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


fn solve(cavern: Vec<u8>, width: usize) -> Option<usize> {
    let end_pos = cavern.len() - 1;

    let heuristic = |pos: usize| -> usize {
        let x1 = (pos % width) as isize;
        let y1 = (pos / width) as isize;
        let x2 = (end_pos % width) as isize;
        let y2 = (end_pos / width) as isize;
        ((x1 - x2).abs() + (y1 - y2).abs()) as usize
    };

    let width = width as isize;

    let directions = vec![-width, width, -1, 1];

    let start = State {
        id: 0,
        weight: 0,
        h: heuristic(0),
    };

    let mut open = BinaryHeap::new();
    open.push(start);

    let mut closed = vec![false; cavern.len()];

    while let Some(current) = open.pop() {
        if closed[current.id] == true { continue } else { closed[current.id] = true }
        if current.id == end_pos { return Some(current.weight) }

        // get edges
        let edges = directions.iter()
            .filter_map(|&direction| {
                let pos = current.id;// as isize;
                pos.checked_add(direction as usize).map(|pos| pos as usize)
            })
            .filter(|&new_pos| new_pos < cavern.len() && !closed[new_pos])
            .map(|new_pos| {
                State {
                    id: new_pos,
                    weight: *cavern.get(new_pos).unwrap() as usize + current.weight,
                    h: heuristic(new_pos),
                }
            });

        for edge in edges {
            open.push(edge)
        }
    }
    None
}

fn part1((cavern, width): (Vec<u8>, usize)) -> usize {
    solve(cavern, width).unwrap()
}

fn part2((cavern, width): (Vec<u8>, usize)) -> usize {
    fn add_risk(risk: u8, n: u8) -> u8 {
        (risk + n - 1) % 9 + 1
    }

    let height = cavern.len() / width;
    let ex_width = width * 5;
    let ex_height = height * 5;

    let cavern: Vec<u8> = (0..ex_height).cartesian_product(0..ex_width)
        .map(|(y, x)| {
            let x_base = x % width;
            let y_base = y % height;
            let base_index = y_base * width + x_base;

            let added_risk = x / width + y / height;
            add_risk(*cavern.get(base_index).unwrap(), added_risk as u8)
        })
        .collect();
    solve(cavern, ex_width).unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581",
    );
    assert_eq!(part1(input.clone()), 40);
    assert_eq!(part2(input), 315);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 602);
    assert_eq!(part2(input), 2935);
}
