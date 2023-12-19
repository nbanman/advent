use advent::prelude::*;

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
enum Dir { N, E, S, W }

#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
struct State {
    pos: (isize, isize),
    dir: Dir,
    straights: u32,
}

#[derive(Clone, Eq, Hash, Debug)]
struct Edge {
    state: State,
    weight: u32,
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }

    fn ne(&self, other: &Self) -> bool {
        self.weight != other.weight
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.weight.partial_cmp(&self.weight)
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        let ordering = other.weight.cmp(&self.weight);
        println!("{:?}", ordering);
        ordering
    }
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn default_input() -> Vec<Vec<u32>> {
    parse_input(include_input!(2023 / 17))
}

fn part1(city: Vec<Vec<u32>>) -> u32 {
    let start = State { pos: (0, 0), dir: Dir::E, straights: 0 };
    let end = (city[0].len() as isize - 1, city.len() as isize - 1);
    let mut weights = HashMap::new();
    weights.insert(start.clone(), 0);
    let mut q = BinaryHeap::new();
    q.push(Edge { state: start, weight: 0 });
    let mut visited: HashSet<Edge> = HashSet::new();
    loop {
        let current = if let Some(popped) = poll_unique(&mut q, &visited) {
            popped
        } else {
            break;
        };
        visited.insert(current.to_owned());
        if current.state.pos == end {
            return current.weight.to_owned();
        }
        // make new edges
        let mut edges = Vec::new();
        if current.state.straights < 3 {
            if let Some(neighbor) = move_crucible(&current.state, &current.state.dir, &city) {
                edges.push(neighbor)
            };
        }
        let left = turn_left(&current.state.dir);
        if let Some(neighbor) = move_crucible(&current.state, &left, &city) {
            edges.push(neighbor)
        };
        let right = turn_right(&current.state.dir);
        if let Some(neighbor) = move_crucible(&current.state, &right, &city) {
            edges.push(neighbor)
        };
        edges.iter().for_each(|edge| {
            let alternate_weight = &current.weight + edge.weight;
            // println!("State: {:?}: {:?}", edge, alternate_weight);
            let weight = if let Some(weight) = weights.get(&edge.state) {
                weight
            } else {
                &u32::MAX
            };
            if &alternate_weight < weight {
                weights.insert(edge.state, alternate_weight);
                q.push(Edge { state: edge.state, weight: alternate_weight });
            }
        });
    }
    0
}

fn turn_left(dir: &Dir) -> Dir {
    match dir {
        Dir::N => Dir::W,
        Dir::E => Dir::N,
        Dir::S => Dir::E,
        Dir::W => Dir::S,
    }
}

fn turn_right(dir: &Dir) -> Dir {
    match dir {
        Dir::N => Dir::E,
        Dir::E => Dir::S,
        Dir::S => Dir::W,
        Dir::W => Dir::N,
    }
}

fn move_crucible(state: &State, dir: &Dir, city: &Vec<Vec<u32>>) -> Option<Edge> {
    let (x, y) = state.pos;
    let (x, y) = match dir {
        Dir::N => (x, y - 1),
        Dir::E => (x + 1, y),
        Dir::S => (x, y + 1),
        Dir::W => (x - 1, y),
    };
    let x1: usize = x.try_into().ok()?;
    let y1: usize = y.try_into().ok()?;

    let weight = city.get(y1)?.get(x1)?;
    let straights = if state.dir == *dir { state.straights + 1 } else { 1 };
    return Some(Edge { state: State { pos: (x, y), dir: dir.to_owned(), straights }, weight: weight.to_owned() });
}

fn move_crucible_2(state: &State, dir: &Dir, distance: u32, city: &Vec<Vec<u32>>) -> Option<Edge> {
    if distance == 0 { return None };
    let potential = (1..=distance)
        .fold(Some((state.pos, 0u32)), |acc, _| {
            if let Some((current_pos, heat_loss)) = acc {
                let (x, y) = current_pos;
                let (x, y) = match dir {
                    Dir::N => (x, y - 1),
                    Dir::E => (x + 1, y),
                    Dir::S => (x, y + 1),
                    Dir::W => (x - 1, y),
                };
                let x1: usize = x.try_into().ok()?;
                let y1: usize = y.try_into().ok()?;
                let weight = heat_loss + city.get(y1)?.get(x1)?;
                Some(((x, y), weight))
            } else {
                None
            }
        });
    if let Some((pos, weight)) = potential {
        let straights = if state.dir == *dir { state.straights + distance } else { distance };
        let edge = Edge { state: State {
            pos,
            dir: dir.to_owned(),
            straights,
        }, weight };
        Some(edge)
    } else {
        None
    }

}

fn poll_unique(q: &mut BinaryHeap<Edge>, visited: &HashSet<Edge>) -> Option<Edge> {
    while let Some(popped) = q.pop() {
        if !visited.contains(&popped) { return Some(popped); }
    }
    None
}

fn part2(city: Vec<Vec<u32>>) -> u32 {
    let start = State { pos: (0, 0), dir: Dir::E, straights: 0 };
    let end = (city[0].len() as isize - 1, city.len() as isize - 1);
    let mut weights = HashMap::new();
    weights.insert(start.clone(), 0);
    let mut q = BinaryHeap::new();
    q.push(Edge { state: start, weight: 0 });
    let mut visited: HashSet<Edge> = HashSet::new();
    loop {
        let current = if let Some(popped) = poll_unique(&mut q, &visited) {
            popped
        } else {
            break;
        };
        visited.insert(current.to_owned());
        if current.state.pos == end {
            return current.weight.to_owned();
        }
        // make new edges
        let mut edges = Vec::new();

        // go straight
        let distance: u32 = match current.state.straights {
            0 => 4,
            4..=9 => 1,
            _ => 0,
        };
        if let Some(neighbor) = move_crucible_2(&current.state, &current.state.dir, distance, &city) {
            edges.push(neighbor)
        };

        let left = turn_left(&current.state.dir);
        if let Some(neighbor) = move_crucible_2(&current.state, &left, 4, &city) {
            edges.push(neighbor)
        };
        let right = turn_right(&current.state.dir);
        if let Some(neighbor) = move_crucible_2(&current.state, &right, 4, &city) {
            edges.push(neighbor)
        };
        edges.iter().for_each(|edge| {
            let alternate_weight = &current.weight + edge.weight;
            let weight = if let Some(weight) = weights.get(&edge.state) {
                weight
            } else {
                &u32::MAX
            };
            if &alternate_weight < weight {
                weights.insert(edge.state, alternate_weight);
                q.push(Edge { state: edge.state, weight: alternate_weight });
            }
        });
    }
    0
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example1() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";
    assert_eq!(part1(parse_input(input)), 102);
    assert_eq!(part2(parse_input(input)), 94);
}

#[test]
fn example2() {
    let input = "111111111111
999999999991
999999999991
999999999991
999999999991
";
    assert_eq!(part1(parse_input(input)), 59);
    assert_eq!(part2(parse_input(input)), 71);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 635);
    assert_eq!(part2(input), 734);
}
