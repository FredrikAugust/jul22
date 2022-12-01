use std::{fs::File, io::Read};

fn solve_part_1(input: &str) -> i32 {
    let parts = input.split("\n\n");
    let segment_sums = parts.map(|part| {
        let numbers = part.lines().map(|line| line.parse::<i32>().unwrap());
        numbers.sum::<i32>()
    });

    segment_sums.max().unwrap()
}

fn solve_part_2(input: &str) -> i32 {
    let parts = input.split("\n\n");
    let mut segment_sums = parts
        .map(|part| {
            let numbers = part.lines().map(|line| line.parse::<i32>().unwrap());
            numbers.sum::<i32>()
        })
        .collect::<Vec<_>>();

    //sort desc
    segment_sums.sort_by(|a, b| b.cmp(a));

    segment_sums[..3].iter().sum()
}

fn main() {
    let mut input_string = String::new();
    File::open("src/input.txt")
        .unwrap()
        .read_to_string(&mut input_string)
        .unwrap();

    println!("Part 1: {}", solve_part_1(&input_string));

    println!("Part 2: {}", solve_part_2(&input_string));
}

#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};

    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part1() {
        assert_eq!(solve_part_1(TEST_INPUT), 24_000);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part_2(TEST_INPUT), 45_000);
    }
}
