use crate::load_input;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2025, 2)?;
    let part1 = solve_part1(&input);
    println!("Day 2 part 1: {}", part1);
    let part2 = solve_part2(&input);
    println!("Day 2 part 2: {}", part2);
    Ok(())
}

fn solve_part1(input: &str) -> u64 {
    let mut output: u64 = 0;
    let puzz: Vec<(u64, u64)> = input
        .split(',')
        .map(|pair| {
            pair.split_once('-')
                .map(|(a, b)| {
                    (
                        a.trim().parse::<u64>().expect("Couldn't unwrap first"),
                        b.trim()
                            .parse::<u64>()
                            .unwrap_or_else(|_| panic!("Couldn't unwrap second {}", b)),
                    )
                })
                .expect("Couldn't unwrap tuple")
        })
        .collect();

    for range in puzz {
        for i in range.0..=range.1 {
            let num_digits = i.ilog10() + 1;
            let mut lhs = i;
            let mut rhs = 0;
            let mut exp = 0;
            while exp < num_digits / 2 {
                rhs += lhs % 10 * 10_u64.pow(exp);
                lhs /= 10;
                exp += 1;
                if lhs == rhs {
                    output += i;
                    break;
                }
            }
        }
    }
    output
}

fn solve_part2(input: &str) -> u64 {
    let mut output: u64 = 0;
    let puzz: Vec<(u64, u64)> = input
        .split(',')
        .map(|pair| {
            pair.split_once('-')
                .map(|(a, b)| {
                    (
                        a.trim().parse::<u64>().expect("Couldn't unwrap first"),
                        b.trim()
                            .parse::<u64>()
                            .unwrap_or_else(|_| panic!("Couldn't unwrap second {}", b)),
                    )
                })
                .expect("Couldn't unwrap tuple")
        })
        .collect();

    for range in puzz {
        for i in range.0..=range.1 {
            let strnum = i.to_string();
            for n in 1..=strnum.len() / 2 + 1 {
                if strnum.len() % n != 0 {
                    continue;
                }
                let split = strnum
                    .chars()
                    .collect::<Vec<char>>()
                    .chunks_exact(n)
                    .map(|c| c.iter().collect::<String>())
                    .collect::<Vec<String>>();
                if split.len() == 1 {
                    break;
                }

                if split.windows(2).all(|w| w[0] == w[1]) {
                    output += i;
                    break;
                }
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part_one() {
        let ans = 1227775554;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 4174379265;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}

