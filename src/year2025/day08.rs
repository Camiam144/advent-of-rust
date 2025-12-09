use crate::load_input;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2025, 8)?;
    let start = std::time::Instant::now();
    let part1 = solve_part1(&input, 1000);
    let p1 = std::time::Instant::now();
    println!("Day 8 part 1: {} in {} μs", part1, (p1 - start).as_micros());
    let start2 = std::time::Instant::now();
    let part2 = solve_part2(&input);
    let p2 = std::time::Instant::now();
    println!(
        "Day 8 part 2: {} in {} μs",
        part2,
        (p2 - start2).as_micros()
    );
    Ok(())
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    c: Option<usize>,
}

fn sq_dist_between(p1: &Point, p2: &Point) -> u64 {
    ((p1.x - p2.x).pow(2) + (p1.y - p2.y).pow(2) + (p1.z - p2.z).pow(2)).unsigned_abs()
}

fn solve_part1(input: &str, num_cons: i32) -> u64 {
    let mut points: Vec<Point> = Vec::new();
    let mut circuits: Vec<Vec<usize>> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let s: Vec<&str> = line.split(",").collect();
        let p = Point {
            x: s[0].parse().unwrap(),
            y: s[1].parse().unwrap(),
            z: s[2].parse().unwrap(),
            c: Some(i),
        };
        points.push(p);
        circuits.push(vec![i]);
    }

    let mut dist_pts: Vec<(u64, usize, usize)> = Vec::new();
    let ptslen = points.len();

    for i in 0..ptslen - 1 {
        for j in i + 1..ptslen {
            dist_pts.push((sq_dist_between(&points[i], &points[j]), i, j));
        }
    }
    dist_pts.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut ctr = 0;

    while ctr < num_cons {
        let Some((_, idx1, idx2)) = dist_pts.pop() else {
            break;
        };
        let p1 = &points[idx1];
        let p2 = &points[idx2];
        let p1c = p1.c.unwrap();
        let p2c = p2.c.unwrap();
        if p1.c == p2.c {
            ctr += 1;
            continue;
        } else {
            let circ2 = circuits[p2c].clone();
            for pidx in &circ2 {
                let p = points.get_mut(*pidx).unwrap();
                p.c = Some(p1c);
            }
            circuits[p1c].extend(circ2);
            circuits[p2c] = Vec::new();

            ctr += 1;
        }
    }
    let mut circ_vec: Vec<u64> = Vec::new();
    for circ in circuits {
        circ_vec.push(circ.len() as u64);
    }
    circ_vec.sort_unstable_by(|a, b| b.cmp(a));

    circ_vec[0] * circ_vec[1] * circ_vec[2]
}

fn solve_part2(input: &str) -> i64 {
    let mut points: Vec<Point> = Vec::new();
    let mut circuits: Vec<Vec<usize>> = Vec::new();

    for (i, line) in input.lines().enumerate() {
        let s: Vec<&str> = line.split(",").collect();
        let p = Point {
            x: s[0].parse().unwrap(),
            y: s[1].parse().unwrap(),
            z: s[2].parse().unwrap(),
            c: Some(i),
        };
        points.push(p);
        circuits.push(vec![i]);
    }

    let mut dist_pts: Vec<(u64, usize, usize)> = Vec::new();
    let ptslen = points.len();

    for i in 0..ptslen - 1 {
        for j in i + 1..ptslen {
            dist_pts.push((sq_dist_between(&points[i], &points[j]), i, j));
        }
    }
    dist_pts.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    let mut num_circuits = circuits.len();
    while num_circuits > 2 {
        let Some((_, idx1, idx2)) = dist_pts.pop() else {
            break;
        };
        let p1 = &points[idx1];
        let p2 = &points[idx2];
        let p1c = p1.c.unwrap();
        let p2c = p2.c.unwrap();
        if p1.c == p2.c {
            continue;
        } else {
            let circ2 = circuits[p2c].clone();
            for pidx in &circ2 {
                let p = points.get_mut(*pidx).unwrap();
                p.c = Some(p1c);
            }
            circuits[p1c].extend(circ2);
            circuits[p2c] = Vec::new();
            num_circuits -= 1;
        }
    }

    // Should have only 2 networks now, so we just have to find the last pair
    // of non-connected boxes.
    let mut out = 0;
    while let Some((_, p1, p2)) = dist_pts.pop() {
        if points[p1].c == points[p2].c {
            // println!("{:?} - {:?}", points[p1], points[p2]);
            continue;
        } else {
            out = points[p1].x * points[p2].x;
            break;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_part_one() {
        let ans = 40;
        assert_eq!(ans, solve_part1(EXAMPLE, 10));
    }
    #[test]
    fn test_part_two() {
        let ans = 25272;
        assert_eq!(ans, solve_part2(EXAMPLE));
    }
}

