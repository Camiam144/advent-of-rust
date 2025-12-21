use std::str::FromStr;

use crate::{load_input, parse_grid_char};
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2025, 12)?;
    let start = std::time::Instant::now();
    let part1 = solve_part1(&input);
    let p1 = std::time::Instant::now();
    println!(
        "Day 12 part 1: {} in {} μs",
        part1,
        (p1 - start).as_micros()
    );
    let start2 = std::time::Instant::now();
    let part2 = solve_part2(&input);
    let p2 = std::time::Instant::now();
    println!(
        "Day 12 part 2: {} in {} μs",
        part2,
        (p2 - start2).as_micros()
    );
    Ok(())
}

struct Polyomino {
    width: usize,
    height: usize,
    num_tiles: usize,
    repr: Vec<Vec<char>>,
}
impl FromStr for Polyomino {
    type Err = &'static str;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut num_tiles = 0;
        let poly_repr = parse_grid_char(s);

        let height = poly_repr.len();
        let width = poly_repr[0].len();

        for line in poly_repr.iter() {
            for c in line {
                if *c == '#' {
                    num_tiles += 1;
                }
            }
        }

        std::result::Result::Ok(Polyomino {
            width,
            height,
            num_tiles,
            repr: poly_repr,
        })
    }
}
fn parse_input(input: &str) -> (Vec<Polyomino>, Vec<(usize, usize, Vec<usize>)>) {
    let (raw_polyominos, raw_grids) = input.rsplit_once("\n\n").unwrap();

    // Parse polyominos, then parse the grids.

    let mut all_polyominos: Vec<Polyomino> = Vec::new();

    for poly in raw_polyominos.split("\n\n") {
        let (_idx, polylines) = poly.split_once("\n").expect("Couldn't split poly");
        let this_poly: Polyomino = Polyomino::from_str(polylines).unwrap();
        all_polyominos.push(this_poly);
    }

    // Grids are given as lengthxwidth: (polycounts)

    let mut all_grids: Vec<(usize, usize, Vec<usize>)> = Vec::new();
    for gridspec in raw_grids.lines() {
        let (sizestr, raw_polycounts) = gridspec.split_once(':').unwrap();
        let dims: Vec<&str> = sizestr.split('x').collect();
        let width = dims[0].parse::<usize>().unwrap();
        let height = dims[1].parse::<usize>().unwrap();

        let polycounts = raw_polycounts
            .trim()
            .split(' ')
            .map(|v| v.parse::<usize>().unwrap())
            .collect();
        all_grids.push((width, height, polycounts));
    }

    (all_polyominos, all_grids)
}

/// I got a little (a lot) spoiled on this one. I put it off because I thought
/// I was going to have to learn recursive backtracking with these complex
/// polyomino shapes, and instead I got spoiled on part 1.
fn solve_part1(input: &str) -> i32 {
    let mut result = 0;
    let mut nontrivial = 0;

    let (polyominos, grids) = parse_input(input);

    for (width, height, polyspec) in grids.iter() {
        // Check if grid is trivially too small
        let total_tiles_required = polyspec
            .iter()
            .zip(polyominos.iter())
            .fold(0, |acc, (n, p)| acc + n * p.num_tiles);

        if total_tiles_required > width * height {
            // println!(
            //     "Grid {} x {} with {:?} is trivally too small: {} required, {}, available",
            //     width,
            //     height,
            //     polyspec,
            //     { total_tiles_required },
            //     width * height
            // );
            continue;
        }

        // Check if grid is trivially large enough
        // This is a little bit easier since all polyominos fit in a 3x3 grid.

        let triv_width = width / 3;
        let triv_height = height / 3;
        let num_polys_required = polyspec.iter().sum();
        if triv_height * triv_width >= num_polys_required {
            // println!(
            //     "Grid {} x {} with {:?} is trivally large enough, can fit {} full polys only requires {}",
            //     width,
            //     height,
            //     polyspec,
            //     width * height,
            //     num_polys_required
            // );
            result += 1;
            continue;
        }
        // Nontrivial examples
        nontrivial += 1;
    }
    println!(
        "Out of {} total examples, we have {} non trivial",
        grids.len(),
        nontrivial
    );

    result
}

fn solve_part2(input: &str) -> i32 {
    // TODO: implementation
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

    #[test]
    fn test_part_one() {
        let ans = 2;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 1;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}

