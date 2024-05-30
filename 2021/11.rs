use advent::prelude::*;

fn parse_input(input: &str) -> (Vec<u8>, isize) {
    let grid: Vec<u8> = input
        .as_bytes()
        .iter()
        .map(|c| {
            if c != &b'\n' {
                c - 48
            } else {
                u8::MAX
            }
        })
        .collect();
    let width = (grid.iter().position(|c| c == &u8::MAX).unwrap() + 1) as isize;
    (grid, width)
}

fn default_input() -> (Vec<u8>, isize) {
    parse_input(include_input!(2021 / 11))
}

fn flash(cave: &mut Vec<u8>, width: isize) -> usize {

    let vecs = vec![-width - 1, -width, -width + 1, -1, 1, width - 1, width, width + 1];

    for cell in cave.iter_mut() {
        if cell != &u8::MAX {
            *cell += 1;
        }
    }
    loop {
        let flasher_indices: Vec<_> = cave.iter().enumerate()
            .filter_map(|(idx, &energy)| {
                if energy != u8::MAX && energy > 9 {
                    Some(idx)
                } else {
                    None
                }
            })
            .collect();
        if flasher_indices.is_empty() { break }

        for &idx in flasher_indices.iter() {
            if let Some(flasher) = cave.get_mut(idx) {
                *flasher = 0
            }
        }

        let neighbors: Vec<_> = flasher_indices.iter()
            .flat_map(|idx| {
                vecs.iter()
                    .filter_map(|pos| {
                        let neighbor_index = pos + idx.clone() as isize;
                        cave
                            .try_get(neighbor_index)
                            .and_then(|neighbor_energy| if neighbor_energy == &u8::MAX {
                                None
                            } else {
                                Some(neighbor_index as usize)
                            })
                    })
            })
            .filter(|&neighbor_index| {
                if let Some(&neighbor_energy) = cave.get(neighbor_index) {
                    if neighbor_energy != 0 { true } else { false }
                } else {
                    false
                }
            })
            .collect();

        for neighbor_index in neighbors {
            if let Some(to_update) = cave.get_mut(neighbor_index) {
                *to_update += 1;
            }
        }
    }
    cave.iter().filter(|&&energy| energy == 0).count()
}

fn part1((mut cave, width): (Vec<u8>, isize)) -> usize {
    iter::repeat_with(|| flash(&mut cave, width)).take(100).sum()
}

fn part2((mut cave, width): (Vec<u8>, isize)) -> usize {
    let cave_size = cave.len();
    1 + iter::repeat_with(|| flash(&mut cave, width)).enumerate()
        .find_or_first(|(idx, flashed)| {
            flashed == &cave_size || idx == &194
        })
        .unwrap()
        .0
}
fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526",
    );
    assert_eq!(part1(input.clone()), 1656);
    assert_eq!(part2(input), 195);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1669);
    assert_eq!(part2(input), 351);
}

// Part 1:                             (377.0 µs)
// 1669
//
// Part 2:                             (1.335 ms)
// 351