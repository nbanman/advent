use advent::prelude::*;
use crate::ItemType::{Generator, Microchip};

#[derive(Copy, Clone)]
enum ItemType {
    Microchip,
    Generator,
}
#[derive(Copy, Clone)]
struct RadioItem<'a> {
    name: &'a str,
    item_type: ItemType,
}

#[derive(Clone)]
struct Floor<'a>(Vec<RadioItem<'a>>);

#[derive(Clone)]
struct FloorState<'a> {
    elevator: usize,
    floors: Vec<Floor<'a>>
}

fn parse_floors(input: &str, part2: bool) -> FloorState {
    let pattern = regex!(r"(\w+)(?: |-compatible )(generator|microchip)");

    let floors: Vec<Floor> = input
        .split('\n')
        .enumerate()
        .map(|(index, line)| {
            let mut from_input: Vec<RadioItem> = pattern.captures_iter(line)
                .filter_map(|captures| {
                    if let Some((name, item_type)) =
                        captures.iter().next_tuple() {

                        let name = name.unwrap().as_str();
                        let item_type = item_type.unwrap().as_str();
                        let item_type = match item_type {
                            "microchip" => Microchip,
                            _ => Generator,
                        };
                        Some(RadioItem { name, item_type })
                    } else {
                        None
                    }
                })
                .collect();
            if part2 && index == 0 {
                from_input.extend(vec![
                    RadioItem { name: "elerium", item_type: Generator },
                    RadioItem { name: "elerium", item_type: Microchip },
                    RadioItem { name: "dilithium", item_type: Generator },
                    RadioItem { name: "dilithium", item_type: Microchip },
                ]);
            }
            Floor(from_input)
        })
        .collect();

    FloorState {
        elevator: 0,
        floors,
    }
}

fn solve_floors(initial_state: FloorState) -> usize {
    todo!()
}


fn default_input() -> &'static str {
    include_input!(2016 / 11)
}

fn part1(input: &str) -> usize {
    let state = parse_floors(input, false);
    solve_floors(state)
}
fn part2(input: &str) -> usize {
    let state = parse_floors(input, true);
    solve_floors(state)
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1);
    assert_eq!(part2(input), 2);
}
