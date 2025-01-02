fn create_disk_image(input: &str) -> Vec<i16> {
    input
        .chars()
        .filter(|c| *c != '\n')
        .into_iter()
        .enumerate()
        .flat_map(|(i, c)| {
            let n = c.to_string().parse().unwrap();

            if i % 2 == 1 {
                vec![-1; n]
            } else {
                vec![i as i16 / 2; n]
            }
        })
        .collect()
}

fn compact(diskmap: &mut Vec<i16>) {
    let mut to: usize = 0;
    let mut from = diskmap.len() - 1;

    loop {
        if to >= from {
            break;
        } else if diskmap[to] >= 0 {
            to += 1;
        } else if diskmap[from] == -1 {
            from -= 1;
        } else {
            diskmap[to] = diskmap[from];
            diskmap[from] = -1;
            to += 1;
        }
    }
}

fn calculate_checksum(diskmap: &[i16]) -> i128 {
    diskmap
        .iter()
        .enumerate()
        .map(|(i, n)| if *n == -1 { 0 } else { i as i128 * *n as i128 })
        .sum()
}

fn solve_part_1(input: &str) -> i128 {
    let mut diskmap = create_disk_image(input);

    compact(&mut diskmap);

    calculate_checksum(&diskmap)
}

fn find_free(diskmap: &[i16], size: usize, stop_i: usize) -> usize {
    let mut block_start = usize::MAX;

    for i in 0..stop_i {
        if block_start == usize::MAX {
            if diskmap[i] == -1 {
                block_start = i;
            }
        } else if diskmap[i] != -1 {
            block_start = usize::MAX;
        }

        if block_start != usize::MAX && (i - block_start) + 1 == size {
            return block_start;
        }
    }

    usize::MAX
}

fn write_block(diskmap: &mut Vec<i16>, start: usize, size: usize, block_id: i16) {
    for i in start..(start + size) {
        diskmap[i] = block_id;
    }
}

fn defragment(diskmap: &mut Vec<i16>) {
    let mut block_id = *diskmap.iter().max().unwrap();
    let mut block_end = usize::MAX;

    for from in (0..diskmap.len()).rev() {
        if diskmap[from] == block_id {
            // inside the current block
            if block_end == usize::MAX {
                block_end = from;
            }
        } else if block_end != usize::MAX {
            let block_size = block_end - from;

            let to = find_free(diskmap, block_size, from + 1);
            if to != usize::MAX {
                write_block(diskmap, to, block_size, block_id);
                write_block(diskmap, from + 1, block_size, -1);
            }

            block_id -= 1;
            block_end = if diskmap[from] == block_id {
                from
            } else {
                usize::MAX
            };
        }
    }
}

fn solve_part_2(input: &str) -> i128 {
    let mut diskmap = create_disk_image(input);

    defragment(&mut diskmap);

    calculate_checksum(&diskmap)
}

fn main() {
    let input = include_str!("day_9_input.txt");

    println!("part 1: {}", solve_part_1(input));
    println!("part 2: {}", solve_part_2(input));
}

#[cfg(test)]
mod tests {
    use crate::{compact, create_disk_image, defragment, find_free, solve_part_1, solve_part_2};
    use rstest::rstest;

    const EXAMPLE_INPUT: &str = "2333133121414131402";
    #[test]
    fn test_create_disk_image() {
        let expected: Vec<i16> = vec![0, -1, -1, 1, 1, 1, -1, -1, -1, -1, 2, 2, 2, 2, 2];

        assert_eq!(expected, create_disk_image("12345"));
    }

    #[test]
    fn test_compact() {
        let mut diskmap: Vec<i16> = vec![0, -1, 1, -1, 2, 2];
        let expected: Vec<i16> = vec![0, 2, 1, 2, -1, -1];

        compact(&mut diskmap);

        assert_eq!(expected, diskmap);
    }

    #[test]
    fn test_solve_part_1() {
        assert_eq!(1928, solve_part_1(EXAMPLE_INPUT))
    }

    #[test]
    fn test_defragment() {
        let mut diskmap: Vec<i16> = vec![0, -1, 1, 1, -1, -1, 2, 3, 3];
        let expected: Vec<i16> = vec![0, 2, 1, 1, 3, 3, -1, -1, -1];

        defragment(&mut diskmap);

        assert_eq!(expected, diskmap);
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(2858, solve_part_2(EXAMPLE_INPUT))
    }

    #[rstest]
    #[case(2, 6, 3)]
    #[case(2, 4, usize::MAX)]
    fn test_find_free(#[case] size: usize, #[case] stop_i: usize, #[case] expected: usize) {
        let diskmap: Vec<i16> = vec![0, -1, 1, -1, -1];

        let to = find_free(&diskmap, size, stop_i);

        assert_eq!(expected, to);
    }
}
