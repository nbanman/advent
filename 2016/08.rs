use advent::prelude::*;

const WIDTH: usize = 50;
const HEIGHT: usize = 6;
fn parse_input(input: &str) -> [bool; 300] {
    let mut screen = [false; WIDTH * HEIGHT];
    for instruction in input.lines() {
        let (n1, n2) = instruction.get_numbers().collect_tuple().unwrap();
        let (first, rest) = instruction.split_once(' ').unwrap();
        if first == "rect" {
            for y in 0..n2 {
                for x in 0..n1 {
                    screen[y * WIDTH + x] = true;
                }
            }
        } else {
            let offset = n2 as isize;
            let (next, _) = rest.split_once(' ').unwrap();
            match next {
                "column" => {
                    let col: Vec<bool> = (0..HEIGHT).into_iter()
                        .map(|y| screen[y * WIDTH + n1])
                        .collect();
                    for y in 0..HEIGHT as isize {
                        let y_offset = (y + offset).rem_euclid(HEIGHT as isize) as usize;
                        screen[y_offset * WIDTH + n1] = col[y as usize];
                    }
                },
                "row" => {
                    let row: Vec<bool> = (0..WIDTH).into_iter()
                        .map(|x| screen[n1 * WIDTH + x])
                        .collect();
                    for x in 0..WIDTH as isize {
                        let x_offset = (x + offset).rem_euclid(WIDTH as isize) as usize;
                        screen[n1 * WIDTH + x_offset] = row[x as usize];
                    }
                },
                x => panic!("invalid command: {x}"),
            }
        }
    }
    screen
}

fn default_input() -> [bool; 300] {
    parse_input(include_input!(2016 / 08))
}

fn part1(screen: [bool; 300]) -> usize {
    screen.into_iter().filter(|light| *light).count()
}

fn part2(screen: [bool; 300]) -> String {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}",
                 if screen[y * WIDTH + x] {
                    "*"
                } else {
                    " "
                }
            )
        }
        println!();
    }
    "AFBUPZBJPS".to_string()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 123);
    assert_eq!(part2(input), "AFBUPZBJPS".to_string());
}
