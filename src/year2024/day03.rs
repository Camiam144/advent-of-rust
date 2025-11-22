use crate::load_input;
use anyhow::Result;
use regex::Regex;

pub fn solve() -> Result<()> {
    let input = load_input(2024, 3)?;
    let part1 = solve_part1(&input);
    println!("Day 3 part 1: {}", part1);
    let part2 = solve_part2(&input);
    println!("Day 3 part 2: {}", part2);
    Ok(())
}

fn solve_part1(input: &str) -> i32 {
    let mut ans = 0;
    let puzz = input.replace("\n", "");
    let p1reg = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    for (_, [a, b]) in p1reg.captures_iter(&puzz).map(|m| m.extract()) {
        ans += a.parse::<i32>().expect("Couldn't parse first val: ")
            * b.parse::<i32>().expect("Couldn't parse second val: ");
    }
    ans
}

fn solve_part2(input: &str) -> i32 {
    let mut ans = 0;
    let puzz = input.replace("\n", "");
    let to_replace = Regex::new(r#"don't\(\).+?(?:do\(\)|$)"#).unwrap();
    let p1reg = Regex::new(r#"mul\((\d+),(\d+)\)"#).unwrap();
    for (_, [a, b]) in p1reg
        .captures_iter(&to_replace.replace_all(&puzz, ""))
        .map(|m| m.extract())
    {
        ans += a.parse::<i32>().expect("Couldn't parse first val: ")
            * b.parse::<i32>().expect("Couldn't parse second val: ");
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let example1: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let ans = 161;
        assert_eq!(ans, solve_part1(example1));
    }
    #[test]
    fn test_part_two() {
        let example2 = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let ans = 48;
        assert_eq!(ans, solve_part2(example2));
    }
}
