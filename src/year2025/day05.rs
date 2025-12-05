use std::cmp::max;

use crate::load_input;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2025, 5)?;
    let now = std::time::Instant::now();
    let part1 = solve_part1(&input);
    let p1 = std::time::Instant::now();
    println!("Day 5 part 1: {} in {} us", part1, (p1 - now).as_micros());
    let part2 = solve_part2(&input);
    println!(
        "Day 5 part 2: {} in {} us",
        part2,
        (std::time::Instant::now() - p1).as_micros()
    );
    Ok(())
}

fn solve_part1(input: &str) -> i32 {
    let (rangetxt, valtxt) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<(u64, u64)> = rangetxt
        .lines()
        .map(|r| {
            let parts = r.split_once("-").unwrap();
            let a = parts.0.parse::<u64>().unwrap();
            let b = parts.1.parse::<u64>().unwrap();
            (a, b)
        })
        .collect();
    let mut vals: Vec<u64> = valtxt.lines().map(|v| v.parse::<u64>().unwrap()).collect();
    let mut output = 0;
    vals.sort();
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let mut rangeiter = ranges.into_iter();
    let mut range = rangeiter.next().unwrap();
    for v in vals {
        while v > range.1
            && let Some(next_range) = rangeiter.next()
        {
            range = next_range;
        }
        if v >= range.0 && v <= range.1 {
            output += 1;
        }
    }

    output
}

fn solve_part2(input: &str) -> u64 {
    let (rangetxt, _) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<(u64, u64)> = rangetxt
        .lines()
        .map(|r| {
            let parts = r.split_once("-").unwrap();
            let a = parts.0.parse::<u64>().unwrap();
            let b = parts.1.parse::<u64>().unwrap();
            (a, b)
        })
        .collect();
    let mut output = 0;
    ranges.sort_by(|a, b| a.0.cmp(&b.0));
    let (mut prev_start, mut prev_end) = ranges[0];
    for (start, end) in ranges.iter().skip(1) {
        if start <= &prev_end {
            prev_end = max(prev_end, *end);
            continue;
        } else {
            output += prev_end - prev_start + 1;
            prev_end = *end;
            prev_start = *start;
        }
    }
    // Grab the final range
    output += prev_end - prev_start + 1;
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part_one() {
        let ans = 3;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 14;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}

