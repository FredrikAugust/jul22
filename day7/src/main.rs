#[derive(Debug)]
struct Directory {
    name: String,
    files: Vec<(String, i32)>,

    sub_directories: Vec<Directory>,
}

impl Directory {
    fn update_from_operations(&mut self, operations: Vec<Operation>) -> Vec<Operation> {
        let Some(head) = operations.first() else {
            return vec![];
        };

        match head {
            Operation::Cd(path) => {
                if path == ".." {
                    return operations[1..].to_vec();
                }

                let sub_directory = self
                    .sub_directories
                    .iter_mut()
                    .find(|dir| &dir.name == path)
                    .unwrap();

                let remaining_operations =
                    sub_directory.update_from_operations(operations[1..].to_vec());

                self.update_from_operations(remaining_operations)
            }
            Operation::Ls(children) => {
                for child in children {
                    match child {
                        Child::Directory(name) => {
                            let sub_dir = Directory {
                                name: name.clone(),
                                files: Vec::new(),
                                sub_directories: Vec::new(),
                            };
                            self.sub_directories.push(sub_dir);
                        }
                        Child::File(name, size) => {
                            self.files.push((name.clone(), *size));
                        }
                    }
                }

                self.update_from_operations(operations[1..].to_vec())
            }
        }
    }

    fn size(&self) -> i32 {
        self.files.iter().map(|(_, size)| size).sum::<i32>()
            + self
                .sub_directories
                .iter()
                .map(|dir| dir.size())
                .sum::<i32>()
    }

    fn size_of_directories_under_or_eq_size(&self, max_size: i32) -> i32 {
        let mut total = 0;

        if self.size() <= max_size {
            total += self.size();
        }

        for sub_dir in &self.sub_directories {
            total += sub_dir.size_of_directories_under_or_eq_size(max_size);
        }

        total
    }

    fn directories_larger_or_eq_than(&self, size: i32) -> Vec<i32> {
        let mut candidates: Vec<i32> = vec![];

        if self.size() >= size {
            candidates.push(self.size());
        }

        for sub_dir in &self.sub_directories {
            candidates.extend(sub_dir.directories_larger_or_eq_than(size));
        }

        candidates
    }
}

#[derive(Debug, Clone)]
enum Child {
    Directory(String),
    File(String, i32),
}

impl Child {
    fn from_str(str: &str) -> Child {
        let (head, tail) = str.split_once(' ').unwrap();

        match head {
            "dir" => Child::Directory(tail.to_string()),
            _ => Child::File(tail.to_string(), head.parse().unwrap()),
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Cd(String),
    Ls(Vec<Child>),
}

fn setup(input_string: &str) -> Directory {
    let input_parts = input_string
        .split("$ ")
        .filter(|s| !s.is_empty())
        .map(|line| {
            let lines = line.lines().collect::<Vec<_>>();

            let head = lines.first().unwrap();

            if head.starts_with("cd") {
                Operation::Cd(head[3..].to_string())
            } else {
                Operation::Ls(lines[1..].iter().map(|str| Child::from_str(str)).collect())
            }
        })
        .collect::<Vec<_>>();

    let mut tree = Directory {
        name: String::from("/"),
        files: Vec::new(),
        sub_directories: Vec::new(),
    };

    tree.update_from_operations(input_parts);

    tree
}

pub fn part1(input_string: &str) -> i32 {
    let dir = setup(input_string);

    dir.size_of_directories_under_or_eq_size(100_000)
}

pub fn part2(input_string: &str) -> i32 {
    let dir = setup(input_string);

    let mut candidates = dir.directories_larger_or_eq_than(30000000 - (70000000 - dir.size()));

    candidates.sort();

    *candidates.first().unwrap()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));

    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    // fjern første CD. den hardkoder vi
    const TEST_INPUT: &str = "$ ls
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
7214296 k";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 95437);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 24933642);
    }
}
