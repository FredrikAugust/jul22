use std::ops::Range;

fn parse_assignment_string(assignment_string: &str) -> Range<i32> {
    let mut parts = assignment_string.split('-');
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();
    start..end
}

fn is_range_subset_of_other_range(range: &Range<i32>, other_range: &Range<i32>) -> bool {
    range.start >= other_range.start && range.end <= other_range.end
}

pub fn part1(assignments_string: &str) -> i32 {
    let assignments = assignments_string.lines().map(|line| {
        let tasks = line.split_once(",").unwrap();

        (
            parse_assignment_string(tasks.0),
            parse_assignment_string(tasks.1),
        )
    });

    assignments
        .map(|(assigment1, assignment2)| {
            is_range_subset_of_other_range(&assigment1, &assignment2)
                || is_range_subset_of_other_range(&assignment2, &assigment1)
        })
        .filter(|x| x == &true)
        .count()
        .try_into()
        .unwrap()
}

pub fn part2(assignments_string: &str) -> i32 {
    let assignments = assignments_string.lines().map(|line| {
        let tasks = line.split_once(",").unwrap();

        (
            parse_assignment_string(tasks.0),
            parse_assignment_string(tasks.1),
        )
    });

    assignments
        .map(|(assignment1, assignment2)| {
            (assignment1.end >= assignment2.start && assignment2.end >= assignment1.start)
                || (assignment2.end >= assignment1.start && assignment1.end >= assignment2.start)
        })
        .filter(|x| x == &true)
        .count()
        .try_into()
        .unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));

    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 4);
    }
}
