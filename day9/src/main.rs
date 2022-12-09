use std::collections::HashSet;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn execute_move(&mut self, next_move: Move) {
        match next_move {
            Move::Up(distance) => self.y += distance,
            Move::Down(distance) => self.y -= distance,
            Move::Left(distance) => self.x -= distance,
            Move::Right(distance) => self.x += distance,
        }
    }

    pub fn should_move(&self, other: Position) -> bool {
        (self.x - other.x).abs() > 1 || (self.y - other.y).abs() > 1
    }

    pub fn move_towards(&mut self, target: &Position) {
        if self.x != target.x && self.y != target.y {
            // move diagonally towards the target
            if self.x < target.x {
                self.x += 1;
            } else {
                self.x -= 1;
            }

            if self.y < target.y {
                self.y += 1;
            } else {
                self.y -= 1;
            }

            return;
        }

        if self.x < target.x {
            self.x += 1;
        } else if self.x > target.x {
            self.x -= 1;
        } else if self.y < target.y {
            self.y += 1;
        } else if self.y > target.y {
            self.y -= 1;
        }
    }
}

#[derive(Debug)]
enum Move {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Move {
    fn get_distance(&self) -> i32 {
        match self {
            Move::Up(distance) => *distance,
            Move::Down(distance) => *distance,
            Move::Left(distance) => *distance,
            Move::Right(distance) => *distance,
        }
    }

    fn single_step(&self) -> Self {
        match self {
            Move::Up(_) => Move::Up(1),
            Move::Down(_) => Move::Down(1),
            Move::Left(_) => Move::Left(1),
            Move::Right(_) => Move::Right(1),
        }
    }
}

impl Move {
    pub fn from_str(input: &str) -> Self {
        let (direction, distance) = input.split_once(' ').unwrap();

        let distance = distance.parse().unwrap();

        match direction {
            "R" => Move::Right(distance),
            "L" => Move::Left(distance),
            "U" => Move::Up(distance),
            "D" => Move::Down(distance),
            _ => panic!("Invalid direction"),
        }
    }
}

pub fn part1(input_string: &str) -> i32 {
    let mut head_position = Position { x: 0, y: 0 };
    let mut tail_position = Position { x: 0, y: 0 };

    let mut visited = HashSet::new();

    let moves = input_string
        .lines()
        .map(|line| Move::from_str(line))
        .collect::<Vec<_>>();

    visited.insert(tail_position);

    for next_move in moves {
        head_position.execute_move(next_move);

        loop {
            if !tail_position.should_move(head_position) {
                break;
            }

            tail_position.move_towards(&head_position);

            visited.insert(tail_position.clone());
        }
    }

    visited.len() as i32
}

pub fn part2(input_string: &str) -> i32 {
    let mut head_position = Position { x: 0, y: 0 };
    let mut tails = (0..9).map(|_| Position { x: 0, y: 0 }).collect::<Vec<_>>();

    let mut visited: HashSet<Position> = HashSet::new();

    let moves = input_string
        .lines()
        .map(|line| Move::from_str(line))
        .collect::<Vec<_>>();

    visited.insert(Position { x: 0, y: 0 });

    for next_move in moves {
        let distance = next_move.get_distance();

        for _ in 0..distance {
            head_position.execute_move(next_move.single_step());

            for index in 0..9 {
                let tail = tails.get(index).unwrap();

                let target = if index == 0 {
                    &head_position
                } else {
                    &tails[index - 1]
                }
                .clone();

                let should_move = tail.should_move(target);

                if !should_move {
                    break;
                }

                tails.get_mut(index).unwrap().move_towards(&target);

                if index == 8 {
                    visited.insert(tails[index]);
                }
            }
        }
    }

    visited.len() as i32
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));

    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const TEST_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 1);
        assert_eq!(part2(TEST_INPUT_2), 36);
    }
}
