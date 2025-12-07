use std::collections::{HashSet, VecDeque};

use crate::{load_input, parse_grid_char};
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2025, 7)?;
    let start = std::time::Instant::now();
    let part1 = solve_part1(&input);
    let p1 = std::time::Instant::now();
    println!("Day 7 part 1: {} in {} μs", part1, (p1 - start).as_micros());
    let start2 = std::time::Instant::now();
    let part2 = solve_part2(&input);
    let p2 = std::time::Instant::now();
    println!(
        "Day 7 part 2: {} in {} μs",
        part2,
        (p2 - start2).as_micros()
    );
    Ok(())
}

fn solve_part1(input: &str) -> i32 {
    let mut puzz = parse_grid_char(input);
    puzz = puzz
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v)
        .collect();

    let cc = puzz[0].iter().position(|c| *c == 'S').unwrap();
    let mut splitters: HashSet<(usize, usize)> = HashSet::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = VecDeque::from([(0, cc)]);

    while let Some((cr, cc)) = queue.pop_front() {
        let nr = cr + 1;
        if visited.contains(&(cr, cc)) || nr >= puzz.len() || cc >= puzz[0].len() || cc == 0 {
            continue;
        }
        visited.insert((cr, cc));
        match puzz[nr][cc] {
            '.' => {
                if !visited.contains(&(nr, cc)) {
                    queue.push_back((nr, cc))
                }
            }
            '^' => {
                splitters.insert((nr, cc));
                queue.push_back((nr, cc - 1));
                queue.push_back((nr, cc + 1));
            }
            _ => unreachable!(),
        }
    }
    splitters.len() as i32
}

fn solve_part2(input: &str) -> u64 {
    let mut puzz = parse_grid_char(input);
    puzz = puzz
        .into_iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, v)| v)
        .collect();

    let cc = puzz[0].iter().position(|c| *c == 'S').unwrap();
    let mut beamcount = vec![0; puzz[0].len()];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = VecDeque::from([(0, cc)]);
    beamcount[cc] += 1;

    while let Some((cr, cc)) = queue.pop_front() {
        let nr = cr + 1;
        if visited.contains(&(cr, cc)) || nr >= puzz.len() || cc >= puzz[0].len() || cc == 0 {
            continue;
        }
        visited.insert((cr, cc));
        match puzz[nr][cc] {
            '.' => {
                if !visited.contains(&(nr, cc)) {
                    queue.push_back((nr, cc))
                }
            }
            '^' => {
                beamcount[cc - 1] += beamcount[cc];
                beamcount[cc + 1] += beamcount[cc];
                beamcount[cc] = 0;
                queue.push_back((nr, cc - 1));
                queue.push_back((nr, cc + 1));
            }
            _ => unreachable!(),
        }
    }
    beamcount.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part_one() {
        let ans = 21;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 40;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}

