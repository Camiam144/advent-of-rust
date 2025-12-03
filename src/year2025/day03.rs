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

fn solve_part1(input: &str) -> i32 {
    let puzz = parse_grid_char(input);
    let mut output = 0;

    for bank in puzz {
        let mut left = 0;
        let mut right = 0;
        let mut idx = 0;
        let intbank = bank
            .iter()
            .map(|e| e.to_string().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        for (i, elem) in intbank.iter().take(intbank.len() - 1).enumerate() {
            if *elem > left {
                left = *elem;
                idx = i;
            }
        }

        for elem in intbank.iter().skip(idx + 1) {
            if *elem > right {
                right = *elem;
            }
        }
        output += left * 10 + right;
    }
    output
}

fn remove_right_smallest<T: std::cmp::PartialOrd>(invec: &mut Vec<T>) {
    let mut midx = 0;

    for i in (1..invec.len()).rev() {
        if invec[i] < invec[i - 1] {
            midx = i;
            break;
        }
    }

    invec.remove(midx);
}

fn solve_part2(input: &str) -> u64 {
    let puzz = parse_grid_char(input);
    let mut output: u64 = 0;
    let seqlen = 12;

    let intpuzz = puzz
        .iter()
        .map(|v| {
            v.iter()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<Vec<u8>>>();

    for bank in intpuzz {
        let mut outdigits: Vec<u8> = bank[(bank.len() - seqlen)..].to_vec();
        outdigits.reverse();
        for elem in bank.iter().take(bank.len() - seqlen).rev() {
            if elem >= outdigits.last().expect("Outdigits is empty!") {
                // Remove leftmost smallest element
                outdigits.push(*elem);
                remove_right_smallest(&mut outdigits);
            }
        }
        // Calculate the total
        let out_num: u64 = outdigits
            .iter()
            .enumerate()
            .fold(0, |acc, (n, &val)| acc + val as u64 * 10_u64.pow(n as u32));

        output += out_num;
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

