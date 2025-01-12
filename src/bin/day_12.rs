use crate::Adjacent::{AdjacentEdge, AdjacentLocation};
use crate::Direction::{X, Y};
use itertools::Itertools;
use maplit::hashset;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Location {
    x: usize,
    y: usize,
}

impl Location {
    fn plus(&self, x: usize, y: usize) -> Self {
        Location {
            x: self.x + x,
            y: self.y + y,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    X,
    Y,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Edge {
    from: Location,
    direction: Direction,
}

impl Edge {
    fn between(lhs: &Location, rhs: &Location) -> Self {
        if lhs.x == rhs.x {
            if lhs.y < rhs.y && lhs.y == rhs.y - 1 {
                return Edge {
                    from: *rhs,
                    direction: X,
                };
            } else if lhs.y > rhs.y && lhs.y - 1 == rhs.y {
                return Edge {
                    from: *lhs,
                    direction: X,
                };
            }
        } else if lhs.y == rhs.y {
            if lhs.x < rhs.x && lhs.x == rhs.x - 1 {
                return Edge {
                    from: *rhs,
                    direction: Y,
                };
            } else if lhs.x > rhs.x && lhs.x - 1 == rhs.x {
                return Edge {
                    from: *lhs,
                    direction: Y,
                };
            }
        }

        panic!("Cannot create edge between {:?} and {:?}", lhs, rhs);
    }

    fn to(&self) -> Location {
        match self.direction {
            X => self.from.plus(1, 0),
            Y => self.from.plus(0, 1),
        }
    }

    fn new(from_x: usize, from_y: usize, direction: Direction) -> Self {
        Edge {
            from: Location {
                x: from_x,
                y: from_y,
            },
            direction,
        }
    }
}

struct Region {
    edges: Vec<Edge>,
    area: usize,
}

#[derive(Clone, Copy)]
enum Adjacent {
    AdjacentEdge(Edge),
    AdjacentLocation(Location),
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn adjacent_locations(x_size: usize, y_size: usize, location: &Location) -> Vec<Adjacent> {
    let mut result = Vec::with_capacity(4);

    if location.x > 0 {
        result.push(AdjacentLocation(Location {
            x: location.x - 1,
            y: location.y,
        }));
    } else {
        result.push(AdjacentEdge(Edge {
            from: *location,
            direction: Y,
        }))
    }
    if location.y > 0 {
        result.push(AdjacentLocation(Location {
            x: location.x,
            y: location.y - 1,
        }));
    } else {
        result.push(AdjacentEdge(Edge {
            from: *location,
            direction: X,
        }))
    }
    if location.x < x_size - 1 {
        result.push(AdjacentLocation(Location {
            x: location.x + 1,
            y: location.y,
        }));
    } else {
        result.push(AdjacentEdge(Edge {
            from: Location {
                x: location.x + 1,
                y: location.y,
            },
            direction: Y,
        }))
    }
    if location.y < y_size - 1 {
        result.push(AdjacentLocation(Location {
            x: location.x,
            y: location.y + 1,
        }));
    } else {
        result.push(AdjacentEdge(Edge {
            from: Location {
                x: location.x,
                y: location.y + 1,
            },
            direction: X,
        }))
    }

    result
}

fn find_region(
    visited: &mut HashSet<Location>,
    map: &[Vec<char>],
    location: &Location,
    region: &mut Region,
) {
    visited.insert(*location);
    region.area += 1;

    let area_id = map[location.y][location.x];

    for adjacent in adjacent_locations(map[0].len(), map.len(), location) {
        match adjacent {
            AdjacentEdge(e) => region.edges.push(e),
            AdjacentLocation(l) => {
                if map[l.y][l.x] == area_id {
                    if !visited.contains(&l) {
                        find_region(visited, map, &l, region);
                    }
                } else {
                    region.edges.push(Edge::between(location, &l));
                }
            }
        }
    }
}

fn calculate_part_1_region_cost(
    visited: &mut HashSet<Location>,
    map: &[Vec<char>],
    location: &Location,
) -> usize {
    let mut region = Region {
        edges: vec![],
        area: 0,
    };

    find_region(visited, map, location, &mut region);

    region.edges.len() * region.area
}

fn calculate_part_2_region_cost(
    visited: &mut HashSet<Location>,
    map: &[Vec<char>],
    location: &Location,
) -> usize {
    let mut region = Region {
        edges: vec![],
        area: 0,
    };

    find_region(visited, map, location, &mut region);

    count_sides(&region.edges) * region.area
}

fn count_sides(edges: &[Edge]) -> usize {
    let from = edges.iter().map(|e| (e.from, e)).into_group_map();

    let collapsed_count: usize = edges
        .iter()
        .map(|e| match from.get(&e.to()) {
            None => 0,
            Some(connected_edges) => {
                if connected_edges.len() == 1 && connected_edges[0].direction == e.direction {
                    1
                } else {
                    0
                }
            }
        })
        .sum();

    edges.len() - collapsed_count
}

fn solve_part_1(map: &[Vec<char>]) -> usize {
    let mut visited = hashset![];
    let mut result = 0;

    for x in 0..map[0].len() {
        for y in 0..map.len() {
            let location = Location { x, y };

            if !visited.contains(&location) {
                result += calculate_part_1_region_cost(&mut visited, map, &location)
            }
        }
    }

    result
}

fn solve_part_2(map: &[Vec<char>]) -> usize {
    let mut visited = hashset![];
    let mut result = 0;

    for x in 0..map[0].len() {
        for y in 0..map.len() {
            let location = Location { x, y };

            if !visited.contains(&location) {
                result += calculate_part_2_region_cost(&mut visited, map, &location)
            }
        }
    }

    result
}

fn main() {
    let map = parse_map(include_str!("day_12_input.txt"));

    println!("part 1: {}", solve_part_1(&map));
    println!("part 2: {}", solve_part_2(&map));
}

#[cfg(test)]
mod tests {
    use crate::Direction::{X, Y};
    use crate::{
        calculate_part_1_region_cost, count_sides, find_region, parse_map, solve_part_1,
        solve_part_2, Edge, Location, Region,
    };
    use maplit::hashset;
    use rstest::rstest;

    const SMALL_EXAMPLE: &str = "AAAA\n\
        BBCD\n\
        BBCC\n\
        EEEC\n";

    #[test]
    fn test_solve_part_1() {
        assert_eq!(140, solve_part_1(&parse_map(SMALL_EXAMPLE)));
    }

    #[rstest]
    #[case("A", 4)]
    #[case("AA", 12)]
    #[case("AA\nBB", 12)]
    #[case("AAA\nBBB", 24)]
    #[case("AAAA\nBBBB", 40)]
    fn test_calculate_part_1_region_cost(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(
            expected,
            calculate_part_1_region_cost(
                &mut hashset![],
                &parse_map(input),
                &Location { x: 0, y: 0 }
            )
        );
    }

    #[test]
    fn test_count_sides() {
        assert_eq!(
            4,
            count_sides(&vec![
                Edge::new(0, 0, X),
                Edge::new(0, 0, Y),
                Edge::new(0, 1, X),
                Edge::new(1, 0, Y),
            ])
        );
        assert_eq!(
            4,
            count_sides(&vec![
                Edge::new(0, 0, X),
                Edge::new(1, 0, X),
                Edge::new(0, 0, Y),
                Edge::new(0, 1, X),
                Edge::new(1, 1, X),
                Edge::new(2, 0, Y)
            ])
        );
        assert_eq!(
            4,
            count_sides(&vec![
                // top edge
                Edge::new(0, 0, X),
                Edge::new(1, 0, X),
                Edge::new(2, 0, X),
                Edge::new(3, 0, X),
                // bottom edge
                Edge::new(0, 1, X),
                Edge::new(1, 1, X),
                Edge::new(2, 1, X),
                Edge::new(3, 1, X),
                // left edge
                Edge::new(0, 0, Y),
                // right edge
                Edge::new(4, 0, Y)
            ])
        );
    }

    #[test]
    fn test_find_region() {
        let map = parse_map("A");
        let mut region = Region {
            edges: vec![],
            area: 0,
        };
        find_region(&mut hashset![], &map, &Location { x: 0, y: 0 }, &mut region);

        assert_eq!(4, region.edges.len());
        assert!(region.edges.contains(&Edge::new(0, 0, X)));
        assert!(region.edges.contains(&Edge::new(0, 0, Y)));
        assert!(region.edges.contains(&Edge::new(0, 1, X)));
        assert!(region.edges.contains(&Edge::new(1, 0, Y)));
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(80, solve_part_2(&parse_map(SMALL_EXAMPLE)))
    }
}
