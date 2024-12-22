use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

trait MixAndPrune {
    fn mix(self, other: Self) -> Self;

    fn prune(self) -> Self;
}

impl MixAndPrune for u64 {
    fn mix(self, other: Self) -> Self {
        self ^ other
    }

    fn prune(self) -> Self {
        self % 16_777_216
    }
}

pub fn next_secret_number(num: u64) -> u64 {
    let mult: u64 = num.mix(num * 64).prune();

    let div = mult.mix(mult / 32).prune();
    let mult2 = div.mix(div * 2048).prune();

    mult2
}

pub fn solution(reader: BufReader<File>) -> Result<u64, std::io::Error> {
    let lines = reader.lines().map_while(Result::ok);
    let mut sum = 0;
    for line in lines {
        let mut num = line.parse::<u64>().unwrap();
        for _ in 0..2000 {
            num = next_secret_number(num);
        }
        sum += num;
    }
    Ok(sum)
}

/* SOLUTION 2 */

pub fn test_solution_one(sol: &[i64], diffs: &[i64], vals: &[u64]) -> u64 {
    for (i, window) in diffs.windows(sol.len()).enumerate() {
        if sol == window {
            return vals[i + window.len()] % 10;
        }
    }
    return 0;
}

pub fn test_solution(sol: &[i64], all_diffs: &[Vec<i64>], all_vals: &[Vec<u64>]) -> u64 {
    let mut sum = 0;
    for (vals, diffs) in all_vals.iter().zip(all_diffs) {
        sum += test_solution_one(sol, diffs, vals);
    }

    sum
}

pub fn solution2(reader: BufReader<File>) -> Result<u64, std::io::Error> {
    let lines = reader.lines().map_while(Result::ok);
    let mut all_vals = Vec::new();
    let mut all_diffs = Vec::new();

    for line in lines {
        let mut num = line.parse::<u64>().unwrap();
        let mut vals = Vec::new();
        let mut diffs = Vec::new();
        vals.push(num);
        for _ in 0..2000 {
            num = next_secret_number(num);
            diffs.push((num % 10) as i64 - (vals.last().unwrap() % 10) as i64);
            vals.push(num);
        }
        all_vals.push(vals);
        all_diffs.push(diffs);
    }

    let mut hmaps: Vec<HashMap<&[i64], u64>> = Vec::new();

    for (i, diffs) in all_diffs.iter().enumerate() {
        let mut hmap = HashMap::new();
        for (j, window) in diffs.windows(4).enumerate() {
            if !hmap.contains_key(window) {
                hmap.insert(window, all_vals[i][j + window.len()] % 10);
            }
        }
        hmaps.push(hmap);
    }

    let mut large_hmap = HashMap::new();

    for hmap in &hmaps {
        for (k, v) in hmap.iter() {
            let entry = large_hmap.entry(k).or_insert(0);
            *entry += v;
        }
    }

    Ok(*large_hmap.values().max().unwrap())
}
