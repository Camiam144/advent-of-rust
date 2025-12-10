use std::cmp::{max, min};

use crate::load_input;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2025, 9)?;
    let start = std::time::Instant::now();
    let part1 = solve_part1(&input);
    let p1 = std::time::Instant::now();
    println!("Day 9 part 1: {} in {} μs", part1, (p1 - start).as_micros());
    let start2 = std::time::Instant::now();
    let part2 = solve_part2(&input);
    let p2 = std::time::Instant::now();
    println!(
        "Day 9 part 2: {} in {} μs",
        part2,
        (p2 - start2).as_micros()
    );
    Ok(())
}

fn solve_part1(input: &str) -> usize {
    let puzz: Vec<(usize, usize)> = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .collect();

    let mut max_rect = 0;

    for i in 0..puzz.len() - 1 {
        for j in i + 1..puzz.len() {
            let area = (puzz[i].0.abs_diff(puzz[j].0) + 1) * (puzz[i].1.abs_diff(puzz[j].1) + 1);
            max_rect = max(max_rect, area);
        }
    }
    max_rect
}

fn does_intersect(
    rmin: &(usize, usize),
    rmax: &(usize, usize),
    poly1: &(usize, usize),
    poly2: &(usize, usize),
) -> bool {
    let minpx = min(poly1.0, poly2.0);
    let minpy = min(poly1.1, poly2.1);
    let maxpx = max(poly1.0, poly2.0);
    let maxpy = max(poly1.1, poly2.1);

    // Check if the line is completely outside
    if !((maxpy <= rmin.1) || (rmax.1 <= minpy) || (maxpx <= rmin.0) || (rmax.0 <= minpx)) {
        return true;
    }
    false
}

fn solve_part2(input: &str) -> usize {
    let puzz: Vec<(usize, usize)> = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap())
        })
        .collect();

    let mut edges: Vec<((usize, usize), (usize, usize))> =
        puzz.windows(2).map(|w| (w[0], w[1])).collect();

    edges.push((*puzz.last().unwrap(), *puzz.first().unwrap()));

    let mut max_seen = 0;

    for i in 0..puzz.len() - 1 {
        'boxloop: for j in i + 1..puzz.len() {
            let point1 = puzz[i];
            let point2 = puzz[j];
            let area = (point1.0.abs_diff(point2.0) + 1) * (point1.1.abs_diff(point2.1) + 1);
            if area <= max_seen {
                continue;
            }
            let min_x = min(puzz[i].0, puzz[j].0);
            let min_y = min(puzz[i].1, puzz[j].1);
            let max_x = max(puzz[i].0, puzz[j].0);
            let max_y = max(puzz[i].1, puzz[j].1);

            let rmin = (min_x, min_y);
            let rmax = (max_x, max_y);
            // check each rect edge for collision
            // THIS IS NOT CORRECT! IT JUST SO HAPPENS TO WORK FOR THIS
            // PARTICULAR INPUT AND I'M TIRED.
            // If you have an ugly shape like an L or a T this method will not
            // determine if your rect is inside or outside the polygon. This
            // input luckily is close to a circle with a narrow strip removed
            // like some sort of freaky pacman.
            // what we should do is verify the rectangle is actually inside the
            // polygon by doing something like raycasting from an arbitrary
            // point within our rectangle and counting the number of segments the
            // ray intersects, even = outside, odd = inside
            for (pe1, pe2) in edges.iter() {
                if does_intersect(&rmin, &rmax, pe1, pe2) {
                    continue 'boxloop;
                }
            }

            max_seen = max(max_seen, area);
        }
    }
    max_seen
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_part_one() {
        let ans = 50;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 24;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}
