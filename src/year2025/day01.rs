use crate::load_input;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2025, 1)?;
    let part1 = solve_part1(&input);
    println!("Day 1 part 1: {}", part1);
    let part2 = solve_part2(&input);
    println!("Day 1 part 2: {}", part2);
    Ok(())
}

fn solve_part1(input: &str) -> i32 {
    // TODO: implementation
    0
}

fn solve_part2(input: &str) -> i32 {
    // TODO: implementation
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "test_input";

    #[test]
    fn test_part_one() {
        let ans = 1;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 1;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}