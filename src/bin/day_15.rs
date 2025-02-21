use itertools::Itertools;
use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Vector {
    x: usize,
    y: usize,
}

impl Vector {
    fn new(x: usize, y: usize) -> Self {
        Vector { x, y }
    }

    fn moving(&self, direction: &Direction) -> Self {
        match direction {
            Direction::UP => Vector::new(self.x, self.y - 1),
            Direction::RIGHT => Vector::new(self.x + 1, self.y),
            Direction::DOWN => Vector::new(self.x, self.y + 1),
            Direction::LEFT => Vector::new(self.x - 1, self.y),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    WALL,
    SPACE,
    BOX,
}

#[derive(Clone)]
struct Room {
    robot: Vector,
    locations: Vec<Vec<Tile>>,
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (row, tiles) in self.locations.iter().enumerate() {
            for (column, tile) in tiles.iter().enumerate() {
                if column == self.robot.x && row == self.robot.y {
                    write!(f, "@")?
                } else {
                    match tile {
                        Tile::WALL => write!(f, "#")?,
                        Tile::SPACE => write!(f, ".")?,
                        Tile::BOX => write!(f, "O")?,
                    }
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

impl Room {
    fn gps_score(self) -> usize {
        self.locations
            .iter()
            .enumerate()
            .map(|(row, tiles)| {
                tiles
                    .iter()
                    .enumerate()
                    .map(|(column, tile)| {
                        if *tile == Tile::BOX {
                            100 * row + column
                        } else {
                            0usize
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }
    
    fn tile(&self, location: &Vector) -> Tile {
        self.locations[location.y][location.x]
    }
    
    fn set_tile(&mut self, location: &Vector, to: Tile) {
        self.locations[location.y][location.x] = to;
    }
    
    fn move_robot(&mut self, direction: &Direction) {
        let new_position = self.robot.moving(direction);
        
        fn update_state(r: &mut Room, p: &Vector, d: &Direction) -> bool {
            match r.tile(p) {
                Tile::WALL => false,
                Tile::SPACE => true,
                Tile::BOX => {
                    let new_position = p.moving(d);
                    
                    if update_state(r, &new_position, d) {
                        r.set_tile(&new_position, Tile::BOX);
                        true
                    } else {
                        false
                    }
                }
            }
        }
        
        if update_state(self, &new_position, direction) {
            self.set_tile(&new_position, Tile::SPACE);
            self.robot = new_position;
        }
    }
}

fn parse_input(input: &str) -> (Room, Vec<Direction>) {
    let (map_input, directions_input) = input.split("\n\n").collect_tuple().unwrap();

    let mut robot = Vector::new(0, 0);
    let locations = map_input
        .split("\n")
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(|(column, c)| match c {
                    '#' => Tile::WALL,
                    '.' => Tile::SPACE,
                    'O' => Tile::BOX,
                    '@' => {
                        robot.x = column;
                        robot.y = row;

                        Tile::SPACE
                    }
                    _ => panic!("unexpected char in input: {}", c),
                })
                .collect_vec()
        })
        .collect_vec();

    let directions = directions_input
        .chars()
        .flat_map(|c| match c {
            '^' => vec![Direction::UP],
            '>' => vec![Direction::RIGHT],
            'v' => vec![Direction::DOWN],
            '<' => vec![Direction::LEFT],
            _ => vec![],
        })
        .collect_vec();

    (Room { robot, locations }, directions)
}

fn solve_part_1(room: &Room, directions: &[Direction]) -> usize {
    let mut room = room.clone();
    
    for direction in directions {
        room.move_robot(direction);
    }

    room.gps_score()
}
fn main() {
    let (room, directions) = parse_input(include_str!("day_15_input.txt"));
    
    println!("part 1: {}", solve_part_1(&room, &directions))
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_part_1, Direction, Vector};

    const EXAMPLE: &str = "########\n\
        #..O.O.#\n\
        ##@.O..#\n\
        #...O..#\n\
        #.#.O..#\n\
        #...O..#\n\
        #......#\n\
        ########\n\
        \n\
        <^^>>>vv<v>>v<<\n";

    #[test]
    fn test_parse_input_on_example() {
        let (room, directions) = parse_input(EXAMPLE);

        assert_eq!(Vector::new(2, 2), room.robot);

        let map = format!("{}\n", EXAMPLE.split("\n\n").next().unwrap());
        assert_eq!(map, room.to_string());
        assert_eq!(
            vec![
                Direction::LEFT,
                Direction::UP,
                Direction::UP,
                Direction::RIGHT,
                Direction::RIGHT,
                Direction::RIGHT,
                Direction::DOWN,
                Direction::DOWN,
                Direction::LEFT,
                Direction::DOWN,
                Direction::RIGHT,
                Direction::RIGHT,
                Direction::DOWN,
                Direction::LEFT,
                Direction::LEFT,
            ],
            directions
        );
    }

    #[test]
    fn test_solve_part_1() {
        let (room, directions) = parse_input(EXAMPLE);

        assert_eq!(2028, solve_part_1(&room, &directions))
    }
}
