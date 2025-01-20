use itertools::Itertools;
use std::ops::{Add, Mul};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Vector {
    x: i32,
    y: i32,
}

impl Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: i32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Vector {
    fn clamp(self, within: &Vector) -> Vector {
        fn clamp_int(max: i32, x: i32) -> i32 {
            let x = x % max;

            if x < 0 {
                x + max
            } else {
                x
            }
        }

        Vector {
            x: clamp_int(within.x, self.x),
            y: clamp_int(within.y, self.y),
        }
    }
}

struct Robot {
    position: Vector,
    velocity: Vector,
}

fn parse_input(input: &str) -> Vec<Robot> {
    fn parse_vector(vector: &str) -> Vector {
        let parts: Vec<i32> = vector
            .split(&[',', '='])
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect();

        Vector {
            x: parts[0],
            y: parts[1],
        }
    }

    input
        .lines()
        .map(|l| {
            let (position, velocity) = l.split(" ").collect_tuple().unwrap();

            Robot {
                position: parse_vector(position),
                velocity: parse_vector(velocity),
            }
        })
        .collect()
}

fn solve_part_1(bounds: &Vector, robots: &[Robot]) -> i32 {
    let (mx, my): (i32, i32) = (bounds.x / 2, bounds.y / 2);
    let final_positions = robots
        .iter()
        .map(|r| (r.position + r.velocity * 100).clamp(bounds))
        .collect::<Vec<Vector>>();

    let (mut ne, mut nw, mut sw, mut se): (i32, i32, i32, i32) = (0, 0, 0, 0);

    for v in final_positions {
        if v.x < mx {
            if v.y < my {
                nw += 1;
            } else if v.y > my {
                sw += 1;
            }
        } else if v.x > mx {
            if v.y < my {
                ne += 1;
            } else if v.y > my {
                se += 1;
            }
        }
    }

    ne * nw * sw * se
}

fn solve_part_2(bounds: &Vector, robots: &[Robot]) -> i32 {
    'next_second: for seconds in 1..i32::MAX {
        let mut positions = vec![false; (bounds.x * bounds.y) as usize];

        for p in robots {
            let position = (p.position + p.velocity * seconds).clamp(bounds);
            let i = position.x + position.y * bounds.x;

            if positions[i as usize] {
                continue 'next_second;
            } else {
                positions[i as usize] = true;
            }
        }

        return seconds;
    }

    -1
}

fn main() {
    let robots = parse_input(include_str!("day_14_input.txt"));

    println!(
        "part 1: {}",
        solve_part_1(&Vector { x: 101, y: 103 }, &robots)
    );
    println!(
        "part 2: {}",
        solve_part_2(&Vector { x: 101, y: 103 }, &robots)
    );
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_part_1, Vector};

    const SMALL_EXAMPLE: &str = "p=0,4 v=3,-3\n\
        p=6,3 v=-1,-3\n\
        p=10,3 v=-1,2\n\
        p=2,0 v=2,-1\n\
        p=0,0 v=1,3\n\
        p=3,0 v=-2,-2\n\
        p=7,6 v=-1,-3\n\
        p=3,0 v=-1,-2\n\
        p=9,3 v=2,3\n\
        p=7,3 v=-1,2\n\
        p=2,4 v=2,-3\n\
        p=9,5 v=-3,-3\n";

    #[test]
    fn test_parse_input_on_small_example() {
        let robots = parse_input(SMALL_EXAMPLE);
        let first_robot = &robots[0];

        assert_eq!(Vector { x: 0, y: 4 }, first_robot.position);
        assert_eq!(Vector { x: 3, y: -3 }, first_robot.velocity);
    }

    #[test]
    fn test_clamp() {
        assert_eq!(
            Vector { x: 9, y: 1 },
            Vector { x: -1, y: 6 }.clamp(&Vector { x: 10, y: 5 })
        )
    }

    #[test]
    fn test_solve_part_1() {
        let robots = parse_input(SMALL_EXAMPLE);

        assert_eq!(12, solve_part_1(&Vector { x: 11, y: 7 }, &robots))
    }
}
