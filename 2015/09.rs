use advent::prelude::*;

fn parse_input(input: &'static str) -> HashMap<&'static str, Vec<(&'static str, usize)>> {
    let mut edge_map = HashMap::new();
    for line in input.lines() {
        let mut splits = line.split(' ');
        let (Some(city_a), Some(_), Some(city_b), Some(_), Some(distance)) =
            (splits.next(), splits.next(), splits.next(), splits.next(), splits.next()) else { panic!("parsing error") };
        let distance = distance.parse().unwrap();
        let entry = edge_map.entry(city_a).or_insert(Vec::new());
        (*entry).push((city_b, distance));
        let entry = edge_map.entry(city_b).or_insert(Vec::new());
        (*entry).push((city_a, distance));
    }
    edge_map
}

fn default_input() -> HashMap<&'static str, Vec<(&'static str, usize)>> {
    parse_input(include_input!(2015 / 09))
}

fn tour<F>(
    edge_map: HashMap<&'static str, Vec<(&'static str, usize)>>,
    city: &str,
    end_condition: F,
) -> usize
where F: FnOnce(&Vec<&str>) -> bool
{
    fn default_edges(
        edge_map: &HashMap<&'static str, Vec<(&'static str, usize)>>,
        cities_visited: &Vec<&str>
    ) -> Vec<(usize, Vec<&'static str>)> {
        edge_map
            .get(cities_visited.last().unwrap())
            .unwrap()
            .iter()
            .filter(|&(city, _)| !cities_visited.contains(city))
            .map(|&(city, distance)| {
                let mut next = cities_visited.clone();
                next.push(city);
                (distance, next)
            })
            .collect::<Vec<_>>()
    }

    let cities: HashSet<_> = edge_map.keys().map(|key| *key).collect();

    let mut visited = HashSet::new();
    let mut pq = BinaryHeap::new();
    let start = (0, vec![city]);
    let mut distances = HashMap::new();

    pq.push(Reverse(start));

    while let Some(Reverse((distance, current))) = pq.pop() {
        if visited.contains(&current) { continue; }

        visited.insert(current);
        if end_condition(&current) == true {
            return distance
        }

        for (neighbor_dist, neighbor) in default_edges(&edge_map, &current) {
            let alternate_weight = distance + neighbor_dist;
            let weight: usize = *distances.get(&neighbor).unwrap_or(&usize::MAX);
            if alternate_weight < weight {
                pq.push(Reverse((alternate_weight, neighbor.clone())));
                distances.insert(neighbor, alternate_weight);
            }
        }
    }
    unreachable!("No end condition obtained!");
}

fn part1(input: HashMap<&'static str, Vec<(&'static str, usize)>>) -> usize {
    let city_num = input.len();
    let end_condition = |current: &Vec<&str>| {
        current.len() == city_num
    };
    input.iter()
        .map(|(&city, _)| tour(input, city, end_condition))
        .min()
        .unwrap()
}

fn part2(input: HashMap<&'static str, Vec<(&'static str, usize)>>) -> usize {
    todo!("part 2")
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 207);
    assert_eq!(part2(input), 804);
}
