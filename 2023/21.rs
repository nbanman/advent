use advent::prelude::*;

fn parse_input(garden: &str) -> (usize, Vec<u8>) {
    let garden_slice = garden.as_bytes();
    let width = garden.find('\n').unwrap();
    let start = garden.find('S').unwrap();
    let mut q = VecDeque::new();
    q.push_back((start, 0u8));
    let mut visited = vec![false; garden.len()];
    visited[start] = true;
    let mut step_count = Vec::new();

    // BFS finds neighbors and runs until the queue is empty, meaning that no more neighbors are found
    // due to everything already being visited.
    while let Some((pos, steps)) = q.pop_front() {
        step_count.push(steps);
        let pos = pos as isize;
        let width = width as isize;
        let neighbors: Vec<_> = vec![pos - (width + 1), pos + 1, pos - 1, pos + (width + 1)]
            .into_iter()
            .filter(|neighbor| {
                if let Ok(neighbor) = usize::try_from(*neighbor) {
                    if let Some(c) = garden_slice.get(neighbor) {
                        "#\n".find(*c as char) == None && !visited[neighbor]
                    } else {
                        false
                    }
                } else {
                    false
                }
            }).collect();
            neighbors.into_iter().for_each(|neighbor| {
                visited[neighbor as usize] = true;
                q.push_back((neighbor as usize, steps + 1));
            });
    }
    (width, step_count)
}

fn default_input() -> (usize, Vec<u8>) {
    parse_input(include_input!(2023 / 21))
}

fn part1((_, garden_path): (usize, Vec<u8>)) -> usize {
    garden_path.iter().filter(|steps| steps <= &&64u8 && *steps & 1 == 0).count()
}

fn part2((width, garden_path): (usize, Vec<u8>)) -> usize {
    let (even_path, odd_path): (Vec<_>, Vec<_>) = garden_path.into_iter()
        .partition(|it| *it & 1 == 0);
    let even_corners = even_path.iter().filter(|it| it > &&65).count();
    let odd_corners = odd_path.iter().filter(|it| it > &&65).count();
    let n = (26501365 - width / 2) / width;
    (n + 1) * (n + 1) * odd_path.len() + n * n * even_path.len() - (n + 1) * odd_corners + n * even_corners
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 3782);
    assert_eq!(part2(input), 630661863455116);
}
