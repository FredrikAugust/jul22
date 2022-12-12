enum Operation {
    Noop,
    AddX(i32),
}

impl Operation {
    fn cycle_count(&self) -> i32 {
        match self {
            Operation::Noop => 1,
            Operation::AddX(_) => 2,
        }
    }

    fn from_str(input: &str) -> Self {
        let operation = &input[0..4];

        match operation {
            "noop" => Operation::Noop,
            "addx" => Operation::AddX(input[5..].parse().unwrap()),
            _ => panic!("Unknown operation: {}", operation),
        }
    }
}

struct State {
    clock: i32,

    register_x: i32,
}

impl State {
    fn new() -> Self {
        State {
            clock: 0,
            register_x: 1,
        }
    }

    fn execute(&mut self, operation: &Operation) {
        match operation {
            Operation::Noop => {}
            Operation::AddX(value) => {
                self.register_x += value;
            }
        }
    }
}

pub fn part1(input_string: &str) -> i32 {
    let cycle_checkpoints = vec![20, 60, 100, 140, 180, 220];

    let operations = input_string
        .lines()
        .map(Operation::from_str)
        .collect::<Vec<_>>();

    let mut state = State::new();

    let mut result_buffer = vec![];

    for operation in operations {
        let cycle_count = operation.cycle_count();

        for _ in 0..cycle_count {
            state.clock += 1;

            if cycle_checkpoints.contains(&state.clock) {
                result_buffer.push(state.register_x * state.clock);
            }
        }

        state.execute(&operation);
    }

    result_buffer.iter().sum()
}

pub fn part2(input_string: &str) {
    let operations = input_string
        .lines()
        .map(Operation::from_str)
        .collect::<Vec<_>>();

    let mut state = State::new();

    let mut result_buffer = [false; 40 * 6];

    for operation in operations {
        let sprite_positions = vec![state.register_x - 1, state.register_x, state.register_x + 1];

        for _ in 0..operation.cycle_count() {
            let y_position = state.clock / 40;

            let crt_position = state.clock - y_position * 40;

            if sprite_positions.contains(&crt_position) {
                result_buffer[state.clock as usize] = true;
            }

            state.clock += 1;
        }

        state.execute(&operation);
    }

    for (index, value) in result_buffer.iter().enumerate() {
        if index % 40 == 0 {
            println!();
        }

        if *value {
            print!("#");
        } else {
            print!(".");
        }
    }
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));

    println!("Part 2:");
    part2(input);
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13140);
    }

    #[test]
    fn test_part2() {
        part2(TEST_INPUT);
    }
}
