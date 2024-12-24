fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut result = vec![];

    for line in input.lines() {
        let parts = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        result.push(parts);
    }

    result
}

fn remove_item_at_index(input: &[i32], index: usize) -> Vec<i32> {
    input
        .iter()
        .enumerate()
        .filter(|(i, _)| *i != index)
        .map(|(_, item)| *item)
        .collect()
}

fn is_safe(input: &[i32], can_remove: bool) -> bool {
    let increasing = input[0] < input[1];

    for index in 1..input.len() {
        let diff = input[index] - input[index - 1];

        if (increasing && (diff < 1 || diff > 3)) || (!increasing && (diff > -1 || diff < -3)) {
            if can_remove {
                return (index > 1 && is_safe(&remove_item_at_index(input, index - 2), false))
                    || is_safe(&remove_item_at_index(input, index - 1), false)
                    || is_safe(&remove_item_at_index(input, index), false);
            } else {
                return false;
            }
        }
    }

    true
}

fn solve_part_1(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|i| if is_safe(i, false) { 1 } else { 0 })
        .sum()
}

fn solve_part_2(input: &Vec<Vec<i32>>) -> i32 {
    input
        .iter()
        .map(|i| if is_safe(i, true) { 1 } else { 0 })
        .sum()
}

fn main() {
    let input = parse_input(include_str!("day_2_input.txt"));
    println!("loaded {} items", input.len());

    println!("part 1: {}", solve_part_1(&input));
    println!("part 2: {}", solve_part_2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{is_safe, parse_input, solve_part_1, solve_part_2};
    use rstest::rstest;

    #[test]
    fn test_parse_input() {
        let input = "1 2 3\n4 5 6";
        let expected = vec![vec![1, 2, 3], vec![4, 5, 6]];

        assert_eq!(expected, parse_input(input));
    }

    #[rstest]
    #[case(vec![1, 2], false, true)]
    #[case(vec![1, 3], false, true)]
    #[case(vec![1, 4], false, true)]
    #[case(vec![1, 5], false, false)]
    #[case(vec![1, 1], false, false)]
    #[case(vec![3, 2], false, true)]
    #[case(vec![1, 2, 4, 7], false, true)]
    #[case(vec![1, 2, 4, 7, 20], true, true)]
    #[case(vec![20, 1, 2, 4, 7], true, true)]
    #[case(vec![1, 20, 2, 4, 7 ], true, true)]
    #[case(vec![54, 55, 57, 59, 61], false, true)]
    #[case(vec![57, 54, 55, 57, 59, 61], true, true)]
    fn test_is_safe(#[case] input: Vec<i32>, #[case] can_remove: bool, #[case] expected: bool) {
        assert_eq!(expected, is_safe(&input, can_remove));
    }

    #[test]
    fn test_failing_case() {
        let input = vec![57, 54, 55, 57, 59, 61];
        assert_eq!(true, is_safe(&input, true));
    }

    #[test]
    fn test_solve_example() {
        let input =
            parse_input("7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9\n");

        assert_eq!(2, solve_part_1(&input));
        assert_eq!(4, solve_part_2(&input));
    }
}
