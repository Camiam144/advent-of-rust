use crate::load_input;
use anyhow::Result;
use itertools::{Itertools, repeat_n};

pub fn solve() -> Result<()> {
    let input = load_input(2024, 7)?;
    let part1 = solve_part1(&input);
    println!("Day 7 part 1: {}", part1);
    let part2 = solve_part2(&input);
    println!("Day 7 part 2: {}", part2);
    Ok(())
}

fn solve_part1(input: &str) -> usize {
    let mut output: usize = 0;
    for line in input.lines() {
        let (target, vals) = line.split_once(":").unwrap();
        let parsed_target = target.parse::<usize>().unwrap();
        let parsed_vals: Vec<usize> = vals
            .split_whitespace()
            .map(|i| i.parse::<usize>().expect("Couldn't parse number"))
            .collect();
        let all_op_combos: Vec<Vec<char>> = repeat_n(['*', '+'], parsed_vals.len() - 1)
            .multi_cartesian_product()
            .collect();

        for ops in all_op_combos.into_iter() {
            let total = ops.iter().zip(parsed_vals.iter().skip(1)).fold(
                parsed_vals[0],
                |acc, (op, val)| match op {
                    '*' => acc * val,
                    '+' => acc + val,
                    _ => panic!("Invalid operator"),
                },
            );

            if total == parsed_target {
                // println!("target {} nums {:?} ops {:?}", target, parsed_vals, ops);
                output += total;
                break;
            }
        }
    }
    output
}

/// Concat a and b like 12, 345 -> 12345
fn concat(a: usize, b: usize) -> usize {
    let mut exp: u32 = 1;
    let mut bcopy = b;

    while bcopy >= 10 {
        bcopy /= 10;
        exp += 1;
    }
    a * 10_usize.pow(exp) + b
}

fn solve_part2(input: &str) -> u64 {
    let mut output: u64 = 0;
    for line in input.lines() {
        let (target, vals) = line.split_once(":").unwrap();
        let parsed_target = target.parse::<usize>().unwrap();
        let parsed_vals: Vec<usize> = vals
            .split_whitespace()
            .map(|i| i.parse::<usize>().expect("Couldn't parse number"))
            .collect();
        let all_op_combos: Vec<Vec<char>> = repeat_n(['*', '+', 'l'], parsed_vals.len() - 1)
            .multi_cartesian_product()
            .collect();

        for ops in all_op_combos.into_iter() {
            // if parsed_target == 1010417 {
            //     println!("Trying {:?}", ops);
            // }
            let total = ops.iter().zip(parsed_vals.iter().skip(1)).fold(
                parsed_vals[0],
                |acc, (op, val)| match op {
                    '*' => acc * val,
                    '+' => acc + val,
                    'l' => concat(acc, *val),
                    _ => panic!("Invalid operator"),
                },
            );
            // if parsed_target == 1010417 {
            //     println!("Got {}", total);
            // }

            if total == parsed_target {
                // println!("target {} nums {:?} ops {:?}", target, parsed_vals, ops);
                output += total as u64;
                break;
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
1002: 50 50 2
1010417: 9 1 10 405 1 11";

    #[test]
    fn test_part_one() {
        let ans = 3749;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 12389 + 1010417;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
    #[test]
    fn test_concat() {
        let t1 = concat(12, 345);
        let t2 = concat(10, 0);
        let t3 = concat(10, 10);
        assert_eq!(12345, t1);
        assert_eq!(100, t2);
        assert_eq!(1010, t3);
    }
}

