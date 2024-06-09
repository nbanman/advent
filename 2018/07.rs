use advent::prelude::*;

fn parse_input(input: &str) -> Vec<(u8, u8)> {
    input
        .lines()
        .map(|line| {
            let line = line.as_bytes();
            (line[5], line[36])
        })
        .collect()
}

fn default_input() -> Vec<(u8, u8)> {
    parse_input(include_input!(2018 / 07))
}

#[derive(Copy, Clone)]
struct Worker {
    working_on: u8,
    ready: usize,
}

impl Worker {
    fn is_finished(&self, t: usize) -> bool {
        self.working_on.is_ascii_uppercase() && self.ready == t
    }

    fn is_idle(&self) -> bool {
        self.working_on == b'.'
    }

    fn assign(&mut self, step: u8, t: usize, offset: usize) {
        self.working_on = step;
        self.ready = t + offset + step as usize - 64;
    }
}

fn solve(instructions: Vec<(u8, u8)>, workers: usize, time_offset: usize) -> (String, usize) {
    let mut next_steps: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut preceding_steps: HashMap<u8, Vec<u8>> = HashMap::new();

    for (from, to) in instructions {
        next_steps.entry(from).or_insert(Vec::new()).push(to);
        preceding_steps.entry(to).or_insert(Vec::new()).push(from);
    }

    let tos: HashSet<_> = next_steps.keys().cloned().collect();
    let froms: HashSet<_> = preceding_steps.keys().cloned().collect();
    let starts: Vec<_> = tos
        .difference(&froms)
        .cloned()
        .collect();

    let mut queue = BinaryHeap::new();
    for start in starts {
        queue.push(Reverse(start))
    }
    let mut steps = HashSet::new();
    let number_of_steps = next_steps.keys().cloned().collect::<HashSet<_>>()
        .union(&preceding_steps.keys().cloned().collect::<HashSet<_>>())
        .collect::<Vec<_>>()
        .len();
    let mut worker_pool = vec![Worker { working_on: b'.', ready: 0 }; workers];
    let mut answer = String::new();
    let time = iter::successors(Some(0), |&t| Some(t + 1) )
        .find(|&t| {
            for worker in worker_pool.iter_mut().filter(|it| it.is_finished(t)) {
                let product = worker.working_on;
                steps.insert(product.clone());
                answer.push(product as char);
                worker.working_on = b'.';

                if let Some(next) = next_steps.get(&product) {
                    next.iter()
                        .filter(|next_char| {
                            preceding_steps.get(&next_char).unwrap().iter()
                                .all(|it| steps.contains(it))
                        })
                        .for_each(|next_char| {
                            queue.push(Reverse(*next_char))
                        });
                }
            }

            for worker in worker_pool.iter_mut().filter(|it| it.is_idle()) {
                if let Some(Reverse(assignment)) = queue.pop() {
                    worker.assign(assignment, t, time_offset)
                }
            }
            steps.len() == number_of_steps
        })
        .unwrap();

    (answer, time)
}

fn part1(instrs: Vec<(u8, u8)>) -> String {
    solve(instrs, 1, 0).0
}

fn part2(instrs: Vec<(u8, u8)>, workers: usize, dt: usize) -> usize {
    solve(instrs, workers, dt).1
}

fn main() {
    let solution = advent::new(default_input)
        .part(part1)
        .part(|i| part2(i, 5, 60))
        .build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.",
    );
    assert_eq!(part1(input.clone()), "CABDFE");
    assert_eq!(part2(input, 2, 0), 15);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), "ABGKCMVWYDEHFOPQUILSTNZRJX");
    assert_eq!(part2(input, 5, 60), 898);
}
