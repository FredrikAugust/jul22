use std::collections::HashMap;

type Stacks = HashMap<i32, Vec<char>>;

pub fn stacks_from_text(textual_representation: &str, number_of_stacks: i32) -> Stacks {
    let lines = textual_representation.lines().collect::<Vec<_>>();

    let item_indices = (1..(3 * number_of_stacks + number_of_stacks - 1))
        .step_by(4)
        .collect::<Vec<_>>();

    let stack_lines = lines[0..lines.len() - 1]
        .iter()
        .map(|line| {
            item_indices
                .iter()
                .map(|index| line.chars().nth(*index as usize).unwrap())
                .collect::<Vec<_>>()
        })
        .rev()
        .collect::<Vec<_>>();

    let mut stacks = Stacks::new();

    for stack_line in stack_lines {
        for (index, character) in stack_line.iter().enumerate() {
            match character {
                ' ' => {}
                _ => {
                    let stack = stacks.entry((index + 1) as i32).or_insert(Vec::new());
                    stack.push(*character);
                }
            }
        }
    }

    stacks
}

struct Instruction {
    pub count: i32,

    pub from: i32,
    pub to: i32,
}

impl Instruction {
    pub fn from_text(text: &str) -> Instruction {
        let parts = text.split_whitespace().collect::<Vec<_>>();

        let count = parts[1].parse::<i32>().unwrap();
        let from = parts[3].parse::<i32>().unwrap();
        let to = parts[5].parse::<i32>().unwrap();

        Instruction { count, from, to }
    }
}

pub fn part1(input_string: &str, number_of_stacks: i32) -> String {
    let mut parts = input_string.split("\n\n");

    let stacks_string = parts.next().unwrap();
    let mut stacks = stacks_from_text(stacks_string, number_of_stacks);

    println!("{:?}", stacks);

    let instructions = parts
        .next()
        .unwrap()
        .lines()
        .map(Instruction::from_text)
        .collect::<Vec<_>>();

    for instruction in instructions {
        for _ in 0..instruction.count {
            let from = stacks.get_mut(&instruction.from).unwrap();
            let item = from.pop().unwrap();
            let to = stacks.get_mut(&instruction.to).unwrap();
            to.push(item);
        }
    }

    (1..=number_of_stacks)
        .into_iter()
        .map(|stack_number| stacks.get(&stack_number).unwrap().last().unwrap_or(&' '))
        .collect::<String>()
}

pub fn part2(input_string: &str, number_of_stacks: i32) -> String {
    let mut parts = input_string.split("\n\n");

    let stacks_string = parts.next().unwrap();
    let mut stacks = stacks_from_text(stacks_string, number_of_stacks);

    println!("{:?}", stacks);

    let instructions = parts
        .next()
        .unwrap()
        .lines()
        .map(Instruction::from_text)
        .collect::<Vec<_>>();

    for instruction in instructions {
        let mut buffer: Vec<char> = Vec::new();

        for _ in 0..instruction.count {
            let from = stacks.get_mut(&instruction.from).unwrap();
            let item = from.pop().unwrap();
            buffer.push(item);
        }

        buffer.reverse();

        for item in buffer {
            let to = stacks.get_mut(&instruction.to).unwrap();
            to.push(item);
        }
    }

    (1..=number_of_stacks)
        .into_iter()
        .map(|stack_number| stacks.get(&stack_number).unwrap().last().unwrap_or(&' '))
        .collect::<String>()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input, 9));

    println!("Part 2: {}", part2(input, 9));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT, 3), "CMZ".to_owned());
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT, 3), "MCD".to_owned());
    }
}
