use advent::prelude::*;

fn parse_input(input: &'static str) -> Vec<&'static str> { input.trim_end().split(',').collect() }

fn default_input() -> Vec<&'static str> {
    parse_input(include_input!(2023 / 15))
}

fn hash_of(step: &str) -> usize {
    step.chars().fold(0, |acc, c| (acc + c as usize) * 17 % 256)
}

fn part1(initialization: Vec<&'static str>) -> usize {
    initialization.into_iter().map(|s| hash_of(s)).sum()
}

fn part2(initialization: Vec<&'static str>) -> usize {
    let mut boxes: [HashMap<&str, (usize, usize)>; 256] = std::array::from_fn(|_| HashMap::new());
    initialization.into_iter().enumerate().for_each(|(index, step)| {
        let (label, operation) = step.split_once(|c: char| !c.is_ascii_alphabetic())
            .unwrap();
        let box_number = hash_of(label);
        if operation == "" {
            boxes[box_number].remove(label);
        } else {
            let digit = operation.parse().unwrap();
            let index = if let Some((i, _)) = boxes[box_number].get(label) {
                *i
            } else {
                index
            };
            boxes[box_number].insert(label, (index, digit));
        }
    });
    boxes.into_iter().enumerate()
        .map(|(box_index, lens_box)| {
            lens_box.values()
                .sorted_by(|(a, _), (b, _)| Ord::cmp(&a, &b))
                .enumerate()
                .fold(0, |acc, (lens_index, (_, focal_length))| {
                    acc + (box_index + 1) * (lens_index + 1) * focal_length
                })
        })
        .sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part1(parse_input(input)), 1320);
    assert_eq!(part2(parse_input(input)), 145);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 505427);
    assert_eq!(part2(input), 243747);
}
