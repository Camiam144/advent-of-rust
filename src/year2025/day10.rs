use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    str::FromStr,
};

use crate::load_input;
use anyhow::Result;

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

struct Joltage {
    stringrep: String,
}

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
        let mut queue: VecDeque<(Lights, Vec<&Button>)> = VecDeque::new();
        queue.push_back((
            Lights {
                val: 0,
                num_bits: target_light.num_bits,
            },
            Vec::new(),
        ));
        while let Some((curr_light, button_path)) = queue.pop_front() {
            if curr_light.val == target_light.val {
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
            if seen_lights.contains(&curr_light.val) {
                continue;
            }
            // Stick our current lights in the hash set
            seen_lights.insert(curr_light.val);
            // Push all the buttons!
            for button in &buttons {
                let mut newlight = curr_light;
                newlight.val ^= button.val;
                let mut new_path = button_path.clone();
                new_path.push(button);
                queue.push_back((newlight, new_path));
            }
        }
    }
    output
}

fn solve_part2(input: &str) -> i32 {
    // TODO: implementation
    0
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
        let ans = 1;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}

