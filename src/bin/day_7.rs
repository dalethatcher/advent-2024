use itertools::Itertools;

#[derive(Debug, PartialEq)]
struct Expression {
    numbers: Vec<i128>,
    result: i128,
}

fn parse_input(input: &str) -> Vec<Expression> {
    input
        .lines()
        .map(|line| {
            let (result_str, numbers_str) = line.split(": ").collect_tuple().unwrap();
            let numbers = numbers_str.split(" ").map(|n| n.parse().unwrap()).collect();

            Expression {
                numbers,
                result: result_str.parse().unwrap(),
            }
        })
        .collect()
}

fn result_match_part_1(expression: &Expression) -> i128 {
    fn check(acc: i128, remainder: &[i128], result: i128) -> i128 {
        match remainder {
            [] => acc,
            [first, rest @ ..] => {
                if check(acc + first, rest, result) == result
                    || check(acc * first, rest, result) == result
                {
                    result
                } else {
                    0
                }
            }
        }
    }

    check(
        expression.numbers[0],
        &expression.numbers[1..],
        expression.result,
    )
}

fn result_match_part_2(expression: &Expression) -> i128 {
    fn check(acc: i128, remainder: &[i128], result: i128) -> i128 {
        match remainder {
            [] => acc,
            [first, rest @ ..] => {
                if check(acc + first, rest, result) == result
                    || check(acc * first, rest, result) == result
                    || check(format!("{}{}", acc, first).parse().unwrap(), rest, result) == result
                {
                    result
                } else {
                    0
                }
            }
        }
    }

    check(
        expression.numbers[0],
        &expression.numbers[1..],
        expression.result,
    )
}

fn solve_part_1(input: &[Expression]) -> i128 {
    input.iter().map(|e| result_match_part_1(e)).sum()
}
fn solve_part_2(input: &[Expression]) -> i128 {
    input.iter().map(|e| result_match_part_2(e)).sum()
}

fn main() {
    let input = parse_input(include_str!("day_7_input.txt"));
    println!("part 1: {}", solve_part_1(&input));
    println!("part 2: {}", solve_part_2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_part_1, solve_part_2, Expression};

    const EXAMPLE_INPUT: &str = "190: 10 19\n\
        3267: 81 40 27\n\
        83: 17 5\n\
        156: 15 6\n\
        7290: 6 8 6 15\n\
        161011: 16 10 13\n\
        192: 17 8 14\n\
        21037: 9 7 18 13\n\
        292: 11 6 16 20\n";

    #[test]
    fn test_parse_input() {
        let input = "123: 4 5\n89: 0 1\n";
        let expected = vec![
            Expression {
                numbers: vec![4, 5],
                result: 123,
            },
            Expression {
                numbers: vec![0, 1],
                result: 89,
            },
        ];

        assert_eq!(expected, parse_input(input));
    }

    #[test]
    fn test_solve_part_1() {
        let input = parse_input(EXAMPLE_INPUT);
        assert_eq!(3749, solve_part_1(&input));
    }

    #[test]
    fn test_solve_part_2() {
        let input = parse_input(EXAMPLE_INPUT);
        assert_eq!(11387, solve_part_2(&input));
    }
}
