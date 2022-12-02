#[derive(PartialEq, Eq)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl Choice {
    pub fn from(textual_representation: &str) -> Self {
        match textual_representation {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissors,
            _ => panic!("Invalid choice"),
        }
    }

    pub fn outcome(&self, other: &Choice) -> i32 {
        if self == other {
            return 3;
        }

        match self {
            Choice::Rock => {
                if other == &Choice::Scissors {
                    6
                } else {
                    0
                }
            }
            Choice::Paper => {
                if other == &Choice::Rock {
                    6
                } else {
                    0
                }
            }
            Choice::Scissors => {
                if other == &Choice::Paper {
                    6
                } else {
                    0
                }
            }
        }
    }

    pub fn value(&self) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        }
    }
}

pub fn part2_get_choice_value_for_outcome(other_choice: &Choice, outcome: &str) -> i32 {
    match outcome {
        // lose
        "X" => match other_choice {
            Choice::Rock => Choice::Scissors.value(),
            Choice::Paper => Choice::Rock.value(),
            Choice::Scissors => Choice::Paper.value(),
        },
        // draw
        "Y" => other_choice.value(),
        // win
        "Z" => match other_choice {
            Choice::Rock => Choice::Paper.value(),
            Choice::Paper => Choice::Scissors.value(),
            Choice::Scissors => Choice::Rock.value(),
        },
        _ => panic!("Invalid outcome"),
    }
}

fn part1(strategy_guide: &str) -> i32 {
    let pairings = strategy_guide
        .lines()
        .map(|round| {
            let mut choices_split = round.split_whitespace();

            let other_choice = Choice::from(choices_split.next().unwrap());
            let my_choice = Choice::from(choices_split.next().unwrap());

            (other_choice, my_choice)
        })
        .collect::<Vec<(Choice, Choice)>>();

    pairings.iter().fold(0, |acc, (other, mine)| {
        acc + (mine.outcome(other) + mine.value())
    })
}

fn part2(strategy_guide: &str) -> i32 {
    strategy_guide.lines().fold(0, |acc, round| {
        let mut choices_split = round.split_whitespace();

        let other_choice = Choice::from(choices_split.next().unwrap());
        let desired_outcome_string = choices_split.next().unwrap();
        let my_choice_value =
            part2_get_choice_value_for_outcome(&other_choice, desired_outcome_string);
        let desired_outcome_value = match desired_outcome_string {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => panic!("Invalid outcome"),
        };

        acc + (desired_outcome_value + my_choice_value)
    })
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));

    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    const STRATEGY_GUIDE: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() {
        assert_eq!(part1(STRATEGY_GUIDE), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(STRATEGY_GUIDE), 12);
    }
}
