use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn gen_nums_from_line(line: &str, c: char) -> (f64, f64) {
    let p_parts = line.split(c).collect::<Vec<&str>>();
    let x_parts = p_parts[1].split(',').collect::<Vec<&str>>();

    (
        x_parts[0].parse::<f64>().unwrap(),
        p_parts[2].parse::<f64>().unwrap(),
    )
}

pub fn get_line(lines: &mut Lines<BufReader<File>>, needle: &str, op: char) -> Option<(f64, f64)> {
    let row = lines.next();
    if row.is_none() {
        return None;
    }
    let mut row = row.unwrap().unwrap();
    while !row.contains(needle) {
        let row_opt = lines.next();
        if row_opt.is_none() {
            return None;
        }
        row = row_opt.unwrap().unwrap();
    }
    Some(gen_nums_from_line(&row, op))
}

#[derive(Debug, PartialEq)]
pub struct ClawMachine {
    pub a: (f64, f64),
    pub b: (f64, f64),
    pub prize: (f64, f64),
}

impl ClawMachine {
    pub fn new(a: (f64, f64), b: (f64, f64), prize: (f64, f64)) -> Self {
        Self { a, b, prize }
    }

    pub fn from_lines(lines: &mut Lines<BufReader<File>>) -> Option<ClawMachine> {
        let a = get_line(lines, "Button A", '+');
        let b = get_line(lines, "Button B", '+');
        let prize = get_line(lines, "Prize", '=');

        if a.is_none() || b.is_none() || prize.is_none() {
            return None;
        }

        Some(ClawMachine::new(a.unwrap(), b.unwrap(), prize.unwrap()))
    }
}

pub fn solution(reader: BufReader<File>) -> Result<f64, std::io::Error> {
    let mut lines = reader.lines();
    let mut machines = Vec::new();

    let mut m = ClawMachine::from_lines(&mut lines);
    while m.is_some() {
        machines.push(m.take());
        m = ClawMachine::from_lines(&mut lines);
    }

    println!("{machines:?}");
    let mut sum = 0.0;
    for m in machines.iter().flatten() {
        let b_val = (m.a.1 * m.prize.0 - m.a.0 * m.prize.1) / (m.a.1 * m.b.0 - m.b.1 * m.a.0);
        let a_val = (m.prize.0 - b_val * m.b.0) / m.a.0;
        println!("{a_val} {b_val}");
        if a_val.fract() == 0.0 && b_val.fract() == 0.0 {
            sum += a_val * 3.0 + b_val;
        }
    }

    Ok(sum)
}

/* SOLUTION 2 */

const VAL: f64 = 10000000000000.0;

pub fn solution2(reader: BufReader<File>) -> Result<f64, std::io::Error> {
    let mut lines = reader.lines();
    let mut machines = Vec::new();

    let mut m = ClawMachine::from_lines(&mut lines);
    while m.is_some() {
        machines.push(m.take());
        m = ClawMachine::from_lines(&mut lines);
    }

    println!("{machines:?}");
    let mut sum = 0.0;
    for m in machines.iter().flatten() {
        let b_val = (m.a.1 * (m.prize.0 + VAL) - m.a.0 * (m.prize.1 + VAL))
            / (m.a.1 * m.b.0 - m.b.1 * m.a.0);
        let a_val = (m.prize.0 + VAL - b_val * m.b.0) / m.a.0;
        println!("{a_val} {b_val}");
        if a_val.fract() == 0.0 && b_val.fract() == 0.0 {
            sum += a_val * 3.0 + b_val;
        }
    }

    Ok(sum)
}
