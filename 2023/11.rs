use advent::prelude::*;

#[derive(Debug, Clone)]
struct Image {
    x_expansions: Vec<usize>,
    y_expansions: Vec<usize>,
    x_galaxies: Vec<(usize, usize)>,
    y_galaxies: Vec<(usize, usize)>,
}

fn parse_input(data: &str) -> Image {
    let width = data.find('\n').unwrap() + 1;
    let height = data.len() / width;
    let x_expansion: Vec<usize> = (0..width - 1).into_iter()
        .filter(|x| {
            (0..height).into_iter().all(|y| data.chars().nth(*x + y * width).unwrap() == '.')
        })
        .collect();
    let y_expansion: Vec<usize> = (0..height).into_iter()
        .filter(|y| {
            (0..width - 1).into_iter().all(|x| {
                data.chars().nth(x + *y * width).unwrap() == '.'
            })
        })
        .collect();

    let y_galaxies: Vec<(usize, usize)> = (0..height).into_iter()
        .map(|y| {
            let count = &data[y * width .. y * width + width].chars()
                .filter(|c| *c == '#')
                .count();
            (y, *count)
        })
        .filter(|(_, count)| *count > 0)
        .collect();

    let x_galaxies: Vec<(usize, usize)> = (0..width).into_iter()
        .map(|x| {
            let count = (0..height).map(|y| data.chars().nth(x + y * width).unwrap())
                .filter(|c| *c == '#')
                .count();
            (x, count)
        })
        .filter(|(_, count)| *count > 0)
        .collect();

    Image { x_expansions: x_expansion, y_expansions: y_expansion, x_galaxies, y_galaxies }
}

fn default_input() -> Image { parse_input(include_input!(2023 / 11))
}

fn distance(
    expansion_factor: usize,
    galaxies: &Vec<(usize, usize)>,
    expansions: &Vec<usize>
) -> usize {
    galaxies.iter().enumerate()
        .map(|(i, (a_pos, a_count))| {
            let already_passed = expansions.binary_search(a_pos).err().unwrap();
            galaxies.iter().dropping(i + 1)
                .map(|(b_pos, b_count)| {
                    let expansions_passed = expansions.binary_search(b_pos).err().unwrap()
                        - already_passed;
                    ((b_pos - a_pos) + expansions_passed * (expansion_factor - 1)) * a_count * b_count
                })
                .sum::<usize>()
        })
        .sum()
}

fn solve(image: &Image, expansion_factor: usize) -> usize {
    distance(expansion_factor, &image.x_galaxies, &image.x_expansions) +
        distance(expansion_factor, &image.y_galaxies, &image.y_expansions)
}
fn part1(image: Image) -> usize {
    solve(&image, 2)
}

fn part2(image: Image) -> usize {
    solve(&image, 1_000_000)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";
    assert_eq!(part1(parse_input(input)), 374);
}
#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 9545480);
    assert_eq!(part2(input), 406725732046);
}
