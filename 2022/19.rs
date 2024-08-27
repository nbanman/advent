use advent::prelude::*;

#[derive(Clone)]
struct Blueprint {
    id: usize,
    robot_costs: HashMap<String, HashMap<String, usize>>,
    max_material: HashMap<String, usize>,
}

impl Blueprint {
    fn new(spec: &str) -> Blueprint {
        let (_, id) = spec.split_once(' ').unwrap();
        let (id, _) = id.split_once(':').unwrap();
        let id = id.parse().unwrap();

        let mut robot_costs: HashMap<String, HashMap<String, usize>> = HashMap::new();
        let rx = regex!(r"Each ([a-z]+) robot costs (\d+) ([a-z]+)(?: and (\d+) ([a-z]+))?. ?");
        for caps in rx.captures_iter(spec) {
            let robot_type = caps.get(1).unwrap().as_str();
            let cost1: usize = caps.get(2).unwrap().as_str().parse().unwrap();
            let resource1 = caps.get(3).unwrap().as_str();

            let mut resource_map: HashMap<String, usize> = HashMap::new();

            resource_map.insert(resource1.to_string(), cost1);

            if let Some(cost2) = caps.get(4) {
                let cost2: usize = cost2.as_str().parse().unwrap();
                let resource2 = caps.get(5).unwrap().as_str();
                resource_map.insert(resource2.to_string(), cost2);
            }

            robot_costs.insert(robot_type.to_string(), resource_map);
        }

        let max_material: HashMap<String, usize> = robot_costs
            .keys()
            .filter_map(|resource| {
                robot_costs
                    .values()
                    .filter_map(|resource_map| {
                        resource_map.get(resource)
                    })
                    .max()
                    .map(|max| (resource.to_string(), *max))
            })
            .collect();

        Blueprint { id, robot_costs, max_material }
    }
}

struct State {
    minute: usize,
    resources: HashMap<String, usize>,
    robots: HashMap<String, usize>,
}

impl State {
    fn new() -> State {
        let robots: HashMap<String, usize> = vec![(String::from("ore"), 1)].into_iter().collect();

        let resources: HashMap<String, usize> = vec![
            (String::from("ore"), 0),
            (String::from("clay"), 0),
            (String::from("obsidian"), 0),
            (String::from("geode"), 0),
        ]
            .into_iter()
            .collect();

        State {
            minute: 0,
            resources,
            robots,
        }
    }

    fn next_states(
        &self, blueprint: &Blueprint, minutes: usize, cutoff: &HashMap<String, usize>,
    ) -> Vec<State> {
        blueprint.robot_costs.keys()
            .filter_map(|robot_type| {
                if self.robots.get(robot_type).unwrap_or(&0) ==
                    (blueprint.max_material.get(robot_type).unwrap_or(&usize::MAX)) {
                    None
                } else {
                    if let None = minutes
                        .checked_sub(self.minute)
                        .and_then(|m| {
                            m.checked_sub(*cutoff.get(robot_type).unwrap_or(&0usize))
                        }) {
                        None
                    } else {
                        let build_time = self.build_time(blueprint, robot_type);
                        if let None = minutes.checked_sub(self.minute).and_then(|m| m.checked_sub(build_time)) {
                            None
                        } else {
                            let mut new_robots = self.robots.clone();
                            new_robots.insert(
                                robot_type.to_string(),
                                self.robots.get(robot_type).unwrap_or(&0) + 1,
                            );
                            let mut new_resources = self.resources.clone();
                            let costs = blueprint.robot_costs.get(robot_type).unwrap();
                            for (component, cost) in new_resources.iter_mut() {
                                let minuend = *cost +
                                    self.robots.get(component).unwrap_or(&0) * build_time;
                                let subtrahend = *costs.get(component).unwrap_or(&0);
                                *cost = minuend.checked_sub(subtrahend).unwrap_or(0);
                            }
                            Some(State {
                                minute: self.minute + build_time,
                                resources: new_resources,
                                robots: new_robots,
                            })
                        }
                    }
                }
            })
            .collect()
    }

    fn build_time(&self, blueprint: &Blueprint, robot_type: &str) -> usize {
        blueprint.robot_costs.get(robot_type).unwrap()
            .iter()
            .map(|(component, cost)| {
                let resources_available = self.resources.get(component).unwrap_or(&0);
                if cost <= resources_available {
                    1
                } else {
                    let robots_available = self.robots.get(component).unwrap_or(&0);
                    if *robots_available == 0 {
                        usize::MAX
                    } else {
                        ((cost - resources_available) as f64 / *robots_available as f64).ceil() as usize
                    }
                }
            })
            .max()
            .unwrap()
    }

    fn max_bound(&self, minutes: usize, resource: &str) -> usize {
        let current_amt = self.resources.get(resource).unwrap_or(&0);
        let current_robot_num = self.robots.get(resource).unwrap_or(&0);
        let calc = 0..minutes - self.minute;
        let calc: usize = calc
            .map(|it| {
                &it + current_robot_num
            })
            .sum();
        current_amt + calc
    }

    fn min_bound(&self, minutes: usize, resource: &str) -> usize {
        let current_amt = self.resources.get(resource).unwrap_or(&0);
        let current_robot_num = self.robots.get(resource).unwrap_or(&0);
        current_amt + (minutes - self.minute) * current_robot_num
    }
}

fn find_resource(
    blueprint: &Blueprint, resource: &str, initial_state: State, minutes: usize,
) -> usize {
    let cutoff = blueprint.robot_costs.keys()
        .map(|robot_type| {
            let amt = if robot_type.as_str() == resource {
                1
            } else if let Some(resource_map) = blueprint.robot_costs.get(resource) {
                if resource_map.contains_key(robot_type) {
                    3
                } else {
                    5
                }
            } else {
                5
            };
            (robot_type.clone(), amt)
        })
        .collect();

    let mut max_geodes = 0usize;
    let mut queue = VecDeque::new();
    queue.push_front(initial_state);
    while let Some(state) = queue.pop_back() {
        if state.max_bound(minutes, resource) < max_geodes {
            continue;
        }
        let min_geodes = state.min_bound(minutes, resource);
        if min_geodes > max_geodes {
            max_geodes = min_geodes
        }
        let next_states = state.next_states(blueprint, minutes, &cutoff);
        for next_state in next_states.into_iter() {
            queue.push_back(next_state)
        }
    }
    max_geodes
}

fn parse_input(input: &str) -> Vec<Blueprint> {
    input
        .lines()
        .map(|spec| Blueprint::new(spec))
        .collect()
}

fn default_input() -> Vec<Blueprint> {
    parse_input(include_input!(2022 / 19))
}

fn part1(blueprints: Vec<Blueprint>) -> usize {
    blueprints.into_iter().map(|blueprint| {
        blueprint.id * find_resource(&blueprint, "geode", State::new(), 24)
    }).sum()
}

fn part2(blueprints: Vec<Blueprint>) -> usize {
    blueprints.into_iter().take(3).map(|blueprint| {
        blueprint.id * find_resource(&blueprint, "geode", State::new(), 32)
    })
        .reduce(|acc, amt| acc * amt)
        .unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input("\
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.");
    assert_eq!(part1(input), 33);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1427);
    assert_eq!(part2(input), 4400);
}
