use maplit::hashset;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Hash, Debug, Copy)]
struct Location {
    x: usize,
    y: usize,
}

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|n| n.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect()
}

fn extended_trail(trail: &[Location], location: &Location) -> Vec<Location> {
    let mut new_trail: Vec<Location> = trail.to_vec();
    new_trail.push(location.clone());

    new_trail
}

fn find_trails(map: &[Vec<u8>], trail: &[Location], location: Location) -> HashSet<Vec<Location>> {
    let new_trail = extended_trail(&trail, &location);
    if map[location.y][location.x] == 9 {
        return hashset![new_trail];
    }

    let mut result = hashset![];
    let next_target = map[location.y][location.x] + 1;

    if location.x > 0 && map[location.y][location.x - 1] == next_target {
        result.extend(find_trails(
            map,
            &new_trail,
            Location {
                x: location.x - 1,
                y: location.y,
            },
        ));
    }
    if location.y > 0 && map[location.y - 1][location.x] == next_target {
        result.extend(find_trails(
            map,
            &new_trail,
            Location {
                x: location.x,
                y: location.y - 1,
            },
        ));
    }
    if location.x < map[0].len() - 1 && map[location.y][location.x + 1] == next_target {
        result.extend(find_trails(
            map,
            &new_trail,
            Location {
                x: location.x + 1,
                y: location.y,
            },
        ));
    }
    if location.y < map.len() - 1 && map[location.y + 1][location.x] == next_target {
        result.extend(find_trails(
            map,
            &new_trail,
            Location {
                x: location.x,
                y: location.y + 1,
            },
        ));
    }

    result
}

fn trails(map: &[Vec<u8>], start: Location) -> HashSet<Vec<Location>> {
    find_trails(map, &vec![start.clone()], start.clone())
}

fn solve_part_1(map: &[Vec<u8>]) -> usize {
    let mut result = 0;

    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == 0 {
                let unique_destinations = trails(map, Location { x, y })
                    .into_iter()
                    .map(|t| t[t.len() - 1])
                    .collect::<HashSet<Location>>();

                result += unique_destinations.len()
            }
        }
    }

    result
}

fn solve_part_2(map: &[Vec<u8>]) -> usize {
    let mut result = 0;

    for x in 0..map[0].len() {
        for y in 0..map.len() {
            if map[y][x] == 0 {
                result += trails(map, Location { x, y }).len()
            }
        }
    }

    result
}

fn main() {
    let map = parse_input(include_str!("day_10_input.txt"));

    println!("part 1: {}", solve_part_1(&map));
    println!("part 2: {}", solve_part_2(&map));
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_part_1};
    use rstest::rstest;

    const SMALL_EXAMPLE: &str = "0123\n\
        1234\n\
        8765\n\
        9876\n";

    const LARGER_EXAMPLE: &str = "89010123\n\
        78121874\n\
        87430965\n\
        96549874\n\
        45678903\n\
        32019012\n\
        01329801\n\
        10456732\n";

    #[rstest]
    #[case(SMALL_EXAMPLE, 1)]
    #[case(LARGER_EXAMPLE, 36)]
    fn test_solve_part_1(#[case] input: &str, #[case] expected: usize) {
        {
            let map = parse_input(input);

            assert_eq!(expected, solve_part_1(&map))
        }
    }

    #[test]
    fn test_parse_input() {
        let map = parse_input(SMALL_EXAMPLE);
        let expected: Vec<Vec<u8>> = vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 3, 4],
            vec![8, 7, 6, 5],
            vec![9, 8, 7, 6],
        ];

        assert_eq!(expected, map);
    }
}
