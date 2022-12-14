extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::{iterators::Pair, Parser};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct InputParser;

fn correctly_ordered_groups(list1: Pair<Rule>, list2: Pair<Rule>) -> Option<bool> {
    let mut inner_list1 = list1.into_inner();
    let mut inner_list2 = list2.into_inner();

    loop {
        let next_item_list1 = inner_list1.next();
        let next_item_list2 = inner_list2.next();

        if next_item_list1 == None && next_item_list2 == None {
            return None;
        }

        if next_item_list1 == None {
            return Some(true);
        }

        if next_item_list2 == None {
            return Some(false);
        }

        let next_item_list1 = next_item_list1.unwrap();
        let next_item_list2 = next_item_list2.unwrap();

        match (next_item_list1.as_rule(), next_item_list2.as_rule()) {
            (Rule::number, Rule::number) => {
                let num1 = next_item_list1.as_str().parse::<i32>().unwrap();
                let num2 = next_item_list2.as_str().parse::<i32>().unwrap();

                if num1 == num2 {
                    continue;
                }

                return Some(num1 < num2);
            }
            (Rule::list, Rule::list) => {
                match correctly_ordered_groups(next_item_list1, next_item_list2) {
                    None => continue,
                    Some(result) => return Some(result),
                }
            }
            (Rule::number, Rule::list) => {
                let num_str1 = next_item_list1.as_str();
                let formated_array_string = format!("[{}]", num_str1);
                let arrayified = InputParser::parse(Rule::list, formated_array_string.as_str())
                    .unwrap()
                    .next()
                    .unwrap();

                match correctly_ordered_groups(arrayified, next_item_list2) {
                    None => continue,
                    Some(result) => return Some(result),
                }
            }
            (Rule::list, Rule::number) => {
                let num_str2 = next_item_list2.as_str();
                let formated_array_string = format!("[{}]", num_str2);
                let arrayified = InputParser::parse(Rule::list, formated_array_string.as_str())
                    .unwrap()
                    .next()
                    .unwrap();

                match correctly_ordered_groups(next_item_list1, arrayified) {
                    None => continue,
                    Some(result) => return Some(result),
                }
            }
            (_, _) => continue,
        }
    }
}

pub fn part1(input_string: &str) -> i32 {
    let parse = InputParser::parse(Rule::input, input_string)
        .unwrap()
        .next()
        .unwrap();

    let mut count: i32 = 0;

    for (index, group) in parse.into_inner().enumerate() {
        match group.as_rule() {
            Rule::group => {
                let mut inner_group = group.into_inner();

                let list1 = inner_group.next().unwrap();
                let list2 = inner_group.next().unwrap();

                if correctly_ordered_groups(list1, list2) == Some(true) {
                    count += index as i32 + 1;
                }
            }
            Rule::EOI => break,
            _ => unreachable!(),
        }
    }

    count
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

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
