use advent::prelude::*;

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input.lines()
        .map(|line| line.chars().filter_map(|c| c.to_digit(10)).collect())
        .collect()
}

fn default_input() -> Vec<Vec<u32>> {
    parse_input(include_input!(2023 / 17))
}

fn astar(city: Vec<Vec<u32>>, l: usize, h: usize) -> u32 {
    let width = city.first().unwrap().len();
    let height = city.len();
    let mut bq = vec![Vec::with_capacity(300); 100];
    bq[0].push(0);
    bq[0].push(1);
    let mut cost = vec![vec![vec![0_u32; 2]; width]; height];
    let end = width * height - 1;
    let mut bucket_index = 0_usize;

    loop {
        while let Some(state) = bq[bucket_index % 100].pop() {
            let pos = state >> 1;
            let dir = state & 1;
            let x = pos % width;
            let y = pos / width;
            let steps = cost[y][x][dir];

            if pos == end { return steps };

            let heuristic = {|x: usize, y: usize, steps: u32|
                (steps as usize + width - x + height - y) % 100
            };

            let mut new_x;
            let mut new_y;
            let mut new_steps;

            if dir == 0 {
                // left
                new_x = x;
                new_y = y;
                new_steps = steps;
                for i in 1..=h {
                    if i > x { break };
                    new_x -= 1;
                    new_steps += city[new_y][new_x];

                    if i >= l && (cost[new_y][new_x][1] == 0 || new_steps < cost[new_y][new_x][1]) {
                        bq[heuristic(x - i, y, new_steps)]
                            .push(((new_y * width + new_x) << 1) + 1);
                        cost[new_y][new_x][1] = new_steps;
                    }
                }
                // right
                new_x = x;
                new_y = y;
                new_steps = steps;
                for i in 1..=h {
                    if x + i >= width { break };
                    new_x += 1;
                    new_steps += city[new_y][new_x];

                    if i >= l && (cost[new_y][new_x][1] == 0 || new_steps < cost[new_y][new_x][1]) {
                        bq[heuristic(x + i, y, new_steps)]
                            .push(((new_y * width + new_x) << 1) + 1);
                        cost[new_y][new_x][1] = new_steps;
                    }
                }
            } else {
                // up
                new_x = x;
                new_y = y;
                new_steps = steps;
                for i in 1..=h {
                    if i > y { break };
                    new_y -= 1;
                    new_steps += city[new_y][new_x];

                    if i >= l && (cost[new_y][new_x][0] == 0 || new_steps < cost[new_y][new_x][0]) {
                        bq[heuristic(x, y - i, new_steps)]
                            .push((new_y * width + new_x) << 1);
                        cost[new_y][new_x][0] = new_steps;
                    }
                }

                // down
                new_x = x;
                new_y = y;
                new_steps = steps;
                for i in 1..=h {
                    if y + i >= height { break };
                    new_y += 1;
                    new_steps += city[new_y][new_x];

                    if i >= l && (cost[new_y][new_x][0] == 0 || new_steps < cost[new_y][new_x][0]) {
                        bq[heuristic(x, y + i, new_steps)]
                            .push((new_y * width + new_x) << 1);
                        cost[new_y][new_x][0] = new_steps;
                    }
                }
            }
        }
        bucket_index += 1;
    }
}

fn part1(city: Vec<Vec<u32>>) -> u32 { astar(city, 1, 3) }

fn part2(city: Vec<Vec<u32>>) -> u32 { astar(city, 4, 10) }

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
