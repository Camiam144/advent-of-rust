use crate::load_input;
use anyhow::Result;
use std::collections::HashMap;

pub fn solve() -> Result<()> {
    let input = load_input(2024, 1)?;
    let part1 = solve_part1(&input);
    println!("Day 01 part 1: {}", part1);
    let part2 = solve_part2(&input);
    println!("Day 01 part 2: {}", part2);
    Ok(())
}

fn solve_part1(input: &str) -> i32 {
    // Input is one big string, need to split left and right and make 2 vecs
    let (mut left_side, mut right_side) = lines_to_vecs(input);
    left_side.sort();
    right_side.sort();

    // Cool rust stuff
    let total_diff: i32 = left_side
        .iter()
        .zip(right_side)
        .map(|(a, b)| (a - b).abs())
        .sum();

    total_diff
}

fn solve_part2(input: &str) -> i32 {
    let (mut left_side, mut right_side) = lines_to_vecs(input);
    let mut mymap: HashMap<i32, i32> = HashMap::new();

    left_side.sort();
    right_side.sort();

    // Cool rust map implementation
    for elem in right_side {
        mymap.entry(elem).and_modify(|val| *val += 1).or_insert(1);
    }

    // Sum the stuff
    let mut total = 0;
    for num in left_side {
        if let Some(rval) = mymap.get(&num) {
            total += num * rval;
        }
    }
    total
}

fn lines_to_vecs(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();
        if let (Some(left), Some(right)) = (split.next(), split.next()) {
            left_side.push(left.parse::<i32>().unwrap());
            right_side.push(right.parse::<i32>().unwrap());
        }
    }
    (left_side, right_side)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_part_one() {
        let ans = 11;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 31;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}
