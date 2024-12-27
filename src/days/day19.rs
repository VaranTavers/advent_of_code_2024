use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_arrangements_set(line: &str) -> Vec<(String, usize)> {
    let mut res = Vec::new();
    for word in line.split(", ") {
        //if !word.contains('w') {
        // Only w is not in elemental form
        res.push((word.to_owned(), word.len()));
        //}
    }

    res
}

pub fn graph_alg(part: &[char], match_indices: &[(usize, usize)]) -> bool {
    let mut points = part.iter().map(|_| false).collect::<Vec<bool>>();
    points.push(true);
    for (s, l) in match_indices.iter().rev() {
        points[*s] |= points[*s + *l];
    }
    //println!("{points:?}");
    points[0]
}

pub fn is_valid(part: &str, set: &[(String, usize)]) -> bool {
    let mut match_indices = Vec::new();

    //println!("{set:?}");
    for (reg, len) in set {
        for val in part.match_indices(reg) {
            match_indices.push((val.0, *len));
        }
    }

    match_indices.sort_unstable();
    //println!("{match_indices:?}");

    let chars = part.chars().collect::<Vec<char>>();
    graph_alg(&chars, &match_indices)
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines().map_while(Result::ok);
    let hset = get_arrangements_set(&lines.next().unwrap());
    lines.next(); // Empty line

    let mut sum = 0;

    for line in lines {
        //println!("{line}");
        if is_valid(&line, &hset) {
            sum += 1;
        }
    }

    Ok(sum)
}

/* SOLUTION 2 */

pub fn graph_alg2(part: &[char], match_indices: &[(usize, usize)]) -> u128 {
    let mut points = part.iter().map(|_| 0_u128).collect::<Vec<u128>>();
    points.push(1);
    for (s, l) in match_indices.iter().rev() {
        points[*s] += points[*s + *l];
    }
    println!("{points:?}");
    points[0]
}

// Some help was required in order to realize that match_indices doesn't handle the "brbrbr" case well if you are searching for "brbr"
pub fn is_valid2(part: &str, set: &[(String, usize)]) -> u128 {
    let mut match_indices = Vec::new();

    let chars = part.chars().collect::<Vec<char>>();
    //println!("{set:?}");
    for (reg, len) in set.iter() {
        for i in 0..chars.len() {
            let (_a, b) = part.split_at(i);
            if b.starts_with(reg) {
                match_indices.push((i, *len));
            }
        }
    }

    match_indices.sort_unstable();
    //println!("{match_indices:?}");

    graph_alg2(&chars, &match_indices)
}

pub fn solution2(reader: BufReader<File>) -> Result<u128, std::io::Error> {
    let mut lines = reader.lines().map_while(Result::ok);
    let hset = get_arrangements_set(&lines.next().unwrap());
    lines.next(); // Empty line

    let mut sum = 0;

    for line in lines {
        //println!("{line}");
        sum += is_valid2(&line, &hset);
    }

    Ok(sum)
}
