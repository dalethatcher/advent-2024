use crate::CheckResults::{Correct, ErrorIndices};
use maplit::hashset;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq)]
enum CheckResults {
    Correct(i32),
    ErrorIndices(usize, usize),
}

fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let mut rules: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut pages = vec![];

    let parts = input.split("\n\n").collect::<Vec<&str>>();
    let (rules_part, pages_part) = (parts[0], parts[1]);

    for line in rules_part.lines() {
        let parts = line
            .split("|")
            .map(|p| p.parse().unwrap())
            .collect::<Vec<i32>>();
        let (before, after) = (parts[0], parts[1]);

        rules.entry(before).or_insert(hashset![]).insert(after);
    }

    for line in pages_part.lines() {
        let values = line.split(",").map(|i| i.parse().unwrap()).collect();
        pages.push(values);
    }

    (rules, pages)
}

fn order_check(rules: &HashMap<i32, HashSet<i32>>, pages: &Vec<i32>) -> CheckResults {
    let mut seen = HashMap::<i32, usize>::new();

    for i in 0..pages.len() {
        if let Some(afters) = rules.get(&pages[i]) {
            for after in afters {
                if let Some(index) = seen.get(after) {
                    return ErrorIndices(*index, i);
                }
            }
        }

        seen.insert(pages[i], i);
    }

    Correct(pages[pages.len() / 2])
}

fn solve_part_1(rules: &HashMap<i32, HashSet<i32>>, pages: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;

    for page in pages {
        match order_check(rules, page) {
            Correct(middle) => {
                result += middle;
            }
            ErrorIndices(_, _) => {}
        }
    }

    result
}

fn solve_part_2(rules: &HashMap<i32, HashSet<i32>>, pages: &Vec<Vec<i32>>) -> i32 {
    let mut result = 0;
    for page_order in pages {
        let mut candidate = page_order.clone();
        let mut reorder_required = false;

        loop {
            match order_check(rules, &candidate) {
                Correct(middle) => {
                    if reorder_required {
                        result += middle;
                    }
                    break;
                }
                ErrorIndices(index_1, index_2) => {
                    let temp = candidate[index_1];
                    candidate[index_1] = candidate[index_2];
                    candidate[index_2] = temp;

                    reorder_required = true;
                }
            }
        }
    }

    result
}

fn main() {
    let input = include_str!("day_5_input.txt");
    let (rules, pages) = parse_input(input);

    println!("part 1: {}", solve_part_1(&rules, &pages));
    println!("part 2: {}", solve_part_2(&rules, &pages));
}

#[cfg(test)]
mod tests {
    use crate::{order_check, parse_input, solve_part_1, solve_part_2, CheckResults};
    use maplit::{hashmap, hashset};
    use rstest::rstest;
    use std::collections::{HashMap, HashSet};

    const EXAMPLE_INPUT: &str = "47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47\n";

    #[test]
    fn test_parse_input() {
        let input = "1|2\n\
            3|2\n\
            3|4\n\
            \n\
            1,2,3\n\
            4,5,6\n";
        let expected_rules = hashmap! {
            1 => hashset![2],
            3 => hashset![2, 4],
        };
        let expected_pages = vec![vec![1, 2, 3], vec![4, 5, 6]];

        let (rules, pages) = parse_input(input);

        assert_eq!(expected_rules, rules);
        assert_eq!(expected_pages, pages);
    }

    #[rstest]
    #[case(hashmap![2=>hashset![3]], vec![1, 2, 3], CheckResults::Correct(2))]
    #[case(hashmap![3=>hashset![1]], vec![1, 2, 3], CheckResults::ErrorIndices(0, 2))]
    #[case(hashmap![], vec![1, 2, 3], CheckResults::Correct(2))]
    fn test_order_check(
        #[case] rules: HashMap<i32, HashSet<i32>>,
        #[case] pages: Vec<i32>,
        #[case] expected: CheckResults,
    ) {
        assert_eq!(expected, order_check(&rules, &pages));
    }

    #[test]
    fn test_solve_part_1() {
        let (rules, pages) = parse_input(EXAMPLE_INPUT);
        assert_eq!(143, solve_part_1(&rules, &pages));
    }

    #[test]
    fn test_solve_part_2() {
        let (rules, pages) = parse_input(EXAMPLE_INPUT);
        assert_eq!(123, solve_part_2(&rules, &pages));
    }
}
