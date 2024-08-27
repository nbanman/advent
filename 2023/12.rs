use advent::prelude::*;

struct SpringRow {
    conditions: String,
    damage_report: Vec<usize>,
    cache: HashMap<State, usize>,
}

impl SpringRow {
    fn arrangements(&mut self, s: State) -> usize {
        if let Some(value) = self.cache.get(&s) {
            *value
        } else {
            // do not consider conditions already handled in previous states
            // if state place exceeds the conditions length string, we are done and the block is blank
            let block = if s.conditions_index < self.conditions.len() {
                self.conditions[s.conditions_index..].to_string()
            } else {
                String::from("")
            };

            // the # of consecutive broken springs in the damage report that we try to place along the row
            let fulfillment = self.damage_report.get(s.damage_index).unwrap_or(&0).to_owned();

            // Base case. Takes states that have fulfilled the entire damage report and returns 1 if valid,
            // 0 if invalid. Valid states are those with no remaining '#' in the current or any future blocks,
            // and that have filled all the damaged spring requirements
            if fulfillment == 0usize {
                let value = if block.find('#').is_some() {
                    0
                } else {
                    1
                };
                self.cache.insert(s, value);
                return value;
            }

            // Otherwise, we go recursive by trying to fit the fulfillment in every place along the block
            // This starts as a sequence of indexes, from 0 until the length of the block minus the fulfillment size
            // (to account for the size of the fulfillment itself in the string).
            let value = if block.len() >= fulfillment {
                (0..=block.len() - fulfillment)
                    .take_while(|index| { *index == 0 || block.as_bytes()[index - 1] as char != '#' })
                    .filter(|index| {
                        // filter out invalid placements, in cascading fashion
                        // if the placement includes a '.', invalid b/c '.' means not broken
                        // if the placement has no part of the string after it, valid b/c nothing else to consider
                        // if the character following the placement is '#', invalid b/c that extra '#' would overfulfill
                        // otherwise valid
                        if let Some(_) = &block[*index..index + fulfillment].find('.') {
                            false
                        } else if index + fulfillment == block.len() {
                            true
                        } else if block.as_bytes()[index + fulfillment] == '#' as u8 {
                            false
                        } else {
                            true
                        }
                    })
                    .map(|index| {
                        let new_state = State {
                            conditions_index: s.conditions_index + index + fulfillment + 1,
                            damage_index: s.damage_index + 1,
                        };
                        self.arrangements(new_state)
                    })
                    .sum()
            } else {
                0
            };
            self.cache.insert(s, value);
            value
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct State {
    conditions_index: usize,
    damage_index: usize,
}

fn parse_input(input: &'static str) -> Vec<(String, Vec<usize>)> {
    input.lines()
        .map(|line| {
            let (conditions, damage_str) = line.split_once(' ').unwrap();
            let damage_report: Vec<usize> = damage_str.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            (conditions.to_string(), damage_report)
        })
        .collect()
}

fn default_input() -> Vec<(String, Vec<usize>)> {
    parse_input(include_input!(2023 / 12))
}

fn part1(spring_reports: Vec<(String, Vec<usize>)>) -> usize {
    let spring_rows: Vec<SpringRow> = spring_reports.into_iter()
        .map(|(conditions, damage_report)| {
            let cache: HashMap<State, usize> = HashMap::new();
            SpringRow { conditions, damage_report, cache }
        })
        .collect();

    spring_rows.into_iter()
        .map(|mut spring_row| {
            let result = spring_row.arrangements(
                State {
                    conditions_index: 0,
                    damage_index: 0,
                }
            );
            result
        }).sum()
}

fn part2(spring_reports: Vec<(String, Vec<usize>)>) -> usize {
    let spring_rows: Vec<SpringRow> = spring_reports.into_iter()
        .map(|(conditions, damage_report)| {
            let expanded_conditions = iter::repeat(conditions).take(5)
                .join("?");
            let expanded_damage_report = damage_report.repeat(5);
            let cache: HashMap<State, usize> = HashMap::new();
            SpringRow {
                conditions: expanded_conditions,
                damage_report: expanded_damage_report,
                cache,
            }
        })
        .collect();

    spring_rows.into_iter()
        .map(|mut spring_row| {
            let result = spring_row.arrangements(
                State {
                    conditions_index: 0,
                    damage_index: 0,
                }
            );
            result
        }).sum()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";
    assert_eq!(part1(parse_input(input)), 21);
    assert_eq!(part2(parse_input(input)), 525152);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 7344);
    assert_eq!(part2(input), 1088006519007);
}
