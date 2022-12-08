fn line_one_direction(line: &Vec<char>) -> Vec<bool> {
    let numbers = line.iter().map(|c| c.to_digit(10).unwrap() as i32);

    let mut max: i32 = -1;

    let mut result: Vec<bool> = vec![];

    for number in numbers {
        result.push(number > max);

        if number > max {
            max = number;
        }
    }

    result
}

pub fn line_both_directions(line: &Vec<char>) -> Vec<bool> {
    let west_to_east = line_one_direction(line);

    let mut reversed = line.clone();
    reversed.reverse();

    let mut east_to_west = line_one_direction(&reversed);
    east_to_west.reverse();

    west_to_east
        .into_iter()
        .zip(east_to_west.into_iter())
        .map(|(a, b)| a | b)
        .collect()
}

// Sub-optimal due to Copy-use
fn transpose<T>(input: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Copy,
{
    let mut result: Vec<Vec<T>> = vec![];

    for i in 0..input[0].len() {
        let mut row: Vec<T> = vec![];

        for j in 0..input.len() {
            row.push(input[j][i]);
        }

        result.push(row);
    }

    result
}

pub fn part1(input_string: &str) -> i32 {
    let rows = input_string
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let columns = transpose(&rows);

    let rows_result: Vec<Vec<bool>> = rows.into_iter().map(|r| line_both_directions(&r)).collect();

    let columns_result = columns.into_iter().map(|r| line_both_directions(&r));
    let transposed_columns_result = transpose(&columns_result.collect());

    let mut result = 0;

    for i in 0..rows_result.len() {
        for j in 0..rows_result[i].len() {
            if rows_result[i][j] | transposed_columns_result[i][j] {
                result += 1;
            }
        }
    }

    result
}

fn visible(item_under_consideration: &u32, line: &Vec<&u32>) -> u32 {
    if line.is_empty() {
        return 0;
    }

    let mut score = 0;

    for item in line {
        if *item < item_under_consideration {
            score += 1;
        } else if *item >= item_under_consideration {
            score += 1;
            break;
        } else {
            break;
        }
    }

    score
}

pub fn part2(input_string: &str) -> i32 {
    let rows = input_string
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>();

    let columns = transpose(&rows);

    // we know it's a square
    let len = rows.len();

    let mut max = 0;

    for i in 0..len {
        for j in 0..len {
            let current = &rows[i][j];

            // get all directions
            let right = rows[i][j + 1..].iter().collect::<Vec<_>>();

            let mut left = rows[i][..j].iter().collect::<Vec<_>>();
            left.reverse();

            let down = columns[j][i + 1..].iter().collect::<Vec<_>>();

            let mut up = columns[j][..i].iter().collect::<Vec<_>>();
            up.reverse();

            // count visible
            let right = visible(current, &right);
            let left = visible(current, &left);
            let down = visible(current, &down);
            let up = visible(current, &up);

            let product_of_visible = right * left * down * up;

            if product_of_visible > max {
                max = product_of_visible;
            }
        }
    }

    max as i32
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));

    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{line_both_directions, part1, part2};

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_line_both_dir() {
        assert_eq!(
            line_both_directions(&"123454321".chars().collect()),
            vec![true, true, true, true, true, true, true, true, true]
        );

        assert_eq!(
            line_both_directions(&"30373".chars().collect()),
            vec![true, false, false, true, true]
        );

        assert_eq!(
            line_both_directions(&"25512".chars().collect()),
            vec![true, true, true, false, true]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 8);
    }
}
