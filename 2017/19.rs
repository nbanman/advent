use advent::prelude::*;

fn default_input() -> &'static str {
    include_input!(2017 / 19)
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Nsew {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

struct Grid<T> {
    data: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    fn new(data: Vec<T>, width: usize) -> Self {
        Grid { data, width }
    }

    fn index(&self, idx: usize) -> &T {
        &self.data[idx]
    }

    fn get_neighbor_indices(&self, index: usize) -> Vec<usize> {
        let mut neighbors = Vec::new();
        if index >= self.width { // has north neighbor
            neighbors.push(index - self.width);
        }
        if index < self.data.len() - self.width { // has south neighbor
            neighbors.push(index + self.width);
        }
        if index % self.width != 0 { // has west neighbor
            neighbors.push(index - 1);
        }
        if (index + 1) % self.width != 0 { // has east neighbor
            neighbors.push(index + 1);
        }
        neighbors
    }
}

struct Mouse {
    maze: Grid<char>,
    start_index: usize,
}

impl Mouse {
    fn new(maze: Grid<char>, start_index: usize) -> Self {
        Mouse { maze, start_index }
    }

    fn run_maze(&self, index: usize, direction: Nsew, report: String, steps: usize) -> (String, usize) {
        let spot = self.maze.index(index);
        let mut new_report = report.clone();
        if spot.is_alphabetic() {
            new_report.push(*spot);
        }
        let neighbor_indices: Vec<usize> = self.maze.get_neighbor_indices(index).into_iter().filter(|&i| *self.maze.index(i) != ' ').collect();

        if neighbor_indices.len() == 1 && index != self.start_index {
            return (new_report, steps + 1);
        }

        let new_index = if *spot == '+' {
            if direction == Nsew::NORTH || direction == Nsew::SOUTH {
                if neighbor_indices.contains(&(index - 1)) {
                    index - 1
                } else {
                    index + 1
                }
            } else { // location is left or right
                if neighbor_indices.contains(&(index - self.maze.width)) {
                    index - self.maze.width
                } else {
                    index + self.maze.width
                }
            }
        } else {
            match direction {
                Nsew::NORTH => index - self.maze.width,
                Nsew::SOUTH => index + self.maze.width,
                Nsew::EAST => index + 1,
                Nsew::WEST => index - 1,
            }
        };
        let new_direction = match new_index {
            i if i == index - 1 => Nsew::WEST,
            i if i == index + 1 => Nsew::EAST,
            i if i == index + self.maze.width => Nsew::SOUTH,
            _ => Nsew::NORTH,
        };
        self.run_maze(new_index, new_direction, new_report, steps + 1)
    }
}

fn part1(input: &str) -> String {
    let maze = input.to_string().chars().collect::<Vec<_>>();
    let width = maze.iter().position(|&c| c == '\n').unwrap_or(maze.len());
    let maze_grid = Grid::new(maze.into_iter().filter(|&c| c != '\n').collect(), width);
    let start_index = maze_grid.data.iter().position(|&c| c != ' ').unwrap();

    let mouse = Mouse::new(maze_grid, start_index);
    let solution = mouse.run_maze(start_index, Nsew::SOUTH, String::new(), 0);
    solution.0
}

fn part2(input: &str) -> usize {
    let maze = input.to_string().chars().collect::<Vec<_>>();
    let width = maze.iter().position(|&c| c == '\n').unwrap_or(maze.len());
    let maze_grid = Grid::new(maze.into_iter().filter(|&c| c != '\n').collect(), width);
    let start_index = maze_grid.data.iter().position(|&c| c != ' ').unwrap();

    let mouse = Mouse::new(maze_grid, start_index);
    let solution = mouse.run_maze(start_index, Nsew::SOUTH, String::new(), 0);
    solution.1
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input), String::from("EOCZQMURF"));
    assert_eq!(part2(input), 16312);
}
