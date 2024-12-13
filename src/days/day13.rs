use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn gen_nums_from_line(line: &str, c: char) -> (usize, usize) {
    let p_parts = line.split(c).collect::<Vec<&str>>();
    let x_parts = p_parts[1].split(',').collect::<Vec<&str>>();

    (
        x_parts[0].parse::<usize>().unwrap(),
        p_parts[2].parse::<usize>().unwrap(),
    )
}

pub fn get_line(
    lines: &mut Lines<BufReader<File>>,
    needle: &str,
    op: char,
) -> Option<(usize, usize)> {
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

#[derive(Debug, Eq, PartialEq)]
pub struct ClawMachine {
    pub a: (usize, usize),
    pub b: (usize, usize),
    pub prize: (usize, usize),
}

impl ClawMachine {
    pub fn new(a: (usize, usize), b: (usize, usize), prize: (usize, usize)) -> Self {
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

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines();
    let mut machines = Vec::new();

    let mut m = ClawMachine::from_lines(&mut lines);
    while m.is_some() {
        machines.push(m.take());
        m = ClawMachine::from_lines(&mut lines);
    }

    println!("{machines:?}");
    let mut sum = 0;

    Ok(sum)
}

/* SOLUTION 2 */

pub fn solution2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut sum = 0;

    Ok(sum)
}
