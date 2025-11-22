use crate::{load_input, parse_grid_i32};
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2024, 2)?;
    let part1 = solve_part1(&input);
    println!("Day 2 part 1: {}", part1);
    let part2 = solve_part2(&input);
    println!("Day 2 part 2: {}", part2);
    Ok(())
}

fn solve_part1(input: &str) -> i32 {
    let mut num_safe = 0;
    let puzz_lines = parse_grid_i32(input);
    let mut bad_lines: Vec<Vec<i32>> = Vec::new();
    for line in puzz_lines {
        let diffs: Vec<i32> = line.windows(2).map(|w| w[1] - w[0]).collect();
        if diff_vec_is_good(&diffs) {
            num_safe += 1;
        } else {
            bad_lines.push(line);
        }
    }
    num_safe
}

fn diff_vec_is_good(slice: &[i32]) -> bool {
    let mut prev_sign = None;
    for diff in slice.iter() {
        let sign = diff.signum();
        if let Some(prev) = prev_sign
            && (prev != sign || sign == 0)
        {
            return false;
        }
        prev_sign = Some(sign);
        // Check diff is okay
        if !(1..=3).contains(&diff.abs()) {
            return false;
        }
    }
    true
}

fn solve_part2(input: &str) -> i32 {
    let mut num_safe = 0;
    let puzz_lines = parse_grid_i32(input);
    for line in puzz_lines {
        let diff: Vec<i32> = line.windows(2).map(|w| w[1] - w[0]).collect();
        if diff_vec_is_good(&diff) {
            num_safe += 1;
            continue;
        }
        // Brute force attempt to fix vec:
        let majority_pos: bool = diff.iter().filter(|&d| *d > 0).count() > diff.len() / 2;
        for (i, d) in diff.iter().enumerate() {
            // If it's even possible to fix the vector, try
            if !(1..=3).contains(d) || (majority_pos && *d < 0) || (!majority_pos && *d > 0) {
                let newline1: Vec<i32> = line
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != i)
                    .map(|(_, &v)| v)
                    .collect();
                let newline2: Vec<i32> = line
                    .iter()
                    .enumerate()
                    .filter(|(idx, _)| *idx != i + 1)
                    .map(|(_, &v)| v)
                    .collect();

                let diff1: Vec<i32> = newline1.windows(2).map(|w| w[1] - w[0]).collect();
                let diff2: Vec<i32> = newline2.windows(2).map(|w| w[1] - w[0]).collect();

                if diff_vec_is_good(&diff1) || diff_vec_is_good(&diff2) {
                    num_safe += 1;
                    break;
                }
            }
        }
    }
    num_safe
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_part_one() {
        let ans = 2;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 4;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}
