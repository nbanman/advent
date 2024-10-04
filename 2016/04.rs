use once_cell::sync::Lazy;
use regex::Regex;
use advent::prelude::*;

static RX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"([a-z-]+)-(\d+)\[([a-z]+)\]").unwrap()
});

#[derive(Clone, Debug)]
struct Room {
    name: String,
    id: usize,
}

impl Room {
    fn new(s: &str) -> Option<Room> {
        RX
            .captures(s)
            .map(|m| {
                let encrypted_name = m.get(1).unwrap().as_str();
                let id: usize = m.get(2).unwrap().as_str().parse().unwrap();
                let checksum = m.get(3).unwrap().as_str();

                let mut entries = [0; 26];
                for c in encrypted_name.chars() {
                    if c != '-' {
                        let code = c as usize - 97;
                        entries[code] += 1;
                    }
                }

                let check: String = entries
                    .into_iter()
                    .enumerate()
                    .sorted_by_key(|(id, freq)| (Reverse(*freq), *id))
                    .take(5)
                    .map(|(id, _)| (id + 97) as u8 as char)
                    .collect();

                if checksum == check {
                    let name = encrypted_name.chars()
                        .map(|c| {
                            if c == '-' {
                                ' '
                            } else {
                                ((c as u8 as usize - 97 + id) % 26 + 97) as u8 as char
                            }
                        })
                        .collect();
                    Some(Room { name, id })
                } else {
                    None
                }
            })
            .flatten()
    }
}
fn parse_input(input: &str) -> Vec<Room> {
    input.lines().filter_map(Room::new).collect()
}

fn default_input() -> Vec<Room> {
    parse_input(include_input!(2016 / 04))
}

fn part1(rooms: Vec<Room>) -> usize {
    rooms.into_iter().map(|room| room.id).sum()
}

fn part2(rooms: Vec<Room>) -> usize {
    rooms.into_iter()
        .find(|room| room.name == "northpole object storage")
        .unwrap()
        .id
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 158835);
    assert_eq!(part2(input), 993);
}
