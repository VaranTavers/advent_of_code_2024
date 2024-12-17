use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone)]
pub struct State<'a> {
    pub reg_a: usize,
    pub reg_b: usize,
    pub reg_c: usize,
    pub reg_ip: usize,
    pub code: &'a [u8],
    pub output: Vec<u8>,
}

impl<'a> State<'a> {
    pub fn new(reg_a: usize, reg_b: usize, reg_c: usize, code: &'a [u8]) -> Self {
        Self {
            reg_a,
            reg_b,
            reg_c,
            reg_ip: 0,
            code,
            output: Vec::new(),
        }
    }

    pub fn combo_operator(&self, val: u8) -> usize {
        match val {
            0..4 => val as usize,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Should not appear at all"),
        }
    }

    pub fn do_op(&mut self) -> bool {
        let opcode = self.code[self.reg_ip];
        let operand = self.code[self.reg_ip + 1];
        let mut jump = false;

        //eprintln!("{opcode} {operand}");
        match opcode {
            0 => {
                // adv: division A with 2^combo
                self.reg_a = self.reg_a / 2_usize.pow(self.combo_operator(operand) as u32);
            }
            1 => {
                // bxl bitwise xor of b and literal
                self.reg_b = self.reg_b ^ operand as usize;
            }
            2 => {
                // bst combo mod 8 to B
                self.reg_b = self.combo_operator(operand) % 8;
            }
            3 => {
                // jnz jumps to literal if A != 0
                if self.reg_a != 0 {
                    self.reg_ip = operand as usize;
                    jump = true;
                }
            }
            4 => {
                // bxc B xor C to B
                self.reg_b = self.reg_b ^ self.reg_c;
            }
            5 => {
                // out outputs combo mod 8
                self.output.push((self.combo_operator(operand) % 8) as u8);
            }
            6 => {
                //bdv division A with 2^combo to B
                self.reg_b = self.reg_a / 2_usize.pow(self.combo_operator(operand) as u32);
            }
            7 => {
                //cdv division A with 2^combo to C
                self.reg_c = self.reg_a / 2_usize.pow(self.combo_operator(operand) as u32);
            }
            _ => {
                panic!("Unknown opcode");
            }
        }

        if !jump {
            self.reg_ip += 2;
        }

        self.reg_ip < self.code.len()
    }
}

// Tests described in the excercise
#[test]
fn test1() {
    // If register C contains 9, the program 2,6 would set register B to 1.

    let code = [2, 6];
    let mut state = State::new(0, 0, 9, &code);
    state.do_op();
    assert_eq!(state.reg_b, 1);
}

#[test]
fn test2() {
    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.

    let code = [5, 0, 5, 1, 5, 4];
    let mut state = State::new(10, 0, 9, &code);
    while state.do_op() {}
    assert_eq!(state.output, vec![0, 1, 2]);
}
#[test]
fn test3() {
    // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.

    let code = [0, 1, 5, 4, 3, 0];
    let mut state = State::new(2024, 0, 9, &code);
    while state.do_op() {}
    assert_eq!(state.reg_a, 0);
    assert_eq!(state.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
}

#[test]
fn test4() {
    // If register B contains 29, the program 1,7 would set register B to 26.

    let code = [1, 7];
    let mut state = State::new(2024, 29, 9, &code);
    state.do_op();
    assert_eq!(state.reg_b, 26);
}
#[test]
fn test5() {
    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.

    let code = [4, 0];
    let mut state = State::new(2024, 2024, 43690, &code);
    state.do_op();
    assert_eq!(state.reg_b, 44354);
}

#[test]
fn test6() {
    // This program outputs a copy of itself if register A is instead initialized to 117440. (The original initial value of register A, 2024, is ignored.)

    let code = [0, 3, 5, 4, 3, 0];
    let mut state = State::new(117440, 0, 0, &code);
    while state.do_op() {}
    assert_eq!(state.output, code);
}

pub fn get_val_from_line(line: &str, needle: &str) -> Option<usize> {
    if !line.contains(needle) {
        return None;
    }

    line.split_once(':').map(|x| x.1.trim().parse().unwrap())
}

pub fn get_vals_from_line(line: &str, needle: &str) -> Option<Vec<u8>> {
    if !line.contains(needle) {
        return None;
    }

    line.split_once(':')
        .map(|x| x.1.trim().split(',').map(|y| y.parse().unwrap()).collect())
}

pub fn solution(reader: BufReader<File>) -> Result<String, std::io::Error> {
    let lines = reader
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();
    let reg_a = get_val_from_line(&lines[0], "Register A").expect("No Register A found");
    let reg_b = get_val_from_line(&lines[1], "Register B").expect("No Register B found");
    let reg_c = get_val_from_line(&lines[2], "Register C").expect("No Register C found");

    let code = get_vals_from_line(&lines[4], "Program").expect("No program found");
    let mut state = State::new(reg_a, reg_b, reg_c, &code);

    while state.do_op() {
        println!("A {}", state.reg_a);
        println!("B {}", state.reg_b);
        println!("C {}", state.reg_c);
        println!("");
    }

    Ok(state
        .output
        .iter()
        .map(|x| format!("{x}"))
        .collect::<Vec<String>>()
        .join(","))
}

/* SOLUTION 2 */

#[derive(Debug, PartialEq, Eq)]
pub struct ToBin(pub usize);

impl Display for ToBin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0 / 4 % 2, self.0 / 2 % 2, self.0 % 2)
    }
}

pub fn is_ok(reg_a: u64, state: &State) -> bool {
    let mut state = state.clone();
    state.reg_a = reg_a as usize;
    while state.do_op() {}

    state.output == state.code
}

// NOT a general solution, only works for the given input
pub fn backtracking(k: usize, reg_a: u64, state: &State) {
    if k == state.code.len() {
        //println!("MAYBE?{reg_a} {:?}", state.code);
        if is_ok(reg_a, state) {
            println!("SOLUTION: {reg_a}");
        }
        return;
    }
    let val = state.code.get(state.code.len() - 1 - k).unwrap();
    for b1 in 0..8 as u64 {
        let b2 = b1 ^ 5;
        for c in 0..8 {
            let mut b3 = b2 ^ 6;
            b3 = b3 ^ c;
            if b3 == *val as u64 {
                let calc_c = (reg_a * 8 + b1) / 2_u64.pow(b2 as u32) % 8;
                if calc_c == c {
                    //println!("k:{k} {reg_a} -> {b1} {c}");
                    backtracking(k + 1, reg_a * 8 + b1, state);
                }
            }
        }
    }
}

pub fn solution2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let lines = reader
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    let reg_a = get_val_from_line(&lines[0], "Register A").expect("No Register A found");
    let reg_b = get_val_from_line(&lines[1], "Register B").expect("No Register B found");
    let reg_c = get_val_from_line(&lines[2], "Register C").expect("No Register C found");

    let code = get_vals_from_line(&lines[4], "Program").expect("No program found");
    let state = State::new(reg_a, reg_b, reg_c, &code);

    backtracking(0, 0, &state);

    Ok(0)
}

/*println!("{}", ToBin(*val as usize));
for x in 0..8 {
    print!("{} ", ToBin(x));
}
println!();
let b3s = get_b3(*val as usize);
for x in &b3s {
    print!("{} ", ToBin(*x));
}
println!();
let b2s = b3s.iter().map(|x| get_b2(*x)).collect::<Vec<usize>>();
for x in &b2s {
    print!("{} ", ToBin(*x));
}
println!();
let b1s = b2s.iter().map(|x| get_b1(*x)).collect::<Vec<usize>>();
for x in &b1s {
    print!("{} ", ToBin(*x));
}
println!();
println!("______");*/
