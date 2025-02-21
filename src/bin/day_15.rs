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
            Direction::Up => Vector::new(self.x, self.y - 1),
            Direction::Right => Vector::new(self.x + 1, self.y),
            Direction::Down => Vector::new(self.x, self.y + 1),
            Direction::Left => Vector::new(self.x - 1, self.y),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Tile {
    Wall,
    Space,
    Box,
    WideBoxL,
    WideBoxR,
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
                        Tile::Wall => write!(f, "#")?,
                        Tile::Space => write!(f, ".")?,
                        Tile::Box => write!(f, "O")?,
                        Tile::WideBoxL => write!(f, "[")?,
                        Tile::WideBoxR => write!(f, "]")?,
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
                        if *tile == Tile::Box || *tile == Tile::WideBoxL {
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
        fn update_state(r: &mut Room, p: &Vector, d: &Direction, dry_run: bool) -> bool {
            let tile = r.tile(p);

            if tile == Tile::Wall {
                false
            } else if tile == Tile::Space {
                true
            } else if tile == Tile::Box {
                let new_position = p.moving(d);

                if update_state(r, &new_position, d, dry_run) {
                    if !dry_run {
                        r.set_tile(&new_position, Tile::Box);
                        r.set_tile(p, Tile::Space);
                    }
                    true
                } else {
                    false
                }
            } else if tile == Tile::WideBoxL || tile == Tile::WideBoxR {
                if *d == Direction::Right || *d == Direction::Left {
                    if update_state(r, &p.moving(d), d, dry_run) {
                        if !dry_run {
                            r.set_tile(&p.moving(d), tile);
                            r.set_tile(p, Tile::Space);
                        }

                        true
                    } else {
                        false
                    }
                } else {
                    let lhs_p = if tile == Tile::WideBoxL {
                        *p
                    } else {
                        p.moving(&Direction::Left)
                    };
                    let rhs_p = if tile == Tile::WideBoxR {
                        *p
                    } else {
                        p.moving(&Direction::Right)
                    };
                    let lhs_np = lhs_p.moving(d);
                    let rhs_np = rhs_p.moving(d);

                    if update_state(r, &lhs_np, d, dry_run) && update_state(r, &rhs_np, d, dry_run)
                    {
                        if !dry_run {
                            r.set_tile(&lhs_p, Tile::Space);
                            r.set_tile(&rhs_p, Tile::Space);
                            r.set_tile(&lhs_np, Tile::WideBoxL);
                            r.set_tile(&rhs_np, Tile::WideBoxR);
                        }
                        true
                    } else {
                        false
                    }
                }
            } else {
                panic!("all cases not covered!")
            }
        }

        let new_position = self.robot.moving(direction);
        if update_state(self, &new_position, direction, true) {
            update_state(self, &new_position, direction, false);
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
                    '#' => Tile::Wall,
                    '.' => Tile::Space,
                    'O' => Tile::Box,
                    '@' => {
                        robot.x = column;
                        robot.y = row;

                        Tile::Space
                    }
                    _ => panic!("unexpected char in input: {}", c),
                })
                .collect_vec()
        })
        .collect_vec();

    let directions = directions_input
        .chars()
        .flat_map(|c| match c {
            '^' => vec![Direction::Up],
            '>' => vec![Direction::Right],
            'v' => vec![Direction::Down],
            '<' => vec![Direction::Left],
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

fn create_wide(room: &Room) -> Room {
    let locations = room
        .locations
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|t| match t {
                    Tile::Wall => vec![Tile::Wall, Tile::Wall],
                    Tile::Space => vec![Tile::Space, Tile::Space],
                    Tile::Box => vec![Tile::WideBoxL, Tile::WideBoxR],
                    _ => panic!("can't convert this type to wide: {:?}", t),
                })
                .collect_vec()
        })
        .collect_vec();

    Room {
        robot: Vector::new(room.robot.x * 2, room.robot.y),
        locations,
    }
}

fn solve_part_2(room: &Room, directions: &[Direction]) -> usize {
    let mut wide_room = create_wide(room);

    for direction in directions {
        wide_room.move_robot(direction);
    }

    wide_room.gps_score()
}

fn main() {
    let (room, directions) = parse_input(include_str!("day_15_input.txt"));

    println!("part 1: {}", solve_part_1(&room, &directions));
    println!("part 2: {}", solve_part_2(&room, &directions));
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_part_1, solve_part_2, Direction, Vector};

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
                Direction::Left,
                Direction::Up,
                Direction::Up,
                Direction::Right,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Down,
                Direction::Left,
                Direction::Down,
                Direction::Right,
                Direction::Right,
                Direction::Down,
                Direction::Left,
                Direction::Left,
            ],
            directions
        );
    }

    #[test]
    fn test_solve_part_1() {
        let (room, directions) = parse_input(EXAMPLE);

        assert_eq!(2028, solve_part_1(&room, &directions))
    }

    const PART_2_EXAMPLE: &str = "#######\n\
        #...#.#\n\
        #.....#\n\
        #..OO@#\n\
        #..O..#\n\
        #.....#\n\
        #######\n\
        \n\
        <vv<<^^<<^^\n";

    #[test]
    fn test_solve_part_2() {
        let (room, directions) = parse_input(PART_2_EXAMPLE);

        assert_eq!(618, solve_part_2(&room, &directions))
    }

    const LARGER_EXAMPLE: &str = "##########\n\
        #..O..O.O#\n\
        #......O.#\n\
        #.OO..O.O#\n\
        #..O@..O.#\n\
        #O#..O...#\n\
        #O..O..O.#\n\
        #.OO.O.OO#\n\
        #....O...#\n\
        ##########\n\
        \n\
        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n";

    #[test]
    fn test_solve_part_2_larger_example() {
        let (room, directions) = parse_input(LARGER_EXAMPLE);

        assert_eq!(9021, solve_part_2(&room, &directions));
    }
}
