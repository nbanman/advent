use advent::prelude::*;

fn parse_input(input: &str) -> (&[u8], isize, usize) {
    let map = input.as_bytes();
    let width = map.iter().position(|&x| x == b'\n').unwrap() as isize + 1;
    let end = map.iter().position(|&x| x == b'E').unwrap();
    (map, width, end)
}

fn default_input() -> (&'static [u8], isize, usize) {
    parse_input(include_input!(2022 / 12))
}

/// For height, I just use the u8 value of the char, with two exceptions. 'S' has the value of 'a,'
/// and 'E' has the value of 'z'. Pass through except for those two.
fn height(spot: u8) -> u8 {
    match spot {
        b'S' => b'a',
        b'E' => b'z',
        spot => spot
    }
}

/// Uses BFS to find the shortest path. Starts at the end and works toward the start(s). This way
/// where there are multiple potential starts, it will quit early once the first start is found,
/// since BFS finds the lowest step solution first.
fn solve(map: &[u8], width: isize, end: usize, targets: &[u8]) -> usize {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back((end, 0));
    let mut visited: Vec<bool> = map.iter().map(|&pos| pos == b'\n').collect();
    while let Some((pos, steps)) = q.pop_front() {
        let neighbors: Vec<(usize, usize)> = neighbors(pos, map, width, &visited)
            .iter()
            .map(|&neighbor| {
                visited[neighbor] = true; // side effect - marks position as visited
                (neighbor, steps + 1)
            }).collect();
        for (neighbor_pos, neighbor_steps) in neighbors {
            if targets.contains(&map[neighbor_pos]) {
                return neighbor_steps;
            } else {
                q.push_back((neighbor_pos, neighbor_steps));
            }
        }
    }
    panic!("Path not found!")
}

fn neighbors(pos: usize, map: &[u8], width: isize, visited: &Vec<bool>) -> Vec<usize> {
    let pos_height = height(map[pos]) - 1;
    let pos = pos as isize;
    vec![-width, width, -1, 1]
        .into_iter()
        .filter_map(|x| {
            match map.try_get(pos + x) {
                None => None,
                Some(&c) => {
                    if visited[(pos + x) as usize] || height(c) < pos_height {
                        None
                    } else {
                        Some((pos + x) as usize)
                    }
                }
            }
        }).collect()
}

fn part1((map, width, end): (&[u8], isize, usize)) -> usize {
    solve(&map, width, end, &[b'S'])
}

fn part2((map, width, end): (&[u8], isize, usize)) -> usize {
    solve(&map, width, end, &[b'S', b'a'])
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    let input = parse_input(input);
    assert_eq!(part1(input), 31);
    assert_eq!(part2(input), 29);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 361);
    assert_eq!(part2(input), 354);
}
