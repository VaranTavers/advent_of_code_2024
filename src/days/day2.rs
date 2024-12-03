use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(PartialEq, Eq)]
pub enum Sorted {
    Ascending,
    Descending,
}

pub fn is_safe(list: &[i64]) -> bool {
    let sorted;
    if list[0] > list[1] {
        sorted = Sorted::Descending;
    } else if list[1] > list[0] {
        sorted = Sorted::Ascending;
    } else {
        return false;
    }

    for (a, b) in list.iter().zip(list.iter().skip(1)) {
        if (a >= b && sorted == Sorted::Ascending) || (a <= b && sorted == Sorted::Descending) {
            return false;
        }
        if (a - b).abs() > 3 || (a - b).abs() < 1 {
            return false;
        }
    }

    true
}

pub fn input(reader: BufReader<File>) -> Vec<Vec<i64>> {
    reader
        .lines()
        .map(|x| {
            x.unwrap()
                .split(' ')
                .map(|y| y.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>()
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let lines = input(reader);

    Ok(lines.iter().filter(|x| is_safe(x)).count() as i64)
}

/* SOLUTION 2 */

pub fn is_safe_dampened(v2: &[i64]) -> bool {
    let mut variations = (0..v2.len()).map(|x| {
        let mut res = v2.to_owned();
        res.remove(x);
        res
    });

    variations.any(|x| is_safe(&x))
}

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let lines = input(reader);

    Ok(lines.iter().filter(|x| is_safe_dampened(x)).count() as i64)
}
