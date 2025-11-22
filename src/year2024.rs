use crate::SolverFn;
mod day01;
mod day02;
mod day03;

pub const SOLUTIONS: &[(&str, SolverFn)] = &[
    ("1", day01::solve),
    // Add days here
    ("2", day02::solve),
    ("3", day03::solve),
];
