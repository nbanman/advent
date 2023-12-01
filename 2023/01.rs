use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2023 / 01)
}

fn part1(calibration: &str) -> u32 {
    calibration
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();
            let second = line
                .chars()
                .rfind(|c| c.is_digit(10))
                .unwrap()
                .to_digit(10)
                .unwrap();
            first * 10 + second
        }).sum()
}

fn part2(calibration: &str) -> u32 {
    let pattern = regex!(r"(\d|one|two|three|four|five|six|seven|eight|nine)");
    let reverse_pattern = regex!(r"(\d|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)");
    calibration
        .lines()
        .map(|line| {
            let first = pattern
                .captures(line)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str();
            let reversed_line = line.chars().rev().collect::<String>();
            // println!("{}", reversed_line);
            
            let second = reverse_pattern
                .captures(&reversed_line)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str();
            let value = str_to_u32(first) * 10 + str_to_u32(second);
            println!("line: {}, first: {}, second: {}, value: {}", line, first, second, value);
            value
        }).sum()
}

fn str_to_u32(s: &str) -> u32 {
    match s {
        "one" | "eno" => 1,
        "two" | "owt" => 2,
        "three" | "eerht" => 3,
        "four" | "ruof" => 4,
        "five" | "evif" => 5,
        "six" | "xis" => 6,
        "seven" | "neves" => 7,
        "eight" | "thgie" => 8,
        "nine" | "enin" => 9,
        _ => s.chars().next().unwrap().to_digit(10).unwrap(),
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(part1(input), 142);
    assert_eq!(part2(input), 142);
}

#[test]
fn example2() {
    let input = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
    assert_eq!(part2(input), 281);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 54388);
    assert_eq!(part2(input), 53515);
}
