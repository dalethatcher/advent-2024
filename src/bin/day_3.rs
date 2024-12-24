use regex::Regex;

fn solve_part_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(input)
        .map(|capture| {
            let (_, [lhs, rhs]) = capture.extract();

            lhs.parse::<i32>().unwrap() * rhs.parse::<i32>().unwrap()
        })
        .sum()
}

fn solve_part_2(input: &str) -> i32 {
    let re = Regex::new(r"((do)\(\))|((don't)\(\))|((mul)\((\d{1,3}),(\d{1,3})\))").unwrap();
    let mut enabled = true;
    let mut result = 0;

    for capture in re.captures_iter(input) {
        if let Some(_do_match) = capture.get(2) {
            enabled = true;
        } else if let Some(_dont_match) = capture.get(4) {
            enabled = false;
        } else if let Some(_mul_match) = capture.get(6) {
            if enabled {
                let lhs = capture.get(7).unwrap().as_str().parse::<i32>().unwrap();
                let rhs = capture.get(8).unwrap().as_str().parse::<i32>().unwrap();

                result += lhs * rhs;
            }
        }
    }

    result
}

fn main() {
    let input = include_str!("day_3_input.txt");

    println!("part 1: {}", solve_part_1(&input));
    println!("part 2: {}", solve_part_2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{solve_part_1, solve_part_2};
    use rstest::rstest;

    #[rstest]
    #[case("", 0)]
    #[case("mul(1,2)", 2)]
    #[case("mul(123,2345)", 0)]
    #[case(
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))",
        161
    )]
    fn test_solve_part_1(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(solve_part_1(input), expected);
    }

    #[rstest]
    #[case("", 0)]
    #[case(
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))",
        48
    )]
    fn test_solve_part_2(#[case] input: &str, #[case] expected: i32) {
        assert_eq!(solve_part_2(input), expected);
    }
}
