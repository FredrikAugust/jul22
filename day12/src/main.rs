use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Start,
    End,
    Path(i32),
}

impl Tile {
    fn from_str(character: char) -> Tile {
        match character {
            'S' => Tile::Start,
            'E' => Tile::End, // height = z
            _ => Tile::Path(character as i32 - 'a' as i32),
        }
    }

    fn height(self) -> i32 {
        match self {
            Tile::Start => 0,
            Tile::End => 'z' as i32 - 'a' as i32,
            Tile::Path(height) => height,
        }
    }

    fn valid_next_step(self, neighbour: &Tile) -> bool {
        neighbour.height() <= self.height() + 1
    }
}

type MapMatrix = Vec<Vec<((i32, i32), Tile)>>;

fn generate_matrix(input_string: &str) -> MapMatrix {
    let lines = input_string.lines();

    lines
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, character)| ((x as i32, y as i32), Tile::from_str(character)))
                .collect()
        })
        .collect()
}

pub fn neighbours_part2(
    ((x, y), current_tile): &((i32, i32), Tile),
    visited: &HashSet<(i32, i32)>,
    map: &MapMatrix,
) -> Vec<((i32, i32), Tile)> {
    let neighbour_positions: Vec<(i32, i32)> =
        vec![(*x, y - 1), (*x, y + 1), (x - 1, *y), (x + 1, *y)];

    neighbour_positions
        .iter()
        .filter_map(|&neighbour| {
            if neighbour.0 < 0 || neighbour.1 < 0 {
                return None;
            };

            let row = map.get(neighbour.1 as usize)?;
            let tile = row.get(neighbour.0 as usize)?;

            if visited.contains(&neighbour) {
                return None;
            }

            if tile.1.height() + 1 >= current_tile.height() {
                return Some(tile.to_owned());
            }

            None
        })
        .collect()
}

pub fn neighbours(
    ((x, y), current_tile): &((i32, i32), Tile),
    visited: &HashSet<(i32, i32)>,
    map: &MapMatrix,
) -> Vec<((i32, i32), Tile)> {
    let neighbour_positions: Vec<(i32, i32)> =
        vec![(*x, y - 1), (*x, y + 1), (x - 1, *y), (x + 1, *y)];

    neighbour_positions
        .iter()
        .filter_map(|&neighbour| {
            if neighbour.0 < 0 || neighbour.1 < 0 {
                return None;
            };

            let row = map.get(neighbour.1 as usize)?;
            let tile = row.get(neighbour.0 as usize)?;

            if visited.contains(&neighbour) {
                return None;
            }

            if !current_tile.valid_next_step(&tile.1) {
                return None;
            }

            Some(tile.to_owned())
        })
        .collect()
}

pub fn distance_to_end(
    start_pos: &((i32, i32), Tile),
    map: &MapMatrix,
    visited: &HashSet<(i32, i32)>,
) -> Option<i32> {
    let valid_moves = neighbours(start_pos, visited, map);

    if valid_moves.is_empty() {
        return None;
    }

    if valid_moves
        .iter()
        .any(|valid_move| valid_move.1 == Tile::End)
    {
        return Some(1);
    }

    let child_distances = valid_moves.iter().map(|valid_move| {
        let mut updated_visited = HashSet::new();
        updated_visited.extend(visited);
        updated_visited.insert(valid_move.0);

        distance_to_end(valid_move, map, &updated_visited)
    });

    child_distances
        .filter(Option::is_some)
        .map(Option::unwrap)
        .min()
        .and_then(|val| Some(val + 1))
}

pub fn solve(map: &MapMatrix, start_pos: (i32, i32)) -> i32 {
    let flattened_map = map.iter().flatten().collect::<Vec<_>>();

    let mut visited = HashSet::new();
    visited.insert(start_pos);

    // distance_to_end(start_pos, &map, &visited).expect("No result")

    let mut parents: Vec<((i32, i32), (i32, i32))> = Vec::new();

    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back(start_pos);

    while !queue.is_empty() {
        let vertex = queue.pop_front().unwrap();
        let tile = *flattened_map.iter().find(|item| item.0 == vertex).unwrap();

        if tile.1 == Tile::End {
            break;
            // done
        }

        let neighbours = neighbours(tile, &visited, &map);

        for (pos, _) in neighbours {
            if visited.contains(&pos) {
                continue;
            }

            visited.insert(pos);
            parents.push((vertex, pos));
            queue.push_back(pos);
        }
    }

    let mut length = 0;

    // start at end and backtrack
    let mut pos = map
        .iter()
        .flatten()
        .find(|&tile| tile.1 == Tile::End)
        .unwrap()
        .0;

    loop {
        let Some(next_step) = parents.iter().find(|(_, to)| to == &pos) else {
            break;
        };

        length += 1;

        pos = next_step.0;
    }

    length
}

pub fn part1(input_string: &str) -> i32 {
    let map = generate_matrix(input_string);
    let start_pos = map
        .iter()
        .flatten()
        .find(|&tile| tile.1 == Tile::Start)
        .unwrap()
        .0;

    solve(&map, start_pos)
}

pub fn part2(input_string: &str) -> i32 {
    let map = generate_matrix(input_string);

    let flattened_map = map.iter().flatten().collect::<Vec<_>>();

    let start_pos = map
        .iter()
        .flatten()
        .find(|&tile| tile.1 == Tile::End)
        .unwrap()
        .0;

    let mut visited = HashSet::new();
    visited.insert(start_pos);

    println!("Start pos: {:?}", start_pos);

    let mut parents: Vec<((i32, i32), (i32, i32))> = Vec::new();

    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    queue.push_back(start_pos);

    while !queue.is_empty() {
        let vertex = queue.pop_front().unwrap();
        let tile = *flattened_map.iter().find(|item| item.0 == vertex).unwrap();

        if tile.1 == Tile::Path(0) {
            println!("Actually found solution");
            break;
            // done
        }

        let neighbours = neighbours_part2(tile, &visited, &map);

        for (pos, _) in neighbours {
            if visited.contains(&pos) {
                continue;
            }

            visited.insert(pos);
            parents.push((vertex, pos));
            queue.push_back(pos);
        }
    }

    let mut length = 0;

    let mut pos = parents
        .iter()
        .find(|&(_, to)| map.iter().flatten().find(|x| x.0 == *to).unwrap().1 == Tile::Path(0))
        .unwrap()
        .0;

    loop {
        let Some(next_step) = parents.iter().find(|(_, to)| to == &pos) else {
            break;
        };

        length += 1;

        pos = next_step.0;
    }

    length + 1
}

fn main() {
    let input = include_str!("input.txt");

    println!("Part 1: {}", part1(input));

    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{distance_to_end, generate_matrix, neighbours, part1, part2, Tile};

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_neighbours() {
        let map = generate_matrix(
            "SabcE
bbc",
        );
        let mut visited = HashSet::new();
        visited.insert((0, 0));
        visited.insert((1, 0));
        let result = neighbours(&((2, 0), Tile::from_str('b')), &visited, &map);

        println!("{:?}", result);

        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_run() {
        let map = generate_matrix("SabcdefghijklmnopqrstuvwxyzE");

        let mut visited = HashSet::new();
        visited.insert((0, 0));

        assert_eq!(
            distance_to_end(&((0, 0), Tile::Start), &map, &visited),
            Some(27)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 29);
    }
}
