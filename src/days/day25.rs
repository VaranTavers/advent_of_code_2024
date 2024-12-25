use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

const LOG: bool = false;

macro_rules! log {
    ($($x:tt)*) => { if LOG { print!($($x)*) } }
}

macro_rules! logln {
    ($($x:tt)*) => { if LOG { println!($($x)*) } }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Key {
    pub heights: [usize; 5],
}

impl Key {
    pub fn from_lines(lines: &[String]) -> Self {
        let map: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

        let mut res = Self {
            heights: [0, 0, 0, 0, 0],
        };
        for col in 0..5 {
            /*let mut row = 5;
            while row > 0 && map[row - 1][col] == '#' {
                row -= 1;
            }
            res.heights[col] = row;*/
            for row in 0..5 {
                if map[row][col] == '#' {
                    res.heights[col] += 1;
                }
            }
        }

        res
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Lock {
    pub heights: [usize; 5],
}

impl Lock {
    pub fn from_lines(lines: &[String]) -> Self {
        let map: Vec<Vec<char>> = lines.iter().map(|x| x.chars().collect()).collect();

        let mut res = Self {
            heights: [0, 0, 0, 0, 0],
        };
        for col in 0..5 {
            /*let mut row = 0;
            while row < 5 && map[row][col] == '#' {
                row += 1;
            }
            res.heights[col] = row - 1;*/
            for row in 0..5 {
                if map[row][col] == '#' {
                    res.heights[col] += 1;
                }
            }
        }

        res
    }
}

pub fn fits(key: &Key, lock: &Lock) -> bool {
    for (k, l) in key.heights.iter().zip(lock.heights.iter()) {
        if k + l > 5 {
            // !=
            return false;
        }
    }

    true
}

#[derive(Debug, PartialEq, Eq)]
pub enum Object {
    Key,
    Lock,
    Unknown,
}

pub fn read_locks_and_keys(mut lines: Lines<BufReader<File>>) -> (HashSet<Lock>, HashSet<Key>) {
    let mut rows = Vec::new();
    let mut obj = Object::Unknown;
    let mut keys = HashSet::new();
    let mut locks = HashSet::new();
    while let Some(line) = lines.next() {
        let line = line.expect("Something happened");
        if line.is_empty() {
            logln!("Empty line");
            match obj {
                Object::Key => {
                    logln!("Created a key");
                    keys.insert(Key::from_lines(&rows));
                }
                Object::Lock => {
                    logln!("Created a lock");
                    locks.insert(Lock::from_lines(&rows));
                }
                Object::Unknown => println!("What did you do?"),
            }
            obj = Object::Unknown;
            rows = Vec::new();
        } else {
            if obj == Object::Unknown {
                if line.starts_with("#") {
                    logln!("Found a lock");
                    obj = Object::Lock;
                } else {
                    logln!("Found a key");
                    obj = Object::Key;
                }
            } else {
                rows.push(line);
            }
        }
    }

    (locks, keys)
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines();

    let (locks, keys) = read_locks_and_keys(lines);

    logln!("{locks:?}");
    logln!("{keys:?}");
    let mut s = 0;
    for lock in &locks {
        for key in &keys {
            if fits(key, lock) {
                s += 1;
            }
        }
    }

    Ok(s)
}

/* SOLUTION 2 */

pub fn solution2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    Ok(0)
}
