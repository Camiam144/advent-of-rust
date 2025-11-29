use crate::load_input;
use crate::parse_grid_char;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2024, 4)?;
    let grid = parse_grid_char(&input);
    let part1 = solve_part1(&grid);
    println!("Day 4 part 1: {}", part1);
    let part2 = solve_part2(&grid);
    println!("Day 4 part 2: {}", part2);
    Ok(())
}

const MAS: &str = "MAS";

const DIRECTIONS: [(isize, isize); 8] = [
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn is_mas(x: usize, y: usize, dir: (isize, isize), grid: &[Vec<char>]) -> bool {
    // Check if the characters in a given direction are the word "MAS"
    let mut search = 0;
    let (dx, dy) = dir;
    let mut point = (x as isize, y as isize);

    while search < 3 {
        point.0 += dx;
        point.1 += dy;

        if point.0 < 0
            || point.0 >= grid.len() as isize
            || point.1 < 0
            || point.1 >= grid[0].len() as isize
        {
            return false;
        }

        let checkchar = MAS.chars().nth(search).unwrap();

        if let Some(thisrow) = grid.get(point.0 as usize) {
            if let Some(thischar) = thisrow.get(point.1 as usize) {
                if thischar == &checkchar {
                    search += 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn solve_part1(input: &[Vec<char>]) -> i32 {
    let mut num_xmas = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'X' {
                for dir in DIRECTIONS {
                    if is_mas(i, j, dir, input) {
                        num_xmas += 1;
                    }
                }
            }
        }
    }
    num_xmas
}

fn solve_part2(input: &[Vec<char>]) -> i32 {
    let mut num_xmas = 0;
    let [ul, ur, dl, dr] = &DIRECTIONS[4..=7] else {
        panic!("Couldnt' destructure")
    };

    for (i, row) in input.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'A' && !(i == 0 || i == input.len() - 1 || j == 0 || j == input[0].len() - 1) {
                let d1u = (i as isize + ul.0, j as isize + ul.1);
                let d1d = (i as isize + dr.0, j as isize + dr.1);
                let d2u = (i as isize + ur.0, j as isize + ur.1);
                let d2d = (i as isize + dl.0, j as isize + dl.1);

                let d1uc = input[d1u.0 as usize][d1u.1 as usize];
                let d1dc = input[d1d.0 as usize][d1d.1 as usize];
                let d2uc = input[d2u.0 as usize][d2u.1 as usize];
                let d2dc = input[d2d.0 as usize][d2d.1 as usize];

                if (d1uc == 'S' || d1uc == 'M')
                    && (d1dc == 'S' || d1dc == 'M')
                    && d1uc != d1dc
                    && (d2uc == 'S' || d2uc == 'M')
                    && (d2dc == 'S' || d2dc == 'M')
                    && d2uc != d2dc
                {
                    num_xmas += 1;
                }
            }
        }
    }
    num_xmas
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_part_one() {
        let exgrid = parse_grid_char(EXAMPLE);
        let ans = 18;
        assert_eq!(ans, super::solve_part1(&exgrid));
    }
    #[test]
    fn test_part_two() {
        let exgrid = parse_grid_char(EXAMPLE);
        let ans = 9;
        assert_eq!(ans, super::solve_part2(&exgrid));
    }
}

