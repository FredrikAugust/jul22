use std::{collections::HashSet, ops::RangeInclusive};

type Position = (i32, i32);

fn manhattan_distance((x1, y1): Position, (x2, y2): Position) -> i32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[derive(Debug)]
pub struct Sensor {
    pub sensor_position: Position,
    pub closest_beacon_position: Position,
}

impl Sensor {
    pub fn from_tuples(positions: (Position, Position)) -> Self {
        Self {
            sensor_position: positions.0,
            closest_beacon_position: positions.1,
        }
    }

    fn sensor_radius(&self) -> i32 {
        manhattan_distance(self.sensor_position, self.closest_beacon_position)
    }

    fn distance_to(&self, to: &Position) -> i32 {
        manhattan_distance(self.sensor_position, *to)
    }

    pub fn span_for_y(&self, y: i32) -> RangeInclusive<i32> {
        let distance_from_center = (self.sensor_position.1 - y).abs();

        let side_width = self.sensor_radius() - distance_from_center;

        (self.sensor_position.0 - side_width)..=(self.sensor_position.0 + side_width)
    }

    pub fn rightmost_point_for_y(&self, y: i32) -> i32 {
        let distance_from_center = (self.sensor_position.1 - y).abs();

        let side_width = self.sensor_radius() - distance_from_center;

        self.sensor_position.0 + side_width
    }
}

// --------------------------------------------------------------------------------

pub fn part1(input: &Vec<(Position, Position)>, y: i32) -> i32 {
    let sensors = input.clone().into_iter().map(Sensor::from_tuples);

    let mut positions: HashSet<i32> = HashSet::new();

    for sensor in sensors {
        for x in sensor.span_for_y(y) {
            if x == sensor.closest_beacon_position.0 && y == sensor.closest_beacon_position.1 {
                continue;
            }

            positions.insert(x);
        }
    }

    positions.len() as i32
}

pub fn part2(input: &Vec<(Position, Position)>, max_xy: i32) -> i64 {
    let factor: i64 = 4_000_000;

    let sensors: Vec<Sensor> = input.clone().into_iter().map(Sensor::from_tuples).collect();

    // -------------

    for sensor1 in sensors.iter().clone() {
        for sensor2 in sensors.iter().clone() {
            if sensor1.distance_to(&sensor2.sensor_position)
                == sensor1.sensor_radius() + sensor2.sensor_radius() + 2
            {
                println!("{:?}\n{:?}", sensor1, sensor2);
            }
        }
    }

    // med disse punktene kunne du tegnet radius med manhattan distance og så sett hvor det ene
    // punktet alle klemte inne var og der er svaret
    // det løser oppgaven på noen ms

    // -------------

    let radii = sensors
        .iter()
        .map(|sensor| sensor.sensor_radius())
        .collect::<Vec<_>>();

    let mut pos = (0, 0);

    loop {
        let Some(in_range_of) = sensors
            .iter()
            .enumerate()
            .find(|(index, sensor)| sensor.distance_to(&pos) <= radii[*index]) else {
                break;
            };

        let new_x = in_range_of.1.rightmost_point_for_y(pos.1) + 1;

        if new_x > max_xy {
            pos = (0, pos.1 + 1);
        } else {
            pos.0 = new_x;
        }
    }

    println!("{:?}", pos);

    (pos.0 as i64) * factor + (pos.1 as i64)
}

fn main() {
    let input = vec![
        ((2765643, 3042538), (2474133, 3521072)),
        ((2745662, 2324735), (2491341, 1883354)),
        ((2015742, 2904055), (2474133, 3521072)),
        ((3375262, 3203288), (3321219, 3415236)),
        ((3276468, 3892409), (3321219, 3415236)),
        ((952573, 3147055), (-41010, 2905006)),
        ((1823659, 1779343), (1592718, 2000000)),
        ((1156328, 865741), (1592718, 2000000)),
        ((3938443, 271482), (4081274, 1177185)),
        ((2815232, 1641178), (2491341, 1883354)),
        ((3984799, 3424711), (3321219, 3415236)),
        ((1658825, 3999931), (2474133, 3521072)),
        ((3199859, 1285962), (4081274, 1177185)),
        ((3538649, 2788193), (3725736, 2414539)),
        ((3522208, 3336284), (3321219, 3415236)),
        ((3093758, 3492396), (3321219, 3415236)),
        ((2464979, 562119), (2491341, 1883354)),
        ((3665010, 1556840), (3735739, 2128164)),
        ((207525, 3893957), (-41010, 2905006)),
        ((3894678, 1974599), (3735739, 2128164)),
        ((2185146, 3822275), (2474133, 3521072)),
        ((31166, 1467978), (-41010, 2905006)),
        ((3242364, 3335961), (3321219, 3415236)),
        ((3773718, 3999789), (3321219, 3415236)),
        ((423046, 2227938), (-41010, 2905006)),
        ((1600225, 2529059), (1592718, 2000000)),
        ((3291752, 2241389), (3735739, 2128164)),
        ((2741333, 3984346), (2474133, 3521072)),
        ((3935288, 2292902), (3725736, 2414539)),
        ((291635, 140996), (212146, -1154950)),
        ((3966296, 2600346), (3725736, 2414539)),
        ((2228916, 1461096), (2491341, 1883354)),
    ];

    // println!("Part 1: {}", part1(&input, 2000000));

    println!("Part 2: {}", part2(&input, 4000000));
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, Position, Sensor};

    #[test]
    fn test_span() {
        assert_eq!(
            Sensor::from_tuples(((8, 7), (2, 10))).span_for_y(14),
            6..=10
        );
    }

    #[test]
    fn test_part1() {
        let test_input: Vec<(Position, Position)> = vec![
            ((2, 18), (-2, 15)),
            ((9, 16), (10, 16)),
            ((13, 2), (15, 3)),
            ((12, 14), (10, 16)),
            ((10, 20), (10, 16)),
            ((14, 17), (10, 16)),
            ((8, 7), (2, 10)),
            ((2, 0), (2, 10)),
            ((0, 11), (2, 10)),
            ((20, 14), (25, 17)),
            ((17, 20), (21, 22)),
            ((16, 7), (15, 3)),
            ((14, 3), (15, 3)),
            ((20, 1), (15, 3)),
        ];

        assert_eq!(part1(&test_input, 10), 26);
    }

    #[test]
    fn test_part2() {
        let test_input: Vec<(Position, Position)> = vec![
            ((2, 18), (-2, 15)),
            ((9, 16), (10, 16)),
            ((13, 2), (15, 3)),
            ((12, 14), (10, 16)),
            ((10, 20), (10, 16)),
            ((14, 17), (10, 16)),
            ((8, 7), (2, 10)),
            ((2, 0), (2, 10)),
            ((0, 11), (2, 10)),
            ((20, 14), (25, 17)),
            ((17, 20), (21, 22)),
            ((16, 7), (15, 3)),
            ((14, 3), (15, 3)),
            ((20, 1), (15, 3)),
        ];

        assert_eq!(part2(&test_input, 20), 56000011);
    }
}
