use crate::load_input;
use anyhow::Result;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

pub fn solve() -> Result<()> {
    let input = load_input(2024, 5)?;
    let part1 = solve_part1(&input);
    println!("Day 5 part 1: {}", part1);
    let part2 = solve_part2(&input);
    println!("Day 5 part 2: {}", part2);
    Ok(())
}

fn solve_part1(input: &str) -> i32 {
    let (rules, all_pages): (&str, &str) = input.trim().split_once("\n\n").unwrap();
    let mut rulemap: HashMap<String, HashSet<String>> = HashMap::new();
    let mut num_good: i32 = 0;

    for rule in rules.lines() {
        let (a, b) = rule.split_once("|").unwrap();
        rulemap
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());
    }

    for pages in all_pages.lines() {
        let pagelist = pages.split(",").collect::<Vec<_>>();
        if pagelist.is_sorted_by(|a, b| {
            if let Some(set) = rulemap.get(*a) {
                set.contains(*b)
            } else {
                false
            }
        }) {
            // Get middle value
            num_good += pagelist[pagelist.len() / 2]
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Couldn't parse val: {:?}", pagelist));
        }
    }

    num_good
}

fn solve_part2(input: &str) -> i32 {
    let (rules, all_pages): (&str, &str) = input.trim().split_once("\n\n").unwrap();
    let mut rulemap: HashMap<String, HashSet<String>> = HashMap::new();
    let mut num_good: i32 = 0;

    for rule in rules.lines() {
        let (a, b) = rule.split_once("|").unwrap();
        rulemap
            .entry(a.to_string())
            .or_default()
            .insert(b.to_string());
    }

    for pages in all_pages.lines() {
        let mut pagelist = pages.split(",").collect::<Vec<_>>();
        if !pagelist.is_sorted_by(|a, b| {
            if let Some(set) = rulemap.get(*a) {
                set.contains(*b)
            } else {
                false
            }
        }) {
            pagelist.sort_by(|a, b| {
                if let Some(set) = rulemap.get(*a) {
                    set.contains(*b).cmp(&true)
                } else {
                    Ordering::Less
                }
            });
            // Get middle value
            num_good += pagelist[pagelist.len() / 2]
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Couldn't parse val: {:?}", pagelist));
        }
    }
    num_good
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_part_one() {
        let ans = 143;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 123;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}
