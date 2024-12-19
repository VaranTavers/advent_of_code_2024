use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_hash_set(line: &str) -> HashSet<String> {
    let mut res = HashSet::new();
    for word in line.split(", ") {
        res.insert(word.to_owned());
    }

    res
}

pub fn is_valid(part: &str, set: &mut HashSet<String>, tabs: usize) -> bool {
    //println!("{}{part}", "\t".repeat(tabs));
    if set.contains(part) {
        return true;
    }
    if !part.contains('w') {
        // only white is not elemental
        return true;
    }
    for i in 1..part.len() {
        let parts = part.split_at(i);
        if is_valid(parts.0, set, tabs + 1) && is_valid(parts.1, set, tabs + 1) {
            //set.insert(parts.0.to_owned());
            //set.insert(parts.1.to_owned());
            return true;
        }
    }

    return false;
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines().map_while(Result::ok);
    let mut hset = get_hash_set(&lines.next().unwrap());
    lines.next(); // Empty line

    println!("PRE_DONE");

    let mut sum = 0;

    for line in lines {
        println!("{line}");
        if is_valid(&line, &mut hset, 0) {
            sum += 1;
        }
    }

    Ok(sum)
}

/* SOLUTION 2 */

// Interesting solutions read after bruteforcing it: only recalculate if a block landed on the best route, go backwards and see when is the first route possible

pub fn solution2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    Ok(0)
}
