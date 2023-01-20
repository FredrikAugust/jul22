use std::collections::HashMap;

pub struct Valve {
    name: String,
    flow_rate: i32,
    tunnels_lead_to: Vec<String>,
}

pub fn part1(valves: Vec<Valve>) -> i32 {
    let mut distances_between_vertices: HashMap<(String, String), i32> = HashMap::new();

    // initialise floyd-warshall distance matrix
    for valve in &valves {
        for leads_to in &valve.tunnels_lead_to {
            distances_between_vertices.insert((valve.name.clone(), leads_to.into()), 1);
        }

        // distance to self is 0
        distances_between_vertices.insert((valve.name.clone(), valve.name.clone()), 0);
    }

    for k in &valves {
        for i in &valves {
            for j in &valves {
                let ij = &(i.name.clone(), j.name.clone());
                let ik = &(i.name.clone(), k.name.clone());
                let kj = &(k.name.clone(), j.name.clone());

                let ij_distance = distances_between_vertices
                    .get(ij)
                    .unwrap_or(&(std::i16::MAX as i32));
                let ik_distance = distances_between_vertices
                    .get(ik)
                    .unwrap_or(&(std::i16::MAX as i32));
                let kj_distance = distances_between_vertices
                    .get(kj)
                    .unwrap_or(&(std::i16::MAX as i32));

                distances_between_vertices.insert(
                    ij.to_owned(),
                    std::cmp::min(ij_distance.to_owned(), ik_distance + kj_distance),
                );
            }
        }
    }

    for (k, v) in distances_between_vertices.iter() {
        println!("{:?}: {}", k, v);
    }

    0
}

pub fn part2(input_string: &str) -> i32 {
    0
}

fn main() {
    let input = include_str!("input.txt");

    // println!("Part 1: {}", part1(input));

    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Valve};

    #[test]
    fn test_part1() {
        let test_input: Vec<Valve> = vec![
            Valve {
                name: "AA".to_owned(),
                flow_rate: 0,
                tunnels_lead_to: vec!["DD".to_owned(), "II".to_owned(), "BB".to_owned()],
            },
            Valve {
                name: "BB".to_owned(),
                flow_rate: 13,
                tunnels_lead_to: vec!["CC".to_owned(), "AA".to_owned()],
            },
            Valve {
                name: "CC".to_owned(),
                flow_rate: 2,
                tunnels_lead_to: vec!["DD".to_owned(), "BB".to_owned()],
            },
            Valve {
                name: "DD".to_owned(),
                flow_rate: 20,
                tunnels_lead_to: vec!["CC".to_owned(), "AA".to_owned(), "EE".to_owned()],
            },
            Valve {
                name: "EE".to_owned(),
                flow_rate: 3,
                tunnels_lead_to: vec!["FF".to_owned(), "DD".to_owned()],
            },
            Valve {
                name: "FF".to_owned(),
                flow_rate: 0,
                tunnels_lead_to: vec!["EE".to_owned(), "GG".to_owned()],
            },
            Valve {
                name: "GG".to_owned(),
                flow_rate: 0,
                tunnels_lead_to: vec!["FF".to_owned(), "HH".to_owned()],
            },
            Valve {
                name: "HH".to_owned(),
                flow_rate: 22,
                tunnels_lead_to: vec!["GG".to_owned()],
            },
            Valve {
                name: "II".to_owned(),
                flow_rate: 0,
                tunnels_lead_to: vec!["AA".to_owned(), "JJ".to_owned()],
            },
            Valve {
                name: "JJ".to_owned(),
                flow_rate: 21,
                tunnels_lead_to: vec!["II".to_owned()],
            },
        ];
        assert_eq!(part1(test_input), 0);
    }

    #[test]
    fn test_part2() {
        // assert_eq!(part2(TEST_INPUT), 0);
    }
}
