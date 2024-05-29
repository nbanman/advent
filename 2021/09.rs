use advent::prelude::*;

fn parse_input(input: &str) -> (&[u8], Vec<isize>, Vec<usize>) {
    let input = input.as_bytes();
    let width = input.iter().position(|&x| x == b'\n').unwrap() as isize + 1;

    let start_vec = vec![-width, width, -1, 1];

    let low_indices = (0..input.len())
        .filter(|&pos| {
            let height = input.get(pos).unwrap();
            if height == &b'\n' {
                false
            } else {
                start_vec.iter()
                    .map(|m| pos as isize + m)
                    .filter_map(|neighbor_idx| {
                        input
                            .try_get(neighbor_idx)
                            .and_then(|neighbor_height| {
                                if neighbor_height == &b'\n' { None } else { Some(neighbor_height) }
                            })
                    })
                    .all(|neighbor_height| neighbor_height > height)
            }

        })
        .collect();
    (input, start_vec, low_indices)
}

fn default_input() -> (&'static [u8], Vec<isize>, Vec<usize>) {
    parse_input(include_input!(2021 / 09))
}

fn part1((height_map, _, low_indices): (&[u8], Vec<isize>, Vec<usize>)) -> usize {
    low_indices.into_iter()
        .map(|idx| (*height_map.get(idx).unwrap() + 1 - b'0') as usize)
        .sum()
}

fn part2((height_map, start_vec, low_indices): (&[u8], Vec<isize>, Vec<usize>)) -> usize {
    low_indices.into_iter()
        .map(|idx| {
            let mut q = VecDeque::new();
            let mut visited = HashSet::new();
            q.push_front(idx);
            visited.insert(idx);
            while !q.is_empty() {
                let current_idx = q.pop_back().unwrap();
                let current_height = height_map.get(current_idx).unwrap();
                let neighbors = start_vec
                    .iter()
                    .map(|m| current_idx as isize + m)
                    .filter(|&neighbor_idx| {
                        let neighbor_height = height_map
                            .try_get(neighbor_idx)
                            .and_then(|x| if x == &b'\n' { None } else { Some(x) });
                        if let Some(neighbor_height) = neighbor_height {
                            neighbor_height != &b'9' && neighbor_height > current_height
                        } else {
                            false
                        }
                    });
                for neighbor_idx in neighbors {
                    let neighbor_idx = neighbor_idx as usize;
                    if visited.insert(neighbor_idx) {
                        q.push_front(neighbor_idx);
                    }
                }
            }
            visited.len()
        })
        .sorted_by(|a, b| b.cmp(a))
        .take(3)
        .reduce(|acc, item| acc * item)
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
2199943210
3987894921
9856789892
8767896789
9899965678",
    );
    assert_eq!(part1(input.clone()), 15);
    assert_eq!(part2(input), 1134);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 448);
    assert_eq!(part2(input), 1417248);
}
