use crate::SolverFn;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

pub const SOLUTIONS: &[(&str, SolverFn)] = &[
    ("1", day01::solve),
    // Add days here
    ("2", day02::solve),
    ("3", day03::solve),
    ("4", day04::solve),
    ("5", day05::solve),
    ("6", day06::solve),
    ("7", day07::solve),
    ("8", day08::solve),
    ("9", day09::solve),
];
