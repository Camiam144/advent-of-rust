use crate::{load_input, parse_grid_char};
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2025, 4)?;
    let start = std::time::Instant::now();
    let part1 = solve_part1(&input);
    let p1 = std::time::Instant::now();
    println!("Day 4 part 1: {} in {} μs", part1, (p1 - start).as_micros());
    let part2 = solve_part2(&input);
    println!(
        "Day 4 part 2: {} in {} μs",
        part2,
        (std::time::Instant::now() - p1).as_micros()
    );
    Ok(())
}

const DIRS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn can_access(puzz: &[Vec<char>], row: usize, col: usize) -> bool {
    let mut num_paper = 0;
    for (dr, dc) in &DIRS {
        let nr = row as i32 + dr;
        let nc = col as i32 + dc;
        if nc >= 0
            && nr >= 0
            && (nc as usize) < puzz[0].len()
            && (nr as usize) < puzz.len()
            && puzz[nr as usize][nc as usize] == '@'
        {
            num_paper += 1;
        }
    }
    num_paper < 4
}

fn solve_part1(input: &str) -> i32 {
    let puzz = parse_grid_char(input);

    let mut output = 0;

    for row in 0..puzz.len() {
        for col in 0..puzz[0].len() {
            let c = puzz[row][col];
            if c != '@' {
                continue;
            }

            if can_access(&puzz, row, col) {
                output += 1;
            }
        }
    }
    output
}

fn solve_part2(input: &str) -> i32 {
    // This will start with a brute force solution, eventually I will refactor
    // to something that only checks the next possible options
    let mut puzz = parse_grid_char(input);
    let mut output = 0;

    // The logic here kinda prevents using a queue of remaining rolls
    // I would need to move the "can access" logic out of the function, or
    // return a list of all neighbors to enqueue
    let mut has_removed = true;
    while has_removed {
        has_removed = false;
        for row in 0..puzz.len() {
            for col in 0..puzz[0].len() {
                let c = puzz[row][col];
                if c != '@' {
                    continue;
                }

                if can_access(&puzz, row, col) {
                    has_removed = true;
                    output += 1;
                    puzz[row][col] = '.';
                }
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part_one() {
        let ans = 13;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 43;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}
