use crate::load_input;
use crate::parse_grid_char;
use anyhow::Result;

use std::collections::HashSet;

pub fn solve() -> Result<()> {
    let input = load_input(2024, 6)?;
    let part1 = solve_part1(&input);
    println!("Day 6 part 1: {}", part1);
    let part2 = solve_part2(&input);
    println!("Day 6 part 2: {}", part2);
    Ok(())
}

const DIRS: [(char, (i32, i32)); 4] =
    [('u', (-1, 0)), ('r', (0, 1)), ('d', (1, 0)), ('l', (0, -1))];

fn solve_part1(input: &str) -> i32 {
    let puzzgrid = parse_grid_char(input);
    let guard_idx = input
        .replace("\n", "")
        .chars()
        .position(|c| c == '^')
        .expect("No guard");
    let (curr_row, curr_col) = (guard_idx / puzzgrid.len(), guard_idx % puzzgrid[0].len());

    let out: (String, Option<usize>) = walk_grid(&puzzgrid, curr_row, curr_col, 0, None);

    if let Some(output) = out.1 {
        output as i32
    } else {
        0
    }
}

fn walk_grid(
    grid: &[Vec<char>],
    row: usize,
    col: usize,
    initial_dir_int: usize,
    seen_states: Option<&HashSet<(usize, usize, char)>>,
) -> (String, Option<usize>) {
    let mut visited_dir = match seen_states {
        Some(set) => set.clone(),
        None => HashSet::new(),
    };

    let mut curr_row = row;
    let mut curr_col = col;
    let mut dir_int = initial_dir_int;

    loop {
        let (dir, deltas) = DIRS[dir_int % 4];
        let next_row = curr_row as i32 + deltas.0;
        let next_col = curr_col as i32 + deltas.1;

        if next_row >= grid.len() as i32
            || next_row < 0
            || next_col >= grid[0].len() as i32
            || next_col < 0
        {
            visited_dir.insert((curr_row, curr_col, dir));
            break;
        }

        if visited_dir.contains(&(curr_row, curr_col, dir)) {
            return ("loop".to_string(), None);
        }

        visited_dir.insert((curr_row, curr_col, dir));

        match grid[next_row as usize][next_col as usize] {
            '#' => {
                dir_int += 1;
            }
            _ => {
                curr_row = next_row as usize;
                curr_col = next_col as usize;
            }
        }
    }
    let mut p1hash: HashSet<(usize, usize)> = HashSet::new();
    for e in visited_dir.iter() {
        p1hash.insert((e.0, e.1));
    }

    ("exited".to_string(), Some(p1hash.len()))
}

fn solve_part2(input: &str) -> i32 {
    let mut puzzgrid = parse_grid_char(input);
    let guard_idx = input
        .replace("\n", "")
        .chars()
        .position(|c| c == '^')
        .expect("No guard");
    let (mut curr_row, mut curr_col) = (guard_idx / puzzgrid.len(), guard_idx % puzzgrid[0].len());

    // Every time we take a step we put an obstacle in front of us and check
    // Can't put an obstacle somewhere we've already stepped
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_dir: HashSet<(usize, usize, char)> = HashSet::new();
    let mut good_locs: HashSet<(i32, i32)> = HashSet::new();
    let mut dir_int = 0;

    loop {
        let (dir, deltas) = DIRS[dir_int % 4];

        let next_row = curr_row as i32 + deltas.0;
        let next_col = curr_col as i32 + deltas.1;

        if next_row >= puzzgrid.len() as i32
            || next_row < 0
            || next_col >= puzzgrid[0].len() as i32
            || next_col < 0
        {
            break;
        }

        visited.insert((curr_row, curr_col));

        match puzzgrid[next_row as usize][next_col as usize] {
            '#' => {
                visited_dir.insert((curr_row, curr_col, dir));
                dir_int += 1;
            }
            _ => {
                if !visited.contains(&(next_row as usize, next_col as usize)) {
                    puzzgrid[next_row as usize][next_col as usize] = '#';
                    let (result, _) =
                        walk_grid(&puzzgrid, curr_row, curr_col, dir_int, Some(&visited_dir));
                    puzzgrid[next_row as usize][next_col as usize] = '.';
                    if result.as_str() == "loop" {
                        good_locs.insert((next_row, next_col));
                    }
                }
                visited_dir.insert((curr_row, curr_col, dir));
                curr_row = next_row as usize;
                curr_col = next_col as usize;
            }
        }
    }
    good_locs.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_part_one() {
        let ans = 41;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 6;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}
