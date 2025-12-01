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
    let mut output = 0;
    let mut curr_val: i32 = 50;

    for line in input.lines() {
        let dir = line.chars().next().expect("No next char");
        let mut steps = line[1..].parse::<i32>().expect("Couldn't unwrap value");
        steps %= 100;

        if dir == 'R' {
            curr_val = (curr_val + steps) % 100;
        } else {
            curr_val -= steps;
            while curr_val < 0 {
                curr_val += 100;
            }
        }
        if curr_val == 0 {
            output += 1;
        }
    }

    output
}

fn solve_part2(input: &str) -> i32 {
    let mut output = 0;
    let mut curr_val: i32 = 50;

    for line in input.lines() {
        let dir = line.chars().next().expect("No next char");
        let mut steps = line[1..].parse::<i32>().expect("Couldn't unwrap value");

        while steps > 0 {
            if dir == 'R' {
                curr_val += 1;
                if curr_val > 99 {
                    curr_val = 0;
                }
            } else if dir == 'L' {
                curr_val -= 1;
                if curr_val < 0 {
                    curr_val = 99;
                }
            }
            if curr_val == 0 {
                output += 1;
            }
            steps -= 1;
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_part_one() {
        let ans = 3;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 6;
        assert_eq!(ans, solve_part2(EXAMPLE));
        assert_eq!(10, solve_part2("R1000"));
    }
}

