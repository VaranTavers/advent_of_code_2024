use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let regex = Regex::new("mul\\(([0-9]+),([0-9]+)\\)").unwrap();

    let mut sum = 0;
    for line in reader.lines() {
        for val in regex.captures_iter(&line.unwrap()) {
            let (_, parts): (&str, [&str; 2]) = val.extract();
            sum += parts[0].parse::<i64>().unwrap() * parts[1].parse::<i64>().unwrap();
        }
    }

    Ok(sum)
}

/* SOLUTION 2 */

pub fn get_closest_from_list(val: usize, list: &[usize]) -> usize {
    if list.is_empty() || list[0] > val {
        return 0;
    }
    for (v1, v2) in list.iter().zip(list.iter().skip(1)) {
        if *v2 > val {
            return *v1;
        }
    }

    *list.last().unwrap()
}

pub fn is_enabled(val: usize, dos: &[usize], donts: &[usize], base: bool) -> bool {
    if dos.is_empty() && donts.is_empty() {
        return base;
    }
    let a = get_closest_from_list(val, dos);
    let b = get_closest_from_list(val, donts);
    if a == b {
        return base;
    }

    a > b
}

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let regex = Regex::new("mul\\(([0-9]+),([0-9]+)\\)").unwrap();

    let mut enabled_for_row = true;
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let do_indices = line
            .match_indices("do()")
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        let dont_indices = line
            .match_indices("don't()")
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        //println!("{do_indices:?}");
        //println!("{dont_indices:?}");
        for val in regex.captures_iter(&line) {
            let start = val.get(0).unwrap().start();
            //println!("AA: {start}");
            if is_enabled(start, &do_indices, &dont_indices, enabled_for_row) {
                let (_, parts): (&str, [&str; 2]) = val.extract();
                /*println!(
                    "{}, {}",
                    parts[0].parse::<i64>().unwrap(),
                    parts[1].parse::<i64>().unwrap()
                );*/
                sum += parts[0].parse::<i64>().unwrap() * parts[1].parse::<i64>().unwrap();
            }
        }
        enabled_for_row =
            do_indices.last().copied().unwrap_or(0) > dont_indices.last().copied().unwrap_or(0);
    }

    Ok(sum)
}
