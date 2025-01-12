use itertools::Itertools;
use regex::Regex;
use std::cmp::PartialEq;
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: usize,
    y: usize,
}

impl Vector {
    fn is_zero(&self) -> bool {
        self.x == 0 && self.y == 0
    }
}

impl Mul<usize> for Vector {
    type Output = Vector;

    fn mul(self, scalar: usize) -> Self::Output {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Rem for Vector {
    type Output = Vector;

    fn rem(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl Div for Vector {
    type Output = Vector;

    fn div(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct Machine {
    button_a: Vector,
    button_b: Vector,
    prize: Vector,
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .map(|definition| {
            let (a, b, p) = definition.lines().collect_tuple().unwrap();

            Machine {
                button_a: parse_vector(a),
                button_b: parse_vector(b),
                prize: parse_vector(p),
            }
        })
        .collect()
}

fn parse_vector(line: &str) -> Vector {
    let vector_re = Regex::new(r"[^0-9]*([0-9]+)[^0-9]*([0-9]+).*").unwrap();
    let (_, [x, y]) = vector_re.captures(line).unwrap().extract();
    let x = x.parse().unwrap();
    let y = y.parse().unwrap();

    Vector { x, y }
}

fn find_cheapest(machine: &Machine) -> usize {
    // 3 tokens for A and 1 for B

    let mut cheapest_tokens = 0;
    for a_presses in 0..usize::MAX {
        let a_position = machine.button_a * a_presses;
        if a_position.x > machine.prize.x || a_position.y > machine.prize.y {
            break;
        }

        if a_position == machine.prize {
            let tokens = a_presses * 3;
            if cheapest_tokens == 0 || tokens < cheapest_tokens {
                cheapest_tokens = tokens;
            }
            continue;
        }

        let remainder = machine.prize - a_position;
        let b_mod = remainder % machine.button_b;

        if b_mod.is_zero() {
            let b_count = remainder / machine.button_b;

            if b_count.x == b_count.y {
                let tokens = a_presses * 3 + b_count.x;

                if cheapest_tokens == 0 || tokens < cheapest_tokens {
                    cheapest_tokens = tokens;
                }
            }
        }
    }

    cheapest_tokens
}

fn solve_part_1(machines: &[Machine]) -> usize {
    machines.iter().map(|m| find_cheapest(m)).sum()
}

fn solve_numerically(machine: &Machine) -> usize {
    let ax = machine.button_a.x as i128;
    let ay = machine.button_a.y as i128;
    let bx = machine.button_b.x as i128;
    let by = machine.button_b.y as i128;
    let px = machine.prize.x as i128;
    let py = machine.prize.y as i128;

    let a_numerator = by * px - bx * py;
    let a_denominator = ax * by - ay * bx;

    if a_denominator != 0 && a_numerator % a_denominator == 0 {
        let a = a_numerator / a_denominator;
        let b = (py - a * ay) / by;

        return (a * 3 + b) as usize;
    }

    0
}

fn solve_part_2(machines: &[Machine]) -> usize {
    machines
        .iter()
        .map(|m| {
            let updated_machine = Machine {
                button_a: m.button_a,
                button_b: m.button_b,
                prize: m.prize
                    + Vector {
                        x: 10000000000000,
                        y: 10000000000000,
                    },
            };

            solve_numerically(&updated_machine)
        })
        .sum()
}

fn main() {
    let machines = parse_input(include_str!("day_13_input.txt"));

    println!("part 1: {}", solve_part_1(&machines));
    println!("part 2: {}", solve_part_2(&machines));
}

#[cfg(test)]
mod tests {
    use crate::{find_cheapest, parse_input, solve_part_1, solve_part_2, Machine, Vector};

    const SMALL_EXAMPLE: &str = "Button A: X+94, Y+34\n\
        Button B: X+22, Y+67\n\
        Prize: X=8400, Y=5400\n\
        \n\
        Button A: X+26, Y+66\n\
        Button B: X+67, Y+21\n\
        Prize: X=12748, Y=12176\n\
        \n\
        Button A: X+17, Y+86\n\
        Button B: X+84, Y+37\n\
        Prize: X=7870, Y=6450\n\
        \n\
        Button A: X+69, Y+23\n\
        Button B: X+27, Y+71\n\
        Prize: X=18641, Y=10279\n";

    #[test]
    fn test_find_cheapest() {
        assert_eq!(
            280,
            find_cheapest(&Machine {
                button_a: Vector { x: 94, y: 34 },
                button_b: Vector { x: 22, y: 67 },
                prize: Vector { x: 8400, y: 5400 },
            })
        )
    }

    #[test]
    fn test_solve_part_1() {
        let machines = parse_input(SMALL_EXAMPLE);

        assert_eq!(480, solve_part_1(&machines))
    }

    #[test]
    fn test_solve_part_2() {
        let machines = parse_input(SMALL_EXAMPLE);

        assert_eq!(875318608908, solve_part_2(&machines))
    }
}
