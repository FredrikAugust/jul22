#[derive(Debug, Clone)]
enum Node {
    Directory(String, Vec<Node>),
    File(String, u64),
}

#[derive(Debug, Clone)]
enum Operation {
    Cd(String),
    Ls(Vec<Node>),
}

impl Node {
    fn from_str(line: &str) -> Self {
        let (head, tail) = line.split_once(" ").unwrap();

        match head {
            "dir" => Node::Directory(tail.to_string(), vec![]),
            _ => Node::File(tail.to_string(), head.parse().unwrap()),
        }
    }

    fn update(self: &mut Self, operations: Vec<Operation>) {
        let Some(head )= operations.first() else {
            return;
        };

        match head {
            Operation::Cd(dir) => match self {
                Node::Directory(_, children) => {
                    if dir == ".." {
                        return;
                    }

                    let child = children
                        .iter_mut()
                        .find(|child| match child {
                            Node::Directory(name, _) => name == dir,
                            _ => false,
                        })
                        .unwrap();

                    child.update(operations[1..].to_vec());
                }
                _ => panic!("Cannot update file"),
            },
            Operation::Ls(nodes) => match self {
                Node::Directory(_, children) => {
                    children.extend(nodes.clone());
                    self.update(operations[1..].to_vec());
                }
                _ => panic!("Cannot update file"),
            },
        }
    }
}

pub fn part1(input_string: &str) -> i32 {
    let input_parts = input_string
        .split("$ ")
        .filter(|s| !s.is_empty())
        .map(|line| {
            let lines = line.lines().collect::<Vec<_>>();

            let head = lines.first().unwrap();

            if head.starts_with("cd") {
                Operation::Cd(head[3..].to_string())
            } else {
                Operation::Ls(lines[1..].iter().map(|str| Node::from_str(str)).collect())
            }
        })
        .collect::<Vec<_>>();

    let mut tree = Node::Directory("/".to_owned(), vec![]);

    tree.update(input_parts);

    println!("{:#?}", tree);

    0
}

pub fn part2(input_string: &str) -> i32 {
    0
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
        assert_eq!(part1(TEST_INPUT), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
