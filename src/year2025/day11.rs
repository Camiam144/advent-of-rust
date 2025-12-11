use std::collections::{HashMap, HashSet};

use crate::load_input;
use anyhow::Result;

pub fn solve() -> Result<()> {
    let input = load_input(2025, 11)?;
    let start = std::time::Instant::now();
    let part1 = solve_part1(&input);
    let p1 = std::time::Instant::now();
    println!(
        "Day 11 part 1: {} in {} μs",
        part1,
        (p1 - start).as_micros()
    );
    let start2 = std::time::Instant::now();
    let part2 = solve_part2(&input);
    let p2 = std::time::Instant::now();
    println!(
        "Day 11 part 2: {} in {} μs",
        part2,
        (p2 - start2).as_micros()
    );
    Ok(())
}

fn dfs_p1(
    curr: &str,
    end: &str,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if curr == end {
        return 1;
    }

    if let Some(&val) = cache.get(curr) {
        return val;
    }

    let mut res = 0;
    if let Some(next_vals) = graph.get(curr) {
        for nv in next_vals {
            res += dfs_p1(nv, end, graph, cache);
        }
    }
    cache.insert(curr.to_string(), res);
    res
}

fn solve_part1(input: &str) -> u64 {
    let puzz: Vec<(String, Vec<String>)> = input
        .lines()
        .map(|l| {
            let mut split = l.split_ascii_whitespace();
            let key: String = split.next().unwrap().chars().take(3).collect();
            let vals: Vec<String> = split.map(|s| s.to_string()).collect();
            (key, vals)
        })
        .collect();
    let mut instructions: HashMap<String, Vec<String>> = HashMap::with_capacity(puzz.len());
    for (k, v) in puzz {
        instructions.insert(k, v);
    }
    let mut cache: HashMap<String, u64> = HashMap::new();
    dfs_p1("you", "out", &instructions, &mut cache)
}

fn solve_part2(input: &str) -> u64 {
    let puzz: Vec<(String, Vec<String>)> = input
        .lines()
        .map(|l| {
            let mut split = l.split_ascii_whitespace();
            let key: String = split.next().unwrap().chars().take(3).collect();
            let vals: Vec<String> = split.map(|s| s.to_string()).collect();
            (key, vals)
        })
        .collect();
    let mut instructions: HashMap<String, Vec<String>> = HashMap::with_capacity(puzz.len());
    for (k, v) in puzz {
        instructions.insert(k, v);
    }

    // Dumb brute force solution tracking path doesn't finish in a reasonable time
    // We know there are no loops, so either FFT or DAC must be first
    // There's a better memoization pattern but I was never going to find it myself

    let mut cache1: HashMap<String, u64> = HashMap::new();
    let mut cache2: HashMap<String, u64> = HashMap::new();
    let mut cache3: HashMap<String, u64> = HashMap::new();

    let paths_svr_fft = dfs_p1("svr", "fft", &instructions, &mut cache1);
    // println!("Paths svr to fft {}", paths_svr_fft);
    let paths_fft_dac = dfs_p1("fft", "dac", &instructions, &mut cache2);
    // println!("Paths fft to dac {}", paths_fft_dac);
    let paths_dac_end = dfs_p1("dac", "out", &instructions, &mut cache3);
    // println!("Paths dac to out {}", paths_dac_end);

    paths_svr_fft * paths_fft_dac * paths_dac_end
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part_one() {
        let ans = 5;
        assert_eq!(ans, solve_part1(EXAMPLE));
    }
    #[test]
    fn test_part_two() {
        let ans = 2;
        assert_eq!(ans, solve_part2(EXAMPLE2));
    }
}
