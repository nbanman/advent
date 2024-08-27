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
    id: isize,
    weight: isize,
    h: isize,
}

impl State
{
    fn f(&self) -> isize {
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
    let end_pos = cavern.len() as isize - 1;

    let width = width as isize;

    let heuristic = |pos: isize| -> isize {
        let x1 = pos % width;
        let y1 = pos / width;
        let x2 = end_pos % width;
        let y2 = end_pos / width;
        (x1 - x2).abs() + (y1 - y2).abs()
    };

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
        if closed[current.id as usize] == true { continue; } else { closed[current.id as usize] = true }
        if current.id == end_pos { return Some(current.weight as usize); }

        // get edges
        let edges = directions.iter()
            .enumerate()
            .filter_map(|(idx, &direction)| {
                let pos = current.id + direction;
                if pos < 0 || pos >= cavern.len() as isize {
                    None
                } else {
                    // horrific kludge because I got rid of line breaks and thus don't know when
                    // something has gone out of range on the x-axis. So I get the value of pos on
                    // the x-axis. If going left puts me at the far right, or going right puts me
                    // at the far left, I return None to filter it out.
                    let x = pos % width;
                    if idx == 2 && x == width - 1 {
                        None
                    } else if idx == 3 && x == 0 {
                        None
                    } else {
                        Some(pos)
                    }
                }
            })
            .filter(|&new_pos| !closed[new_pos as usize])
            .map(|new_pos| {
                State {
                    id: new_pos,
                    weight: *cavern.get(new_pos as usize).unwrap() as isize + current.weight,
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
