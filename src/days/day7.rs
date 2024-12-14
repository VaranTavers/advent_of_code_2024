use std::{
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, Debug)]
pub enum Operator {
    Plus,
    Multiply,
    Concat,
}

pub fn read_row(line: &str) -> (i64, Vec<i64>) {
    let parts = line.split_once(':').unwrap();
    (
        parts.0.parse::<i64>().unwrap(),
        parts
            .1
            .trim()
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>(),
    )
}

pub fn read_rows(reader: BufReader<File>) -> Vec<(i64, Vec<i64>)> {
    reader
        .lines()
        .map(|x| read_row(x.unwrap().as_str()))
        .collect()
}

pub fn calculate_result(nums: &[i64], operators: &[Operator]) -> i64 {
    let mut s = nums[0];

    for (val, op) in nums.iter().skip(1).zip(operators) {
        match op {
            Operator::Plus => s += val,
            Operator::Multiply => s *= val,
            Operator::Concat => {
                let mut p = 10;
                while *val / p != 0 {
                    p *= 10;
                }
                s = s * p + val;
            }
        }
    }

    s
}

pub fn backtrack_operators(
    k: usize,
    ops: &mut Vec<Operator>,
    goal: i64,
    nums: &[i64],
    possible_ops: &[Operator],
) -> bool {
    if k == nums.len() - 1 {
        //println!("{ops:?} {nums:?} = {goal}");
        return calculate_result(nums, ops) == goal;
    }
    for op in possible_ops {
        ops.push(*op);
        if backtrack_operators(k + 1, ops, goal, nums, possible_ops) {
            return true;
        }
        ops.pop();
    }
    false
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let rows = read_rows(reader);

    let mut sum = 0;
    for (goal, nums) in &rows {
        let mut ops = Vec::new();
        if backtrack_operators(
            0,
            &mut ops,
            *goal,
            nums,
            &[Operator::Plus, Operator::Multiply],
        ) {
            //println!("{goal}");
            sum += goal;
        }
    }
    Ok(sum)
}

/* SOLUTION 2 */

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let rows = read_rows(reader);

    let mut sum = 0;
    for (goal, nums) in &rows {
        let mut ops = Vec::new();
        if backtrack_operators(
            0,
            &mut ops,
            *goal,
            nums,
            &[Operator::Plus, Operator::Multiply, Operator::Concat],
        ) {
            //println!("{goal}");
            sum += goal;
        }
    }
    Ok(sum)
}
