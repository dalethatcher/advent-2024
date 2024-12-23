use std::collections::HashMap;
use std::iter::zip;

fn solve_part_1(lhs_list: &[i32], rhs_list: &[i32]) -> i32 {
    let mut lhs_list_sorted = lhs_list.to_vec();
    lhs_list_sorted.sort();
    let mut rhs_list_sorted = rhs_list.to_vec();
    rhs_list_sorted.sort();

    let mut result = 0;
    for (lhs, rhs) in zip(lhs_list_sorted, rhs_list_sorted) {
        result += (lhs - rhs).abs();
    }

    result
}

fn solve_part_2(lhs_list: &[i32], rhs_list: &[i32]) -> i32 {
    let mut rhs_counts = HashMap::new();

    for i in rhs_list {
        let count = rhs_counts.entry(*i).or_insert(0);
        *count += 1;
    }

    let mut result = 0;

    for i in lhs_list {
        let rhs_count = rhs_counts.entry(*i).or_insert(0);
        result += i * *rhs_count;
    }

    result
}

fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut lhs_list = vec![];
    let mut rhs_list = vec![];

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        lhs_list.push(parts.next().unwrap().parse().unwrap());
        rhs_list.push(parts.next().unwrap().parse().unwrap());
    }

    (lhs_list, rhs_list)
}

fn main() {
    let input = include_str!("day_1_input.txt");
    let (lhs_list, rhs_list) = parse_input(&input);

    println!("part 1: {}", solve_part_1(&lhs_list, &rhs_list));
    println!("part 2: {}", solve_part_2(&lhs_list, &rhs_list));
}

#[cfg(test)]
mod tests {
    use crate::{parse_input, solve_part_1};

    #[test]
    fn test_solve_part_1() {
        assert_eq!(11, solve_part_1(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]));
    }

    #[test]
    fn test_parse() {
        let input = "1 2\n3 4";
        let (lhs_list, rhs_list) = parse_input(input);

        assert_eq!(vec![1, 3], lhs_list);
        assert_eq!(vec![2, 4], rhs_list);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(31, crate::solve_part_2(&vec![3, 4, 2, 1, 3, 3], &vec![4, 3, 5, 3, 9, 3]));
    }
}
