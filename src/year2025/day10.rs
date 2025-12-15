use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

use crate::load_input;
use anyhow::Result;

use good_lp::{Expression, Solution, SolverModel, constraint, default_solver, variable, variables};

pub fn solve() -> Result<()> {
    let input = load_input(2025, 10)?;
    let start = std::time::Instant::now();
    let part1 = solve_part1(&input);
    let p1 = std::time::Instant::now();
    println!(
        "Day 10 part 1: {} in {} μs",
        part1,
        (p1 - start).as_micros()
    );
    let start2 = std::time::Instant::now();
    let part2 = solve_part2(&input);
    let p2 = std::time::Instant::now();
    println!(
        "Day 10 part 2: {} in {} μs",
        part2,
        (p2 - start2).as_micros()
    );
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Lights {
    val: u16,
    num_bits: u16,
}

impl FromStr for Lights {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut val: u16 = 0;
        let mut power: u16 = 0;
        for c in s.chars() {
            match c {
                '.' => {
                    power += 1;
                }
                '#' => {
                    val += 2_u16.pow(power as u32);
                    power += 1;
                }
                _ => {}
            };
        }

        Ok(Lights {
            val,
            num_bits: power,
        })
    }
}

impl Display for Lights {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::with_capacity((self.num_bits + 3) as usize);
        s.push('[');
        for i in 0..self.num_bits {
            match (self.val >> i) & 1 {
                0 => {
                    s.push('.');
                }
                1 => {
                    s.push('#');
                }
                _ => unreachable!(),
            }
        }
        s.push(']');
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Button {
    val: u16,
    strrep: String,
}

impl FromStr for Button {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut val: u16 = 0;
        for c in s.chars() {
            if let '0'..='9' = c {
                val += 2_u16.pow(c.to_digit(10).unwrap());
            }
        }
        Ok(Button {
            val,
            strrep: s.to_string(),
        })
    }
}

#[allow(dead_code)]
fn print_history(light: &Lights, button_path: &[&Button]) {
    let mut l = *light;
    println!("{}", l);

    for button in button_path {
        l.val ^= button.val;
        println!("{} after pushing {}", l, button.strrep);
    }
}

fn solve_part1(input: &str) -> usize {
    let puzz: Vec<(Lights, Vec<Button>)> = input
        .lines()
        .map(|l| {
            let mut lineiter = l.split_ascii_whitespace();
            let lights: Lights = lineiter.next().unwrap().parse().unwrap();
            let buttons: Vec<Button> = lineiter
                .filter(|e| e.starts_with('('))
                .filter_map(|e| e.parse::<Button>().ok())
                .collect();
            (lights, buttons)
        })
        .collect();
    let mut output: usize = 0;
    for (target_light, buttons) in puzz {
        let mut seen_lights = HashSet::new();
        let mut queue: VecDeque<(u16, Vec<&Button>)> = VecDeque::new();
        queue.push_back((0, Vec::new()));
        seen_lights.insert(0_u16);
        while let Some((curr_val, button_path)) = queue.pop_front() {
            if curr_val == target_light.val {
                output += button_path.len();
                // Print here to see some viz
                // println!("Solved in {} buttons", button_path.len(),);
                // print_history(
                //     &Lights {
                //         val: 0,
                //         num_bits: target_light.num_bits,
                //     },
                //     &button_path,
                // );
                break;
            }
            // Push all the buttons!
            // I know I could just pass the number of buttons pushed to make
            // it faster, but passing the whole path lets us do viz at the end.
            for button in &buttons {
                let new_val = curr_val ^ button.val;
                // If we haven't seen the new val, add to the BFS.
                if seen_lights.insert(new_val) {
                    let mut new_path = button_path.clone();
                    new_path.push(button);
                    queue.push_back((new_val, new_path));
                }
            }
        }
    }
    output
}

fn solve_part2(input: &str) -> i32 {
    let puzz: Vec<(Vec<Vec<u32>>, Vec<u32>)> = input
        .lines()
        .map(|l| {
            let (rest, jolt) = l.rsplit_once(' ').unwrap();
            let (_, but) = rest.split_once(' ').unwrap();
            let buttons: Vec<Vec<u32>> = but
                .split_ascii_whitespace()
                .map(|b| b.chars().filter_map(|c| c.to_digit(10)).collect())
                .collect();
            let joltages: Vec<u32> = jolt
                .trim_matches(&['{', '}'] as &[_])
                .split(',')
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            (buttons, joltages)
        })
        .collect();
    let mut output = 0;
    for (buttons, joltages) in &puzz {
        let max = joltages.iter().max().unwrap();
        let num_vars = buttons.len();
        variables! { problem: 0 <= b[num_vars] (integer) <= *max; };
        let xvars: Vec<variable::Variable> = b.into_iter().collect();
        let objective: Expression = xvars.iter().map(|&v| v * 1).sum();
        let model = problem.minimise(objective).using(default_solver);
        let mut constraints = Vec::with_capacity(joltages.len());

        for (i, j) in joltages.iter().enumerate() {
            let button_idxs: Vec<usize> = buttons
                .iter()
                .enumerate()
                .filter(|&(_idx, v)| v.contains(&(i as u32)))
                .map(|(idx, _v)| idx)
                .collect();

            let button_vars: Vec<variable::Variable> =
                button_idxs.iter().map(|&i| xvars[i]).collect();
            let this_expr: Expression = button_vars.iter().map(|&v| v * 1).sum();

            constraints.push(constraint!(this_expr == *j));
        }

        let result = model.with_all(constraints).solve();

        match result {
            Ok(res) => {
                let final_value: f64 = xvars.iter().map(|&x| res.value(x)).sum();
                // println!("Solved Problem {}", final_value);
                output += final_value as i32;
            }
            Err(_) => println!("Error with problem"),
        }
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_part_one() {
        let ans = 7;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 33;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}
