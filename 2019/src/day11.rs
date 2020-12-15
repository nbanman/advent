use std::collections::BTreeMap;

use crate::day09::Computer;
use crate::intcode::{parse_program, State};

const INPUT: &str = include_str!("input/day11.txt");

pub fn default_input() -> Vec<i64> {
    parse_program(INPUT)
}

type Point = (i64, i64);

#[derive(Debug, Clone, Copy)]
enum Color {
    Black = 0,
    White = 1,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

use Color::*;
use Turn::*;

impl Computer {
    fn next_turn(&mut self, input: i64) -> Turn {
        match self.next(input).unwrap() {
            0 => Left,
            1 => Right,
            t => panic!("invalid turn `{}`", t),
        }
    }

    fn next_color(&mut self, input: i64) -> Option<Color> {
        match self.next(input) {
            State::Complete => None,
            State::Yielded(0) => Some(Black),
            State::Yielded(1) => Some(White),
            state => panic!("unexpected state `{:?}`", state),
        }
    }

    fn next_color_and_turn(&mut self, color: Color) -> Option<(Color, Turn)> {
        let input = color as i64;
        self.next_color(input)
            .map(|color| (color, self.next_turn(input)))
    }
}

fn rotate((x, y): Point, turn: Turn) -> Point {
    match turn {
        Left => (-y, x),
        Right => (y, -x),
    }
}

fn paint(program: &[i64], color: Color) -> BTreeMap<Point, Color> {
    let mut computer = Computer::new(program.to_vec());
    let mut map = BTreeMap::new();
    let mut position = (0, 0);
    let mut direction = (0, 1);
    while let Some((color, turn)) =
        computer.next_color_and_turn(*map.get(&position).unwrap_or(&color))
    {
        map.insert(position, color);
        direction = rotate(direction, turn);
        position = (position.0 + direction.0, position.1 + direction.1);
    }
    map
}

pub fn part1(program: &[i64]) -> usize {
    paint(program, Black).len()
}

pub fn part2(program: &[i64]) -> String {
    let map = paint(program, White);

    let min_x = *map.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *map.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *map.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *map.keys().map(|(_, y)| y).max().unwrap();

    let mut result = String::from('\n');
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            result.push_str(match map.get(&(x, y)) {
                Some(Black) | None => "  ",
                Some(White) => "██",
            })
        }
        result.push('\n');
    }
    result
}