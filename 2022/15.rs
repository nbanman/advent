use std::ops::RangeInclusive;
use advent::prelude::*;

#[derive(Clone)]
struct Sensor {
    pos: Vector2,
    beacon_pos: Vector2,
}

impl Sensor {
    fn to_range(&self, y: i64) -> Option<RangeInclusive<i64>> {
        let x_distance = (self.pos.x - self.beacon_pos.x).abs() +
            (self.pos.y - self.beacon_pos.y).abs() -
            (self.pos.y - y).abs();
        if x_distance >= 0 {
            Some(self.pos.x - x_distance..=self.pos.x + x_distance)
        } else {
            None
        }
    }
}

fn min_max<T, R: PartialOrd, F>(items: &[T], selector: F) -> (&T, &T)
where
    R: Clone,
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

fn is_contiguous(a: &RangeInclusive<i64>, b: &RangeInclusive<i64>) -> Option<RangeInclusive<i64>> {
    let binding = &[a, b];
    let (lesser, greater) = min_max(binding, |it| it.start());
    if lesser.end() >= greater.start() {
        Some(*lesser.start()..=*max(lesser.end(), greater.end()))
    } else {
        None
    }
}

fn parse_input(input: &str) -> Vec<Sensor> {
    regex!(r"-?\d+")
        .find_iter(input)
        .map(|cap| cap.as_str().parse().unwrap())
        .array_chunked::<4>()
        .map(|[x1, y1, x2, y2]: [i64; 4]| Sensor { pos: vector![x1, y1], beacon_pos: vector![x2, y2] })
        .collect()
}

fn concatenate(row_ranges: &mut Vec<RangeInclusive<i64>>) -> Vec<RangeInclusive<i64>> {
    let mut i: usize;
    let mut size = 0usize;
    while size != row_ranges.len() {
        size = row_ranges.len();
        i = 0;
        while i < row_ranges.len() - 1 {
            let mut j = i + 1;
            while j < row_ranges.len() {
                if let Some(union) = is_contiguous(&row_ranges[i],&row_ranges[j]){
                    row_ranges[i] = union;
                    row_ranges.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
    }
    row_ranges.clone()
}
fn row_ranges(sensors: &Vec<Sensor>, y: i64) -> Vec<RangeInclusive<i64>> {
    let mut row_ranges: Vec<_> = sensors.into_iter()
        .filter_map(|sensor| sensor.to_range(y))
        .collect();
    concatenate(&mut row_ranges)
}

fn default_input() -> Vec<Sensor> {
    parse_input(include_input!(2022 / 15))
}


fn part1(sensors: Vec<Sensor>) -> i64 {
    row_ranges(&sensors, 2_000_000).into_iter()
        .map(|range| {
            range.end() - range.start()
        })
        .sum()
}

fn part2(sensors: Vec<Sensor>) -> i64 {
    let (y, range) = iter::successors(Some(0i64), |i| Some(i + 1))
        .map(|y| (y, row_ranges(&sensors, y)))
        .find(|(_, ranges)| ranges.len() > 1)
        .unwrap();
    let x = range.iter().min_by(|x, y| x.start().cmp(&y.start())).unwrap().end() + 1;
    4_000_000i64 * x + y
}

fn main() {
    let solution = advent::new(default_input)
        .part(|i| part1(i))
        .part(|i| part2(i))
        .build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3",
    );
    assert_eq!(part1(input.clone()), 26);
    assert_eq!(part2(input), 56000011);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 5073496);
    assert_eq!(part2(input), 13081194638237);
}
