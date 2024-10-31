pub use std::cmp;
pub use std::cmp::{max, min, Ordering, Reverse};
pub use std::collections::{BinaryHeap, BTreeMap, BTreeSet, VecDeque};
pub use std::iter;
pub use std::mem;
use std::str::FromStr;

pub use ahash::{HashMap, HashMapExt as _, HashSet, HashSetExt as _};
pub use either::Either;
pub use itermore::{IterArrayChunks as _, IterArrayCombinations as _, IterArrayWindows as _};
pub use itertools::{iproduct, Itertools as _};
use pathfinding::num_traits::CheckedAdd;
pub use regex_macro::regex;
pub use vectrix::{Matrix, vector, Vector};

pub use then::Some as _;

pub use advent_ocr::ocr;

pub type Vector2 = vectrix::Vector<i64, 2>;
pub type Vector3 = vectrix::Vector<i64, 3>;
pub type Vector4 = vectrix::Vector<i64, 4>;

#[macro_export]
macro_rules! include_input {
    ($year:literal / $day:literal) => {{
        include_str!(concat!(
            "../input/",
            stringify!($year),
            "/",
            stringify!($day),
            ".txt"
        ))
    }};
}

#[macro_export]
macro_rules! vectors {
    ($([$($e:expr),+]),+ $(,)?) => {
        [$($crate::vector![$($e,)+],)+]
    };
}

/// Returns the greatest common divisor of two numbers.
pub fn gcd(mut x: i64, mut y: i64) -> i64 {
    while x != 0 {
        let tmp = x;
        x = y % tmp;
        y = tmp;
    }
    y.abs()
}

/// Returns the least common multiple of two numbers.
pub fn lcm(x: i64, y: i64) -> i64 {
    x * y / gcd(x, y)
}

/// Parses a 2D map into a data structure of type `M`.
pub fn parse_map<M, F, V>(input: &str, parse: F) -> M
    where
        M: FromIterator<(Vector2, V)>,
        F: Fn(char) -> V,
{
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| (x, parse(c)))
                .map(move |(x, v)| (vector![x as i64, y as i64], v))
        })
        .collect()
}

/// Parses a 2D map that uses '.' for spaces and '#' for walls as a set of the
/// the '#' points.
pub fn parse_map_set(input: &str) -> HashSet<Vector2> {
    let map: HashMap<_, _> = parse_map(input, |c| match c {
        '#' => Some(()),
        '.' => None,
        c => panic!("unrecognized character `{c}`"),
    });
    map.into_iter().filter_map(|(k, v)| v.map(|_| k)).collect()
}

pub fn get_numbers<N>(s: &str) -> Vec<N>
    where
        N: CheckedAdd + FromStr
{
    let mut numbers: Vec<N> = Vec::new();
    let mut start_position = -1isize;
    let s = s.as_bytes();
    for (position, c) in s.iter().enumerate() {
        let position = position as isize;
        if c.is_ascii_digit() ||
            (c == &b'-' && s.try_get(position - 1).map(|x| x.is_ascii_digit()) != Some(true)) {
            if start_position == -1 {
                start_position = position
            }
        } else {
            if start_position != -1 {
                if let Ok(sub_str) = std::str::from_utf8(&s[start_position as usize..position as usize]) {
                    if let Ok(parsed) = sub_str.parse::<N>() {
                        numbers.push(parsed);
                    }
                }
                start_position = -1;
            }
        }
    }
    if start_position != -1 {
        if let Ok(sub_str) = std::str::from_utf8(&s[start_position as usize..]) {
            if let Ok(parsed) = sub_str.parse::<N>() {
                numbers.push(parsed);
            }
        }
    }
    numbers
}

pub struct NumberIterator<'a, N>
    where
        N: CheckedAdd + FromStr
{
    s: &'a [u8],
    start_position: isize,
    position: isize,
    phantom: std::marker::PhantomData<N>,
}

impl<'a, N> NumberIterator<'a, N>
    where
        N: CheckedAdd + FromStr
{
    pub fn new(input: &'a str) -> Self {
        NumberIterator {
            s: input.as_bytes(),
            start_position: -1,
            position: 0,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, N> Iterator for NumberIterator<'a, N>
    where
        N: CheckedAdd + FromStr
{
    type Item = N;

    fn next(&mut self) -> Option<Self::Item> {
        while self.position < self.s.len() as isize {
            let c = self.s[self.position as usize];

            if c.is_ascii_digit() ||
                (c == b'-' && self.s.try_get(self.position - 1).map(|x| x.is_ascii_digit()) != Some(true)) {
                if self.start_position == -1 {
                    self.start_position = self.position;
                }
            } else {
                if self.start_position != -1 {
                    if let Ok(sub_str) =
                        std::str::from_utf8(&self.s[self.start_position as usize..self.position as usize]) {
                        if let Ok(parsed) = sub_str.parse::<N>() {
                            self.start_position = -1;
                            self.position += 1;
                            return Some(parsed);
                        } else {
                            self.start_position = -1;
                        }
                    }
                }
            }
            self.position += 1;
        }

        if self.start_position != -1 {
            if let Ok(sub_str) = std::str::from_utf8(&self.s[self.start_position as usize..]) {
                if let Ok(parsed) = sub_str.parse::<N>() {
                    self.start_position = -1; // probably unnecessary
                    return Some(parsed);
                }
            }
        }
        None
    }
}

pub trait ContainsNumbers
{
    fn get_numbers<N>(&self) -> NumberIterator<'_, N>
        where
            N: CheckedAdd + FromStr;
}

impl<'a> ContainsNumbers for &'a str {
    fn get_numbers<N>(&self) -> NumberIterator<'_, N>
        where
            N: CheckedAdd + FromStr
    {
        NumberIterator::new(self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl Direction {
    pub fn ordinal(&self) -> usize {
        match self {
            Direction::N => 0,
            Direction::E => 1,
            Direction::S => 2,
            Direction::W => 3,
        }
    }

    fn from_ordinal(ordinal: usize) -> Option<Direction> {
        match ordinal {
            0 => Some(Direction::N),
            1 => Some(Direction::E),
            2 => Some(Direction::S),
            3 => Some(Direction::W),
            _ => None,
        }
    }
    pub fn left(&self) -> Direction {
        let left_ordinal = ((self.ordinal() as isize) - 1).rem_euclid(4) as usize;
        Direction::from_ordinal(left_ordinal).unwrap()
    }

    pub fn right(&self) -> Direction {
        let right_ordinal = (self.ordinal() + 1).rem_euclid(4);
        Direction::from_ordinal(right_ordinal).unwrap()
    }
}

/// Get from arrays with anything that can be converted to a usize
pub trait TryGet<T> {
    fn try_get<U: TryInto<usize>>(&self, n: U) -> Option<&T>;
}

impl<T> TryGet<T> for [T] {
    fn try_get<U: TryInto<usize>>(&self, n: U) -> Option<&T> {
        if let Ok(index) = n.try_into() {
            self.get(index)
        } else {
            None
        }
    }
}

pub trait Grid<T> {
    fn get_neighbors(&self, position: usize, width: usize) -> Vec<usize>;
    fn get_neighbors_diagonal(&self, position: usize, width: usize) -> Vec<usize>;
}

trait GridHelper<T> {
    fn get_neighbors_helper(&self, position: usize, directions: Vec<isize>) -> Vec<usize>;
}

impl<T> GridHelper<T> for [T] {
    fn get_neighbors_helper(&self, position: usize, directions: Vec<isize>) -> Vec<usize> {
        directions.into_iter()
            .filter_map(|direction| {
                let new_position = position as isize + direction;
                if let Some(_) = self.try_get(new_position) {
                    Some(new_position as usize)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<T> Grid<T> for [T] {
    fn get_neighbors(&self, position: usize, width: usize) -> Vec<usize> {
        let width = width as isize;
        let directions = vec![-width, width, -1, 1];
        self.get_neighbors_helper(position, directions)
    }

    fn get_neighbors_diagonal(&self, position: usize, width: usize) -> Vec<usize> {
        let width = width as isize;
        let directions = vec![
            -width - 1, -width, -width + 1, -1, 1, width - 1, width, width + 1,
        ];
        self.get_neighbors_helper(position, directions)
    }
}

pub fn min_max_with<T, R, F>(items: &[T], selector: F) -> (&T, &T)
    where
        R: PartialOrd + Clone,
        F: Fn(&T) -> R
{
    let mut min = &items[0];
    let mut min_value = selector(min);
    let mut max = min;
    let mut max_value = min_value.clone();
    for item in items {
        let selected = selector(item);
        if selected < min_value {
            min = item;
            min_value = selected;
        } else if selected > max_value {
            max = item;
            max_value = selected
        }
    }
    (min, max)
}

pub fn min_max<T>(items: &[T]) -> (&T, &T)
    where
        T: PartialOrd,
{
    let mut min = &items[0];
    let mut max = min;
    for item in items {
        if item < min {
            min = item;
        } else if item > max {
            max = item;
        }
    }
    (min, max)
}