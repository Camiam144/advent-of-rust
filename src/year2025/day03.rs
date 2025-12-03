use crate::{load_input, parse_grid_char};
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2025, 3)?;
    let part1 = solve_part1(&input);
    println!("Day 3 part 1: {}", part1);
    let part2 = solve_part2(&input);
    println!("Day 3 part 2: {}", part2);
    Ok(())
}

fn sum_batts(bank: &[u8], num_batts: usize) -> u64 {
    // let mut sum = 0;
    let mut monostack: Vec<u8> = Vec::new();
    monostack.push(bank[0]);

    for (idx, batt) in bank.iter().enumerate().skip(1) {
        // What do we need? A decreasing monostack where the sum of the remaining
        // bank elements and the sum of the empty stack elements is greater than
        // or equal to the batteries required and the stack length is less than or
        // equal to the number of batteries required?

        let remaining = bank.len() - idx;
        while !monostack.is_empty()
            && monostack.last().unwrap() < batt
            && monostack.len() + remaining > num_batts
        {
            monostack.pop();
        }

        if monostack.len() < num_batts {
            monostack.push(*batt);
        }
    }
    monostack
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (n, &val)| acc + val as u64 * 10_u64.pow(n as u32))
}

fn solve_part1(input: &str) -> u64 {
    let puzz = parse_grid_char(input);
    let mut output = 0;

    let intpuzz = puzz
        .iter()
        .map(|v| {
            v.iter()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    for bank in intpuzz {
        let val = sum_batts(&bank, 2);
        output += val;
    }
    output
}

fn solve_part2(input: &str) -> u64 {
    let puzz = parse_grid_char(input);
    let mut output: u64 = 0;

    let intpuzz = puzz
        .iter()
        .map(|v| {
            v.iter()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    for bank in intpuzz {
        let val = sum_batts(&bank, 12);
        output += val;
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_part_one() {
        let ans = 357;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 3121910778619;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}
