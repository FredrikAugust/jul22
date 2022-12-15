use std::collections::{HashSet, VecDeque};

type Position = (i32, i32);

pub fn parse_line(line_input: &str) -> HashSet<Position> {
    let points = line_input
        .split(" -> ")
        .map(|pair| {
            let (x, y) = pair.split_once(',').unwrap();

            (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut result: HashSet<Position> = HashSet::new();

    points.windows(2).for_each(|window| {
        let (x1, y1) = window[0];
        let (x2, y2) = window[1];

        if x1 == x2 {
            if y1 >= y2 {
                result.extend((y2..=y1).map(|y| (x1, y)));
            } else {
                result.extend((y1..=y2).map(|y| (x1, y)));
            }
        } else {
            if x1 >= x2 {
                result.extend((x2..=x1).map(|x| (x, y1)));
            } else {
                result.extend((x1..=x2).map(|x| (x, y1)));
            }
        }
    });

    result
}

pub fn parse_lines(lines_input: &str) -> HashSet<Position> {
    let individual_lines = lines_input.lines().map(parse_line);

    let mut final_result = HashSet::new();

    for line in individual_lines {
        final_result.extend(line);
    }

    final_result
}

fn valid_steps(
    position: &Position,
    settled: &HashSet<Position>,
    walls: &HashSet<Position>,
) -> Option<Position> {
    let prioritised_step_deltas: Vec<Position> = vec![(0, 1), (-1, 1), (1, 1)];

    let mut updated_proposed_positions = prioritised_step_deltas
        .iter()
        .map(|(delta_x, delta_y)| (position.0 + delta_x, position.1 + delta_y));

    updated_proposed_positions.find(|proposed_position| {
        walls.contains(proposed_position) == false && settled.contains(proposed_position) == false
    })
}

fn print(settled: &HashSet<Position>, walls: &HashSet<Position>, position: &Position) {
    for y in 0..10 {
        for x in 493..504 {
            let pos = &(x, y);

            if settled.contains(pos) {
                print!("o");
            } else if walls.contains(pos) {
                print!("#");
            } else if pos == position {
                print!("+");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn dfs(
    position: &Position,
    walls: &HashSet<Position>,
    settled: &mut HashSet<Position>,
    void_start_y_pos: i32,
) -> bool {
    if position.1 == void_start_y_pos {
        return true;
    }

    print(&settled, &walls, &position);

    while let Some(valid_move) = valid_steps(&position, &settled, &walls) {
        if dfs(&valid_move, &walls, settled, void_start_y_pos) {
            return true;
        }
    }

    settled.insert(*position);

    return false;
}

pub fn part1(input_string: &str) -> i32 {
    let walls = parse_lines(input_string);

    // Lowest wall is start of void
    let void_start_y_pos = walls
        .iter()
        .max_by(|(_, y1), (_, y2)| y1.cmp(y2))
        .unwrap()
        .1;

    println!("Void starts at y={}", void_start_y_pos);

    let start_position = (500, 0);

    let mut settled = HashSet::new();

    dfs(&start_position, &walls, &mut settled, void_start_y_pos);

    settled.len() as i32
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
    use std::collections::HashSet;

    use crate::{parse_line, parse_lines, part1, part2, Position};

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_parse_line() {
        let mut expected: HashSet<Position> = HashSet::new();

        expected.insert((498, 4));
        expected.insert((498, 5));
        expected.insert((498, 6));
        expected.insert((497, 6));
        expected.insert((496, 6));

        assert_eq!(parse_line("498,4 -> 498,6 -> 496,6"), expected);
    }

    #[test]
    fn test_parse_lines() {
        let mut expected: HashSet<Position> = HashSet::new();

        expected.insert((498, 4));
        expected.insert((498, 5));
        expected.insert((498, 6));
        expected.insert((497, 6));
        expected.insert((496, 6));
        expected.insert((1, 1));

        assert_eq!(
            parse_lines(
                "498,4 -> 498,6 -> 496,6
1,1 -> 1,1 -> 1,1"
            ),
            expected
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 24);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 93);
    }
}
