use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(usize, isize)> {
    input
        .trim_end()
        .split(['\n', ' '])
        .map(|word| {
            match word {
                "addx" | "noop" => 0,
                d => d.parse::<isize>().unwrap()
            }
        })
        .scan(1isize, |state, x| {
            let old_state = state.clone();
            *state += x;
            Some(old_state)
        })
        .enumerate()
        .collect()
}

fn default_input() -> Vec<(usize, isize)> {
    parse_input(include_input!(2022 / 10))
}
fn part1(cpu: Vec<(usize, isize)>) -> isize {
    cpu.into_iter()
        .filter(|(cycle, _)| (cycle + 19) % 40 == 0)
        .map(|(cycle, register)| (cycle + 1) as isize * register)
        .sum()
}

fn part2(cpu: Vec<(usize, isize)>) -> String {
    let mut s = String::new();
    cpu.into_iter()
        .take(240)
        .map(|(cycle, register)| {
            ((register - 1)..=(register + 1)).contains(&(cycle as isize % 40))
        })
        .enumerate()
        .for_each(|(index, on)| {
            if index != 0 && index % 40 == 0 {
                s.push('\n');
            }
            if on {
                s.push('*');
            } else {
                s.push(' ');
            }
        });
    s
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop",
    );
    assert_eq!(part1(input.clone()), 11884);
    assert_eq!(
        trim_ends(part2(input)),
        "⚪⚪⚫⚫⚪⚪⚫⚫⚪⚪⚫⚫⚪⚪⚫⚫⚪⚪⚫⚫⚪⚪⚫⚫⚪⚪⚫⚫⚪⚪⚫⚫⚪⚪⚫⚫⚪⚪⚫⚫
⚪⚪⚪⚫⚫⚫⚪⚪⚪⚫⚫⚫⚪⚪⚪⚫⚫⚫⚪⚪⚪⚫⚫⚫⚪⚪⚪⚫⚫⚫⚪⚪⚪⚫⚫⚫⚪⚪⚪⚫
⚪⚪⚪⚪⚫⚫⚫⚫⚪⚪⚪⚪⚫⚫⚫⚫⚪⚪⚪⚪⚫⚫⚫⚫⚪⚪⚪⚪⚫⚫⚫⚫⚪⚪⚪⚪⚫⚫⚫⚫
⚪⚪⚪⚪⚪⚫⚫⚫⚫⚫⚪⚪⚪⚪⚪⚫⚫⚫⚫⚫⚪⚪⚪⚪⚪⚫⚫⚫⚫⚫⚪⚪⚪⚪⚪⚫⚫⚫⚫⚫
⚪⚪⚪⚪⚪⚪⚫⚫⚫⚫⚫⚫⚪⚪⚪⚪⚪⚪⚫⚫⚫⚫⚫⚫⚪⚪⚪⚪⚪⚪⚫⚫⚫⚫⚫⚫⚪⚪⚪⚪
⚪⚪⚪⚪⚪⚪⚪⚫⚫⚫⚫⚫⚫⚫⚪⚪⚪⚪⚪⚪⚪⚫⚫⚫⚫⚫⚫⚫⚪⚪⚪⚪⚪⚪⚪⚫⚫⚫⚫⚫"
    );
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 16406);
    assert_eq!(
        trim_ends(part2(input)),
        "**** *  *   ** **** ***    ** **** ****
   * * *     * *    *  *    * *       *
  *  **      * ***  ***     * ***    *
 *   * *     * *    *  *    * *     *
*    * *  *  * *    *  * *  * *    *
**** *  *  **  *    ***   **  *    ****"
    );
}

#[cfg(test)]
fn trim_ends(s: String) -> String {
    s.lines().map(|line| line.trim_end()).join("\n")
}
