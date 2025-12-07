use crate::{load_input, parse_grid_char};
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2025, 6)?;
    let start = std::time::Instant::now();
    let part1 = solve_part1(&input);
    let p1 = std::time::Instant::now();
    println!("Day 6 part 1: {} in {} μs", part1, (p1 - start).as_micros());
    let start2 = std::time::Instant::now();
    let part2 = solve_part2(&input);
    let p2 = std::time::Instant::now();
    println!(
        "Day 6 part 2: {} in {} μs",
        part2,
        (p2 - start2).as_micros()
    );
    Ok(())
}

fn solve_part1(input: &str) -> u64 {
    let mut puzz: Vec<Vec<&str>> = input
        .trim()
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split_whitespace().collect())
        .collect();
    let ops = puzz.pop().unwrap();
    let mut sols: Vec<u64> = vec![0; ops.len()];
    for row in puzz {
        for (i, c) in row.iter().enumerate() {
            let uc = c.to_string().parse::<u64>().unwrap();
            if sols[i] == 0 {
                sols[i] += uc;
            } else {
                match ops[i] {
                    "+" => {
                        sols[i] += uc;
                    }
                    "*" => {
                        sols[i] *= uc;
                    }
                    _ => panic!("Invalid op"),
                }
            }
        }
    }
    sols.iter().sum()
}

fn solve_part2(input: &str) -> u64 {
    let puzz: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|s| s.chars().collect())
        .collect();

    let mut output: u64 = 0;
    // let mut sols: Vec<u64> = Vec::new();
    let mut tempvec: Vec<u64> = Vec::new();

    for col in (0..puzz[0].len()).rev() {
        let mut tempstring = String::new();
        for row in &puzz {
            tempstring.push(row[col]);
        }
        tempstring = tempstring.trim().to_string();
        if tempstring.is_empty() {
            continue;
        }
        if !tempstring.ends_with('*') && !tempstring.ends_with('+') {
            tempvec.push(tempstring.trim().parse::<u64>().unwrap());
        } else if let Some(op) = tempstring.pop() {
            tempvec.push(tempstring.trim().parse::<u64>().unwrap());
            match op {
                '*' => {
                    output += tempvec.iter().product::<u64>();
                    tempvec.clear();
                }
                '+' => {
                    output += tempvec.iter().sum::<u64>();
                    tempvec.clear();
                }
                _ => panic!("Should be an op"),
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part_one() {
        let ans = 4277556;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 3263827;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}
