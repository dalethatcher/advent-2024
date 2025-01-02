use maplit::{hashmap, hashset};
use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Vector {
    fn is_within(&self, v: &Vector) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < v.x && self.y < v.y
    }
}

#[derive(Debug, PartialEq)]
struct Map {
    size: Vector,
    antennas: HashMap<char, Vec<Vector>>,
}

fn parse_input(input: &str) -> Map {
    let mut antennas = hashmap![];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                antennas.entry(c).or_insert(vec![]).push(Vector {
                    x: x as i32,
                    y: y as i32,
                })
            }
        }
    }

    Map {
        size: Vector {
            x: input.lines().next().unwrap().len() as i32,
            y: input.lines().count() as i32,
        },
        antennas,
    }
}

fn compute_part_1_locations(
    size: &Vector,
    antenna_1: &Vector,
    antenna_2: &Vector,
) -> HashSet<Vector> {
    let difference = antenna_2 - antenna_1;

    [antenna_1 - &difference, antenna_2 + &difference]
        .into_iter()
        .filter(|l| l.is_within(size))
        .collect()
}

fn solve_part_1(map: &Map) -> usize {
    let mut locations = hashset![];

    for antenna_group in map.antennas.values() {
        for (from_i, antenna_1) in antenna_group[..antenna_group.len() - 1].iter().enumerate() {
            for antenna_2 in antenna_group[from_i + 1..].iter() {
                locations.extend(compute_part_1_locations(&map.size, antenna_1, antenna_2));
            }
        }
    }

    locations.len()
}

fn compute_part_2_locations(
    size: &Vector,
    antenna_1: &Vector,
    antenna_2: &Vector,
) -> HashSet<Vector> {
    let mut result = hashset![];
    let difference = antenna_2 - antenna_1;

    let mut l = antenna_1.clone();
    while l.is_within(size) {
        result.insert(l.clone());
        l = &l - &difference;
    }

    let mut l = antenna_2.clone();
    while l.is_within(size) {
        result.insert(l.clone());
        l = &l + &difference;
    }

    result
}

fn solve_part_2(map: &Map) -> usize {
    let mut locations = hashset![];

    for antenna_group in map.antennas.values() {
        for (from_i, antenna_1) in antenna_group[..antenna_group.len() - 1].iter().enumerate() {
            for antenna_2 in antenna_group[from_i + 1..].iter() {
                locations.extend(compute_part_2_locations(&map.size, antenna_1, antenna_2));
            }
        }
    }

    locations.len()
}

fn main() {
    let input = include_str!("day_8_input.txt");
    let map = parse_input(input);

    println!("part 1: {}", solve_part_1(&map));
    println!("part 2: {}", solve_part_2(&map));
}

#[cfg(test)]
mod tests {
    use crate::{compute_part_1_locations, parse_input, solve_part_1, solve_part_2, Map, Vector};
    use maplit::{hashmap, hashset};
    use rstest::rstest;
    use std::collections::HashSet;

    const EXAMPLE_INPUT: &str = "............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............\n";

    #[test]
    fn test_parse_input() {
        let expected_map = Map {
            size: Vector { x: 12, y: 12 },
            antennas: hashmap! {
                '0' => vec![
                    Vector{x: 8, y: 1},
                    Vector{x: 5, y: 2},
                    Vector{x: 7, y: 3},
                    Vector{x: 4, y: 4},
                ],
                'A' => vec![
                    Vector{x: 6, y: 5},
                    Vector{x: 8, y: 8},
                    Vector{x: 9, y: 9},
                ],
            },
        };

        assert_eq!(expected_map, parse_input(EXAMPLE_INPUT));
    }

    #[test]
    fn test_solve_part_1() {
        let map = parse_input(EXAMPLE_INPUT);

        assert_eq!(14, solve_part_1(&map));
    }

    #[test]
    fn test_solve_part_2() {
        let map = parse_input(EXAMPLE_INPUT);

        assert_eq!(34, solve_part_2(&map));
    }

    #[rstest]
    #[case(Vector{x: 1, y: 1}, Vector{x: 2, y: 2}, hashset![Vector{x: 0, y: 0}, Vector{x:3, y: 3}])]
    #[case(Vector{x: 0, y: 0}, Vector{x: 4, y: 4}, hashset![])]
    fn test_compute_locations(
        #[case] antenna_1: Vector,
        #[case] antenna_2: Vector,
        #[case] expectation: HashSet<Vector>,
    ) {
        assert_eq!(
            expectation,
            compute_part_1_locations(&Vector { x: 5, y: 5 }, &antenna_1, &antenna_2)
        )
    }

    #[test]
    fn test_vector_add() {
        let lhs = Vector { x: 1, y: 2 };
        let rhs = Vector { x: 3, y: 4 };
        let sum = Vector { x: 4, y: 6 };

        assert_eq!(sum, &lhs + &rhs)
    }

    #[test]
    fn test_vector_subtract() {
        let lhs = Vector { x: 4, y: 6 };
        let rhs = Vector { x: 1, y: 1 };
        let sum = Vector { x: 3, y: 5 };

        assert_eq!(sum, &lhs - &rhs)
    }
}
