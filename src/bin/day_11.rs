use crate::ProcessResult::{PairResult, SingletonResult};
use maplit::hashmap;
use std::collections::HashMap;

fn parse_number_string(input: &str) -> Vec<u128> {
    input.split(" ").map(|n| n.parse().unwrap()).collect()
}

enum ProcessResult {
    SingletonResult(u128),
    PairResult(u128, u128),
}

#[derive(PartialEq, Eq, Hash)]
struct ResultKey {
    i: u128,
    steps: usize,
}

fn process_number(n: u128) -> ProcessResult {
    if n == 0 {
        SingletonResult(1)
    } else {
        let n_str = format!("{}", n);

        if n_str.len() % 2 == 0 {
            let first_half = n_str[0..n_str.len() / 2].parse::<u128>().unwrap();
            let second_half = n_str[n_str.len() / 2..n_str.len()].parse::<u128>().unwrap();

            PairResult(first_half, second_half)
        } else {
            SingletonResult(n * 2024)
        }
    }
}

fn apply_part_one_step(input: &[u128]) -> Vec<u128> {
    input
        .iter()
        .flat_map(|n| match process_number(*n) {
            SingletonResult(i) => vec![i],
            PairResult(i, j) => vec![i, j],
        })
        .collect()
}

fn solve_part_1(input: &str, n: usize) -> usize {
    let mut result = parse_number_string(input);

    for _ in 0..n {
        result = apply_part_one_step(&result);
    }

    result.len()
}

fn recursive_solve(cache: &mut HashMap<ResultKey, usize>, i: u128, steps: usize) -> usize {
    if steps == 0 {
        return 1;
    }

    let key = ResultKey { i, steps };

    if let Some(r) = cache.get(&key) {
        return *r;
    }

    let result = match process_number(i) {
        SingletonResult(i) => recursive_solve(cache, i, steps - 1),
        PairResult(i, j) => {
            recursive_solve(cache, i, steps - 1) + recursive_solve(cache, j, steps - 1)
        }
    };

    cache.insert(key, result);

    result
}

fn solve_part_2(input: &str, n: usize) -> usize {
    let initial_list = parse_number_string(input);
    let mut cache = hashmap![];

    initial_list
        .iter()
        .map(|i| recursive_solve(&mut cache, *i, n))
        .sum()
}

fn main() {
    let input = include_str!("day_11_input.txt").trim();

    println!("part 1: {}", solve_part_1(input, 25));
    println!("part 2: {}", solve_part_2(input, 75));
}

#[cfg(test)]
mod tests {
    use crate::{apply_part_one_step, parse_number_string, solve_part_1, solve_part_2};

    const EXAMPLE_SEQUENCE: [&str; 7] = [
        "125 17",
        "253000 1 7",
        "253 0 2024 14168",
        "512072 1 20 24 28676032",
        "512 72 2024 2 0 2 4 2867 6032",
        "1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32",
        "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2",
    ];

    #[test]
    fn test_simple_example() {
        let parsed_input = EXAMPLE_SEQUENCE
            .iter()
            .map(|l| parse_number_string(l))
            .collect::<Vec<Vec<u128>>>();

        for i in 0..parsed_input.len() - 1 {
            let next = apply_part_one_step(&parsed_input[i]);

            assert_eq!(parsed_input[i + 1], next)
        }
    }

    #[test]
    fn test_solve_part_1() {
        let expected = parse_number_string(EXAMPLE_SEQUENCE[EXAMPLE_SEQUENCE.len() - 1]).len();

        assert_eq!(
            expected,
            solve_part_1(EXAMPLE_SEQUENCE[0], EXAMPLE_SEQUENCE.len() - 1)
        )
    }

    #[test]
    fn test_solve_part_2() {
        let expected = parse_number_string(EXAMPLE_SEQUENCE[EXAMPLE_SEQUENCE.len() - 1]).len();

        assert_eq!(
            expected,
            solve_part_2(EXAMPLE_SEQUENCE[0], EXAMPLE_SEQUENCE.len() - 1)
        )
    }
}
