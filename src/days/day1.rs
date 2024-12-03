use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn get_lists_from_lines(lines: &mut Lines<BufReader<File>>) -> (Vec<i64>, Vec<i64>) {
    let mut v1 = Vec::new();
    let mut v2 = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let parts = line.split(' ').collect::<Vec<&str>>();
            //println!("{parts:?}");
            let num1 = parts[0].parse::<i64>().expect("NaN num 1");
            let num2 = parts[3].parse::<i64>().expect("NaN num 2");

            v1.push(num1);
            v2.push(num2);
        }
    }

    (v1, v2)
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let (mut v1, mut v2) = get_lists_from_lines(&mut reader.lines());
    v1.sort();
    v2.sort();
    Ok(v1.iter().zip(v2.iter()).map(|(x, y)| (y - x).abs()).sum())
}

/* SOLUTION 2 */

pub fn count_occurances(val: i64, v2: &[i64]) -> i64 {
    v2.iter().filter(|x| **x == val).count() as i64
}

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let (v1, v2) = get_lists_from_lines(&mut reader.lines());
    //v1.sort();
    //v2.sort();
    Ok(v1
        .iter()
        .zip(v1.iter().map(|x| count_occurances(*x, &v2)))
        .map(|(x, y)| x * y)
        .sum())
}
