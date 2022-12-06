use std::collections::HashSet;

pub fn solve(input_string: &str, message_size: usize) -> usize {
    let mut buffer: HashSet<char> = HashSet::from_iter(input_string[0..message_size].chars());
    let mut index = message_size;

    while buffer.len() != message_size {
        index += 1;
        buffer.clear();
        buffer.extend(input_string[index - message_size..index].chars());
    }

    index
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", solve(input, 4));

    println!("Part 2: {}", solve(input, 14));
}

#[cfg(test)]
mod tests {
    use crate::solve;

    const TEST_INPUT: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";

    #[test]
    fn test_part1() {
        assert_eq!(solve(TEST_INPUT, 4), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(solve("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14), 29);
    }
}
