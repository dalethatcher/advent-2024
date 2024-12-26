fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn check_char(lines: &[&str], row: i32, column: i32, expected: char) -> bool {
    if row < 0 || column < 0 || row >= lines.len() as i32 || column >= lines[0].len() as i32 {
        return false;
    }

    lines[row as usize].chars().nth(column as usize).unwrap() == expected
}

fn xmas_check(
    lines: &[&str],
    row: i32,
    column: i32,
    row_direction: i32,
    column_direction: i32,
) -> bool {
    check_char(lines, row, column, 'X')
        && check_char(lines, row + row_direction, column + column_direction, 'M')
        && check_char(
            lines,
            row + 2 * row_direction,
            column + 2 * column_direction,
            'A',
        )
        && check_char(
            lines,
            row + 3 * row_direction,
            column + 3 * column_direction,
            'S',
        )
}

fn solve_part_1(lines: &[&str]) -> i32 {
    let mut result = 0;

    for row in 0..lines[0].len() {
        for column in 0..lines.len() {
            for (row_direction, column_direction) in [
                (1, 0),
                (-1, 0),
                (0, 1),
                (0, -1),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ] {
                if xmas_check(
                    lines,
                    row as i32,
                    column as i32,
                    row_direction,
                    column_direction,
                ) {
                    result += 1;
                }
            }
        }
    }

    result
}

fn mas_check(
    lines: &[&str],
    row: i32,
    column: i32,
    row_direction: i32,
    column_direction: i32,
) -> bool {
    check_char(lines, row + row_direction, column + column_direction, 'A')
        && ((check_char(lines, row, column, 'M')
            && check_char(
                lines,
                row + 2 * row_direction,
                column + 2 * column_direction,
                'S',
            ))
            || (check_char(lines, row, column, 'S')
                && check_char(
                    lines,
                    row + 2 * row_direction,
                    column + 2 * column_direction,
                    'M',
                )))
}

fn solve_part_2(lines: &[&str]) -> i32 {
    let mut result = 0;

    for row in 0..lines[0].len() {
        for column in 0..lines.len() {
            if mas_check(lines, row as i32, column as i32, 1, 1)
                && mas_check(lines, row as i32, column as i32 + 2, 1, -1)
            {
                result += 1;
            }
        }
    }

    result
}

fn main() {
    let input = parse_input(include_str!("day_4_input.txt"));
    println!("loaded {} lines", input.len());

    println!("part 1: {}", solve_part_1(&input));
    println!("part 1: {}", solve_part_2(&input));
}

#[cfg(test)]
mod tests {
    use crate::{mas_check, parse_input, solve_part_1, xmas_check};
    use rstest::rstest;

    #[test]
    fn test_parse_input() {
        let input = "123\n456";
        let expected = vec!["123", "456"];

        assert_eq!(expected, parse_input(input));
    }

    #[rstest]
    #[case(vec!["XMAS"], 0, 0, 0, 1, true)]
    fn test_xmas_check(
        #[case] input: Vec<&str>,
        #[case] row: i32,
        #[case] column: i32,
        #[case] row_direction: i32,
        #[case] column_direction: i32,
        #[case] expected: bool,
    ) {
        assert_eq!(
            xmas_check(&input, row, column, row_direction, column_direction),
            expected
        );
    }

    #[test]
    fn test_solve_part_1() {
        let input = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        assert_eq!(18, solve_part_1(&input));
    }

    #[rstest]
    #[case(vec!["MAS"], 0, 0, 0, 1, true)]
    #[case(vec!["SAM"], 0, 0, 0, 1, true)]
    #[case(vec!["MXX", "XAX", "XXS"], 0, 0, 1, 1, true)]
    #[case(vec!["XXS", "XAX", "MXX"], 0, 2, 1, -1, true)]
    fn test_mas_check(
        #[case] input: Vec<&str>,
        #[case] row: i32,
        #[case] column: i32,
        #[case] row_direction: i32,
        #[case] column_direction: i32,
        #[case] expected: bool,
    ) {
        assert_eq!(
            mas_check(&input, row, column, row_direction, column_direction),
            expected
        );
    }

    #[test]
    fn test_solve_part_2() {
        let input = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ];

        assert_eq!(9, crate::solve_part_2(&input));
    }
}
