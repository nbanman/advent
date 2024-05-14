use advent::prelude::*;

fn parse_input(input: &str) -> HashMap<String, u32> {
    let mut path = Vec::new();
    let mut directories = HashMap::new();

    for line in input.lines() {
        if let Some((first, remainder)) = line.split_once(' ') {
            if first == "$" {
                if let Some((_, directory_name)) = remainder.split_once(' ') {
                    if directory_name == ".." {
                        path.pop();
                    } else {
                        path.push(directory_name);
                    }
                }
            } else if let Ok(file_size) = first.parse::<u32>() {
                let mut path_name = String::new();
                path.iter().for_each(|folder| {
                    path_name.push_str(folder);
                    let current_file_size = directories.entry(String::from(path_name.clone())).or_insert(0);
                    *current_file_size += file_size;
                })
            }
        }
    }
    directories
}

fn default_input() -> HashMap<String, u32> { parse_input(include_input!(2022 / 07)) }

fn part1(directories: HashMap<String, u32>) -> u32 {
    directories.values().filter(|file_size| file_size <= &&100_000).sum()
}

fn part2(directories: HashMap<String, u32>) -> u32 {
    let space_available = 70_000_000 - directories.get(&String::from("/")).unwrap();
    let min_dir_size = 30_000_000 - space_available;

    *directories.values().filter(|file_size| file_size >= &&min_dir_size).min().unwrap()
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn example() {
    let input = parse_input(
        "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k",
    );
    assert_eq!(part1(input.clone()), 95437);
    assert_eq!(part2(input), 24933642);
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1477771);
    assert_eq!(part2(input), 3579501);
}
