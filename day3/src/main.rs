use std::collections::HashSet;

pub fn priority(item: char) -> i32 {
    match item {
        'a'..='z' => item as i32 - 'a' as i32 + 1,
        'A'..='Z' => item as i32 - 'A' as i32 + 27,
        _ => panic!("Invalid item: {}", item),
    }
}

pub fn part1(rucksack_content: &str) -> i32 {
    rucksack_content
        .lines()
        .map(|line| {
            let first_part = &line[0..(line.len() / 2)].chars().collect::<HashSet<char>>();
            let last_part = &line[(line.len() / 2)..].chars().collect::<HashSet<char>>();

            first_part
                .intersection(last_part)
                .map(|item| priority(*item))
                .sum::<i32>()
        })
        .sum()
}

pub fn part2(rucksack_content: &str) -> i32 {
    let binding = rucksack_content.lines().collect::<Vec<&str>>();
    let chunks = binding.chunks(3);

    chunks
        .map(|group| {
            let first_part = &group[0].chars().collect::<HashSet<char>>();
            let second_part = &group[1].chars().collect::<HashSet<char>>();
            let third_part = &group[2].chars().collect::<HashSet<char>>();

            let first_and_second_intersection = first_part
                .intersection(second_part)
                .map(|item| *item)
                .collect::<HashSet<char>>();

            first_and_second_intersection
                .intersection(third_part)
                .map(|item| priority(*item))
                .sum::<i32>()
        })
        .sum()
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));

    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, priority};

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_priority() {
        assert_eq!(16, priority('p'));
        assert_eq!(38, priority('L'));
        assert_eq!(42, priority('P'));
        assert_eq!(22, priority('v'));
        assert_eq!(20, priority('t'));
        assert_eq!(19, priority('s'));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 70);
    }
}
