use std::{fs, path::PathBuf};

use anyhow::{Context, Result};

pub fn create_day_file(year: u16, day: u8) -> Result<()> {
    let solution_path = PathBuf::from(format!("src/year{}/day{:02}.rs", year, day));

    if solution_path.exists() {
        anyhow::bail!(format!(
            "Solution already exists for year {} day {:02}",
            year, day
        ));
    }

    let template = generate_template(year, day);

    fs::write(&solution_path, template).context("Failed to write solution file. If this failed because it's a new year, manually create the folders first. New years must also be manually registered in the lib.")?;

    let input_path = PathBuf::from(format!("input/year{}/day{:02}.txt", year, day));
    fs::write(&input_path, "").context("Failed to create empty input file. If this failed because it's a new year, manually create the folders first. New years must also be manually registered in the lib")?;

    println!(
        "Created new solution file for year {} and day {:02}",
        year, day
    );
    println!("Next you must register the day in the year{}.rs file", year);

    Ok(())
}

fn generate_template(year: u16, day: u8) -> String {
    let template = r#"use crate::load_input;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(YEAR, DAY)?;
    let start = std::time::Instant::now();
    let part1 = solve_part1(&input);
    let p1 = std::time::Instant::now();
    println!("Day DAY part 1: {} in {} μs", part1, (p1 - start).as_micros());
    let start2 = std::time::Instant::now();
    let part2 = solve_part2(&input);
    let p2 = std::time::Instant::now();
    println!("Day DAY part 2: {} in {} μs", part2, (p2 - start2).as_micros());
    Ok(())
}

fn solve_part1(input: &str) -> i32 {
    // TODO: implementation
    0
}

fn solve_part2(input: &str) -> i32 {
    // TODO: implementation
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "test_input";

    #[test]
    fn test_part_one() {
        let ans = 1;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 1;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}"#;
    template
        .replace("YEAR", &year.to_string())
        .replace("DAY", &day.to_string())
}
