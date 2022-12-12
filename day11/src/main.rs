use std::borrow::BorrowMut;

#[derive(Debug)]
pub struct Monkey {
    items: Vec<i64>,
    operation: fn(i64) -> i64,
    test: fn(i64) -> bool,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn update_items_part1(&mut self) {
        for item in self.items.iter_mut() {
            *item = (self.operation)(*item) / 3;
        }
    }

    fn update_items_part2(&mut self) {
        // test
        // let gcd = 23 * 19 * 13 * 17;

        // prod
        let gcd = 5 * 17 * 7 * 13 * 19 * 3 * 11 * 2;

        for item in self.items.iter_mut() {
            *item = (self.operation)(*item) % gcd;
        }
    }
}

pub fn part1(monkeys: &mut Vec<Monkey>) -> i64 {
    let num_rounds = 20;

    let mut scores = vec![0; monkeys.len()];

    for _ in 0..num_rounds {
        for index in 0..monkeys.len() {
            let mut moves: Vec<(usize, i64)> = vec![];

            let monkey = &mut monkeys[index];
            monkey.update_items_part1();
            scores[index] += monkey.items.len() as i64;

            for item in monkey.items.iter_mut() {
                if (monkey.test)(*item) {
                    moves.push((monkey.true_target, *item));
                } else {
                    moves.push((monkey.false_target, *item));
                }
            }

            let _ = &mut monkeys[index].borrow_mut().items.clear();

            for (target, item) in moves {
                monkeys[target].items.push(item);
            }
        }
    }

    println!("{:?}", monkeys);

    scores.sort();
    scores.reverse();

    println!("{:?}", scores);

    scores.iter().take(2).product()
}

pub fn part2(monkeys: &mut Vec<Monkey>) -> i64 {
    let num_rounds = 10_000;

    let mut scores = vec![0; monkeys.len()];

    for _ in 0..num_rounds {
        for index in 0..monkeys.len() {
            let mut moves: Vec<(usize, i64)> = vec![];

            let monkey = &mut monkeys[index];
            monkey.update_items_part2();
            scores[index] += monkey.items.len() as i64;

            for item in monkey.items.iter_mut() {
                if (monkey.test)(*item) {
                    moves.push((monkey.true_target, *item));
                } else {
                    moves.push((monkey.false_target, *item));
                }
            }

            let _ = &mut monkeys[index].borrow_mut().items.clear();

            for (target, item) in moves {
                monkeys[target].items.push(item);
            }
        }
    }

    println!("{:?}", monkeys);

    scores.sort();
    scores.reverse();

    println!("{:?}", scores);

    scores.iter().take(2).product()
}

fn main() {
    println!(
        "Part 1: {}",
        part1(&mut vec![
            Monkey {
                items: vec![74, 64, 74, 63, 53],
                operation: |old| old * 7,
                test: |x| x % 5 == 0,
                true_target: 1,
                false_target: 6
            },
            Monkey {
                items: vec![69, 99, 95, 62],
                operation: |old| old * old,
                test: |x| x % 17 == 0,
                true_target: 2,
                false_target: 5
            },
            Monkey {
                items: vec![59, 81],
                operation: |old| old + 8,
                test: |x| x % 7 == 0,
                true_target: 4,
                false_target: 3
            },
            Monkey {
                items: vec![50, 67, 63, 57, 63, 83, 97],
                operation: |old| old + 4,
                test: |x| x % 13 == 0,
                true_target: 0,
                false_target: 7
            },
            Monkey {
                items: vec![61, 94, 85, 52, 81, 90, 94, 70],
                operation: |old| old + 3,
                test: |x| x % 19 == 0,
                true_target: 7,
                false_target: 3
            },
            Monkey {
                items: vec![69],
                operation: |old| old + 5,
                test: |x| x % 3 == 0,
                true_target: 4,
                false_target: 2
            },
            Monkey {
                items: vec![54, 55, 58],
                operation: |old| old + 7,
                test: |x| x % 11 == 0,
                true_target: 1,
                false_target: 5
            },
            Monkey {
                items: vec![79, 51, 83, 88, 93, 76],
                operation: |old| old * 3,
                test: |x| x % 2 == 0,
                true_target: 0,
                false_target: 6
            },
        ])
    );

    println!(
        "Part 2: {}",
        part2(&mut vec![
            Monkey {
                items: vec![74, 64, 74, 63, 53],
                operation: |old| old * 7,
                test: |x| x % 5 == 0,
                true_target: 1,
                false_target: 6
            },
            Monkey {
                items: vec![69, 99, 95, 62],
                operation: |old| old * old,
                test: |x| x % 17 == 0,
                true_target: 2,
                false_target: 5
            },
            Monkey {
                items: vec![59, 81],
                operation: |old| old + 8,
                test: |x| x % 7 == 0,
                true_target: 4,
                false_target: 3
            },
            Monkey {
                items: vec![50, 67, 63, 57, 63, 83, 97],
                operation: |old| old + 4,
                test: |x| x % 13 == 0,
                true_target: 0,
                false_target: 7
            },
            Monkey {
                items: vec![61, 94, 85, 52, 81, 90, 94, 70],
                operation: |old| old + 3,
                test: |x| x % 19 == 0,
                true_target: 7,
                false_target: 3
            },
            Monkey {
                items: vec![69],
                operation: |old| old + 5,
                test: |x| x % 3 == 0,
                true_target: 4,
                false_target: 2
            },
            Monkey {
                items: vec![54, 55, 58],
                operation: |old| old + 7,
                test: |x| x % 11 == 0,
                true_target: 1,
                false_target: 5
            },
            Monkey {
                items: vec![79, 51, 83, 88, 93, 76],
                operation: |old| old * 3,
                test: |x| x % 2 == 0,
                true_target: 0,
                false_target: 6
            },
        ])
    );
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Monkey};

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(&mut vec![
                Monkey {
                    items: vec![79, 98],
                    operation: |x| x * 19,
                    test: |x| x % 23 == 0,
                    true_target: 2,
                    false_target: 3,
                },
                Monkey {
                    items: vec![54, 65, 75, 74],
                    operation: |x| x + 6,
                    test: |x| x % 19 == 0,
                    true_target: 2,
                    false_target: 0,
                },
                Monkey {
                    items: vec![79, 60, 97],
                    operation: |x| x * x,
                    test: |x| x % 13 == 0,
                    true_target: 1,
                    false_target: 3,
                },
                Monkey {
                    items: vec![74],
                    operation: |x| x + 3,
                    test: |x| x % 17 == 0,
                    true_target: 0,
                    false_target: 1,
                },
            ]),
            10605
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(&mut vec![
                Monkey {
                    items: vec![79, 98],
                    operation: |x| x * 19,
                    test: |x| x % 23 == 0,
                    true_target: 2,
                    false_target: 3,
                },
                Monkey {
                    items: vec![54, 65, 75, 74],
                    operation: |x| x + 6,
                    test: |x| x % 19 == 0,
                    true_target: 2,
                    false_target: 0,
                },
                Monkey {
                    items: vec![79, 60, 97],
                    operation: |x| x * x,
                    test: |x| x % 13 == 0,
                    true_target: 1,
                    false_target: 3,
                },
                Monkey {
                    items: vec![74],
                    operation: |x| x + 3,
                    test: |x| x % 17 == 0,
                    true_target: 0,
                    false_target: 1,
                },
            ]),
            2713310158
        );
    }
}
