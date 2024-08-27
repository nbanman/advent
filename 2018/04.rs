use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<isize, Vec<[bool; 60]>> {
    let mut guards = HashMap::new();
    let log: Vec<_> = input
        .lines()
        .map(|line| {
            let mut ints: NumberIterator<'_, isize> = line.get_numbers();
            let date: Vec<_> = ints.by_ref().take(5).collect();
            let id = if let Some(id) = ints.next() { id } else { -1 };
            (date, id)
        })
        .sorted_unstable_by_key(|(date_time, _)| date_time.clone())
        .collect();

    let mut index = 0;

    loop {
        let (_, id) = log[index];
        let mut hour = [false; 60];
        let mut minute = 0;
        let mut asleep = false;
        index += 1;

        while index < log.len() && log[index].1 == -1 {
            let next_minute = log[index].0[4];
            for min in minute..next_minute {
                hour[min as usize] = asleep;
            }
            minute = next_minute;
            asleep = !asleep;
            index += 1;
        }

        for min in minute..60 {
            hour[min as usize] = asleep;
        }

        let entry = guards.entry(id).or_insert(Vec::new());
        entry.push(hour);
        if index >= log.len() { break; }
    }

    guards
}

fn default_input() -> HashMap<isize, Vec<[bool; 60]>> {
    parse_input(include_input!(2018 / 04))
}

fn part1(guards: HashMap<isize, Vec<[bool; 60]>>) -> isize {
    let (id, days) = guards
        .into_iter()
        .max_by_key(|(_, days)| {
            days
                .iter()
                .flat_map(|hour| hour.iter().filter(|b| **b == true))
                .count()
        })
        .unwrap();

    let most_asleep = (0..60)
        .max_by_key(|&minute| { days.iter().filter(|day| day[minute as usize]).count() })
        .unwrap();

    id * most_asleep
}

fn part2(guards: HashMap<isize, Vec<[bool; 60]>>) -> isize {
    guards
        .into_iter()
        .map(|(id, days)| {
            let days_ref = &days;
            (0..60)
                .map(|x| (0..days.len()).map(move |y| days_ref[y][x]))
                .enumerate()
                .map(|(minute, sleep_record)| {
                    (minute, sleep_record.filter(|p| *p).count())
                })
                .max_by_key(|(_, asleep)| asleep.clone())
                .map(|(minute, asleep)| (id, minute, asleep))
                .unwrap()
        })
        .max_by_key(|(_, _, asleep)| asleep.clone())
        .map(|(id, minute, _)| id * minute as isize)
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up",
    );
    assert_eq!(part1(input), 240)
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 19025);
    assert_eq!(part2(input), 23776);
}
