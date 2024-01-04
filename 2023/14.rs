use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.trim_end().lines().map(|line| line.chars().collect()).collect()
}

fn default_input() -> Vec<Vec<char>> {
    parse_input(include_input!(2023 / 14))
}

fn part1(initial: Vec<Vec<char>>) -> usize {
    load(&tilt_up(&initial))
}

fn part2(initial: Vec<Vec<char>>) -> usize {
    let mut rock_formations = HashMap::new();
    let mut rocks = initial;
    let mut index = 0_usize;
    let first_index_of_cycle = loop {
        if let Some(first_index_of_cycle) = rock_formations.insert(rocks.clone(), index) {
            break first_index_of_cycle
        }
        rocks = spin_cycle(&rocks);
        index += 1;
    };
    let cycle_space = 1_000_000_000 - first_index_of_cycle;
    let cycle_length = rock_formations.len() - first_index_of_cycle;
    let answer = rock_formations.iter()
        .find(|(_, v)| {
            **v == first_index_of_cycle + cycle_space % cycle_length
        }).unwrap().0;
    load(answer)
}

fn tilt_up(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted = vec![vec!['.'; rocks.len()]; rocks.len()];

    rocks.into_iter().enumerate().for_each(|(y, row)| {
        row.into_iter().enumerate().for_each(|(x, c)| {
            match c {
                '#' => tilted[y][x] = '#',
                'O' => {
                    for yy in (0..=y).rev() {
                        let next = yy as isize - 1;
                        if next < 0 || "#O".contains(tilted[next as usize][x]) {
                            tilted[yy][x] = 'O';
                            break
                        }
                    }
                },
                _ => { },
            }
        })
    });
    tilted
}

fn spin_cycle(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    (1..=4).fold(rocks.clone(), |acc, _| {
        rotate(tilt_up(&acc))
    })
}

fn rotate(rocks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated = vec![vec!['.'; rocks.len()]; rocks.len()];
    rocks.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, c)| {
            if *c != '.' {
                rotated[x][rocks.len() - 1 - y] = c.to_owned();
            }
        })
    });
    rotated
}

fn load(rocks: &Vec<Vec<char>>) -> usize {
    rocks.iter().enumerate()
        .map(|(index, row)| {
            (rocks.len() - index) * row.iter().filter(|c| **c == 'O').count()
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";
    assert_eq!(part1(parse_input(input)), 136);
    assert_eq!(part2(parse_input(input)), 64);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 106990);
    assert_eq!(part2(input), 100531);
}
