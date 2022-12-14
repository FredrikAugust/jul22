extern crate pest;
#[macro_use]
extern crate pest_derive;

use std::cmp::Ordering;

use pest::{iterators::Pair, Parser};

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    List(Vec<Packet>),
    Signal(i32),
}

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct InputParser;

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use Packet::{List, Signal};

        match (self, other) {
            (Signal(sig1), Signal(sig2)) => {
                if sig1 == sig2 {
                    return None;
                }

                if sig1 > sig2 {
                    return Some(Ordering::Greater);
                }

                return Some(Ordering::Less);
            }
            (List(packets1), List(packets2)) => {
                let mut items1 = packets1.clone().iter();
                let mut items2 = packets2.clone().iter();

                loop {
                    let next_item1 = items1.next();
                    let next_item2 = items2.next();

                    match (next_item1, next_item2) {
                        (None, None) => return None,
                        (Some(_), None) => return Some(Ordering::Greater),
                        (None, Some(_)) => return Some(Ordering::Less),
                        (Some(val1), Some(val2)) => match val1.partial_cmp(val2) {
                            None => continue,
                            value => return value,
                        },
                    }
                }
            }
            (list1, Packet::Signal(signal2)) => {
                return list1.partial_cmp(&Packet::List(vec![Packet::Signal(*signal2)]))
            }
            (Packet::Signal(signal1), list2) => {
                return Packet::List(vec![Packet::Signal(*signal1)]).partial_cmp(list2)
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn list_to_packet(list: Pair<Rule>) -> Vec<Packet> {
    let mut packets: Vec<Packet> = vec![];

    let mut pairs_list = list.into_inner();

    for element in pairs_list {
        match element.as_rule() {
            Rule::number => packets.push(Packet::Signal(element.as_str().parse::<i32>().unwrap())),
            Rule::list => packets.push(Packet::List(list_to_packet(element))),
            _ => unreachable!(),
        }
    }

    packets
}

pub fn part2(input_string: &str) -> i32 {
    let parse = InputParser::parse(Rule::input, input_string)
        .unwrap()
        .next()
        .unwrap();

    let mut count: i32 = 0;

    let mut packets: Vec<Packet> = Vec::new();

    for (index, list) in parse.into_inner().enumerate() {
        match list.as_rule() {
            Rule::list => {
                // println!("{:?}", list);
                packets.push(Packet::List(list_to_packet(list)));
            }
            Rule::EOI => break,
            _ => unreachable!(),
        }
    }

    packets.sort();

    let first_marker = packets
        .iter()
        .enumerate()
        .find(|(_, val)| **val == Packet::List(vec![Packet::List(vec![Packet::Signal(2)])]))
        .unwrap()
        .0;

    let second_marker = packets
        .iter()
        .enumerate()
        .find(|(_, val)| **val == Packet::List(vec![Packet::List(vec![Packet::Signal(6)])]))
        .unwrap()
        .0;

    return ((first_marker + 1) * (second_marker + 1)) as i32;
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::part2;

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
[1,[2,[3,[4,[5,6,0]]]],8,9]
[[2]]
[[6]]";

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 140);
    }
}
