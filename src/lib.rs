use anyhow::{Context, Result};
use std::path::PathBuf;

pub mod template;
pub mod year2024;

pub fn get_input_path(year: u16, day: u8) -> PathBuf {
    PathBuf::from(format!("input/year{}/day{:02}.txt", year, day))
}

pub fn load_input(year: u16, day: u8) -> Result<String> {
    let path = get_input_path(year, day);
    std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read input file {}", path.display()))
}

/// Parses a typical input grid of numbers into a vec of vec of numbers
pub fn parse_grid_i32(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|s| s.parse::<i32>().expect("Couldn't parse number"))
                .collect()
        })
        .collect()
}

/// Parses a typical input grid in a vec of vec of chars
pub fn parse_grid_char(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim())
        .map(|s| s.chars().collect())
        .collect()
}

// Ripped this from another template kinda
pub type SolverFn = fn() -> anyhow::Result<()>;
pub struct Solutions;

impl Solutions {
    pub fn get_solver(year: u16, day: u8) -> Option<SolverFn> {
        match year {
            2024 => Self::get_year_sols(year2024::SOLUTIONS, day),
            _ => None,
        }
    }

    fn get_year_sols(days: &[(&str, SolverFn)], day: u8) -> Option<SolverFn> {
        days.iter()
            .find(|(d, _)| d.parse::<u8>().ok() == Some(day))
            .map(|(_, solver)| *solver)
    }

    pub fn list_years() -> Vec<u16> {
        vec![2024]
    }

    pub fn available_days(year: u16) -> Vec<u8> {
        let days = match year {
            2024 => &year2024::SOLUTIONS,
            _ => return vec![],
        };

        days.iter().filter_map(|(d, _)| d.parse().ok()).collect()
    }
}
