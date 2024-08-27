use advent::prelude::*;

fn parse_input(input: &str) -> &[u8] {
    input.as_bytes()
}

fn default_input() -> &'static [u8] {
    parse_input(include_input!(2022 / 08))
}

fn safe_subtract(num: &usize, subtracted: &usize) -> Option<usize> {
    if subtracted <= num {
        Some(num - subtracted)
    } else {
        None
    }
}

fn move_one(tree_heights: &[u8], pos: &usize, direction: &Direction, width: usize) -> Option<usize> {
    match direction {
        Direction::N => { safe_subtract(pos, &(width + 1)) }
        Direction::E => {
            if *pos == 0 {
                None
            } else {
                if tree_heights[pos - 1] == 10 {
                    None
                } else {
                    Some(pos - 1)
                }
            }
        }
        Direction::S => {
            if tree_heights.len() > pos + 1 + width {
                Some(pos + 1 + width)
            } else {
                None
            }
        }
        Direction::W => {
            if tree_heights[pos + 1] == 10 {
                None
            } else {
                Some(pos + 1)
            }
        }
    }
}

fn is_visible(tree_heights: &[u8], pos: &usize, width: usize) -> bool {
    if tree_heights[*pos] == 10 { return false; }
    let rays = vec![Direction::N, Direction::S, Direction::E, Direction::W];
    rays.iter().any(|direction| {
        let (_, got_out) = ray(tree_heights, pos, direction, width);
        // dbg!(direction, got_out);
        got_out
    })
}

fn ray(tree_heights: &[u8], pos: &usize, direction: &Direction, width: usize) -> (usize, bool) {
    let mut visible_trees = 0;
    let mut pos = *pos;
    let treehouse_height = tree_heights[pos];
    while let Some(moved) = move_one(tree_heights, &pos, direction, width) {
        visible_trees += 1;
        if tree_heights[moved] >= treehouse_height {
            return (visible_trees, false);
        }
        pos = moved;
    }
    (visible_trees, true)
}

fn part1(tree_heights: &[u8]) -> usize {
    let width = tree_heights.iter().position(|x| x == &10).unwrap();
    (0..tree_heights.len()).into_iter()
        .filter(|pos| {
            let is_vis = is_visible(tree_heights, pos, width);
            // println!("pos: {pos}, visible: {is_vis}");
            is_vis
        })
        .count()
}

fn part2(tree_heights: &[u8]) -> usize {
    8
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "30373
25512
65332
33549
35390",
    );
    assert_eq!(part1(input), 21);
    assert_eq!(part2(input), 8);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), 1845);
    assert_eq!(part2(input), 230112);
}
