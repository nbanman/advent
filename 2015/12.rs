use advent::prelude::*;

const DIVIDERS: [u8; 4] = [b'[', b']', b'{', b'}'];

fn default_input() -> &'static str {
    include_input!(2015 / 12).trim_end()
}

fn part1(input: &str) -> i64 {
    input.get_numbers::<i64>().sum()
}

fn part2(input: &str) -> i64 {
    next_block(input.as_bytes(), 0, false).0
}

fn next_block(input: &[u8], start: usize, already_red: bool) -> (i64, usize) {
    let mut index = start;

    if already_red {
        let mut depth = 0;
        while depth >= 0 {
            index += 1;
            match input[index] {
                b'[' | b'{' => depth += 1,
                b']' | b'}' => depth -= 1,
                _ => {}
            }
        }
        (0, index)
    } else {
        let mut is_red = false;
        let mut value = 0;
        loop {
            let end_index = input[index + 1..]
                .iter()
                .enumerate()
                .find(|(_, c)| DIVIDERS.contains(*c))
                .map(|(pos, _)| pos + index + 1)
                .expect("No closing bracket found!");
            let snippet = std::str::from_utf8(&input[index..end_index])
                .unwrap();

            if !is_red {
                if snippet.contains(r#":"red""#) {
                    is_red = true;
                    value = 0;
                } else {
                    value += snippet.get_numbers::<i64>().sum::<i64>()
                }
            }

            if input[end_index] == b']' || input[end_index] == b'}' {
                return (value, end_index)
            } else {
                let (inner_value, inner_end) = next_block(input, end_index, is_red);
                value += inner_value;
                index = inner_end;
            }
        }
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 111754);
    assert_eq!(part2(input), 65402);
}

#[test]
fn example1() {
    let input = "[1,2,3]";
    assert_eq!(part2(input), 6);
}

#[test]
fn example2() {
    let input = r#"[1,{"c":"red","b":2},3]"#;
    assert_eq!(part2(input), 4);
}

#[test]
fn example3() {
    let input = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;
    assert_eq!(part2(input), 0);
}

#[test]
fn example4() {
    let input = r#"[1,"red",5]"#;
    assert_eq!(part2(input), 6);
}