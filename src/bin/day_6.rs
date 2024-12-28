use maplit::{hashmap, hashset};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone)]
enum Content {
    Empty,
    Object,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Location {
    row: usize,
    column: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<Content>>, Location) {
    let mut location = Location {
        row: usize::MAX,
        column: usize::MAX,
    };

    let map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(column, c)| match c {
                    '.' => Content::Empty,
                    '#' => Content::Object,
                    '^' => {
                        location.row = row;
                        location.column = column;
                        Content::Empty
                    }
                    _ => panic!("unexpected character: {}", c),
                })
                .collect()
        })
        .collect();

    (map, location)
}

// fn print_map(map: &Vec<Vec<Content>>, visited: &HashSet<Location>) {
//     for (row, row_content) in map.iter().enumerate() {
//         for (column, content) in row_content.iter().enumerate() {
//             if visited.contains(&Location {
//                 row: row,
//                 column: column,
//             }) {
//                 print!("X");
//             } else {
//                 match content {
//                     Content::Empty => print!("."),
//                     Content::Object => print!("#"),
//                 }
//             }
//         }
//         println!();
//     }
// }

fn visited_locations(map: &Vec<Vec<Content>>, start_location: &Location) -> (HashSet<Location>, bool) {
    let mut location: Location = start_location.clone();
    let mut direction = Direction::North;
    let row_len = map.len() as i32;
    let column_len = map[0].len() as i32;
    let mut visited = hashmap![location.clone() => hashset![direction.clone()]];

    loop {
        let (row, column):(i32, i32) = match direction {
            Direction::North => (location.row as i32 - 1, location.column as i32),
            Direction::East => (location.row as i32, location.column as i32 + 1),
            Direction::South => (location.row as i32 + 1, location.column as i32),
            Direction::West => (location.row as i32, location.column as i32 - 1),
        };

        if row < 0 || row >= row_len || column < 0 || column >= column_len {
            return (visited.keys().cloned().collect(), false);
        }
        let row = row as usize;
        let column = column as usize;

        match map[row][column] {
            Content::Empty => {
                location.row = row;
                location.column = column;
                
                match visited.get_mut(&location) {
                    Some(directions) => {
                        if directions.contains(&direction) {
                            return (visited.keys().cloned().collect(), true);
                        }
                        else {
                            directions.insert(direction.clone());
                        }
                    }
                    None => {
                        visited.insert(location.clone(), hashset![direction.clone()]);
                    }
                }
            }
            Content::Object => match direction {
                Direction::North => direction = Direction::East,
                Direction::East => direction = Direction::South,
                Direction::South => direction = Direction::West,
                Direction::West => direction = Direction::North,
            },
        }
    }
}

fn solve_part_1(map: &Vec<Vec<Content>>, start_location: &Location) -> usize {
    let (visited, _) = visited_locations(map, start_location);

    visited.len()
}

fn is_loop(map: &Vec<Vec<Content>>, start_location: &Location) -> bool {
    let (_, looped) = visited_locations(map, start_location);
    
    looped
}

fn solve_part_2(map: &Vec<Vec<Content>>, location: &Location) -> usize {
    let (obstacle_candidates, _) = visited_locations(map, location);
    let mut result = 0;
    
    for obstacle in obstacle_candidates {
        let mut map_copy = map.clone();
        map_copy[obstacle.row][obstacle.column] = Content::Object;
        if is_loop(&map_copy, location) {
            result += 1;
        }
    }
    
    result
}

fn main() {
    let input = include_str!("day_6_input.txt");
    let (parsed_input, location) = parse_input(input);

    println!("part 1: {}", solve_part_1(&parsed_input, &location));
    println!("part 2: {}", solve_part_2(&parsed_input, &location));
}

#[cfg(test)]
mod tests {
    use crate::Content::{Empty, Object};
    use crate::{parse_input, Location};

    const EXAMPLE_INPUT: &str = "....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...\n";
    #[test]
    fn test_parse_input() {
        let (map, location) = parse_input(".#.\n.^.\n");

        assert_eq!(
            map,
            vec![vec![Empty, Object, Empty], vec![Empty, Empty, Empty]]
        );
        assert_eq!(location, Location { row: 1, column: 1 });
    }

    #[test]
    fn test_solve_part_1() {
        let (map, location) = parse_input(EXAMPLE_INPUT);
        assert_eq!(super::solve_part_1(&map, &location), 41);
    }
    #[test]
    fn test_solve_part_2() {
        let (map, location) = parse_input(EXAMPLE_INPUT);
        assert_eq!(super::solve_part_2(&map, &location), 6);
    }
}
