use std::{
    fs::File,
    io::Write,
    io::{self, BufRead, BufReader},
};
fn num_of_digits(n: i128) -> usize {
    if n == 0 {
        return 1;
    }
    let mut db = 0;
    let mut n = n;
    while n > 0 {
        db += 1;
        n /= 10;
    }

    db
}

fn split_in_two(n: i128) -> (i128, i128) {
    let half = num_of_digits(n);

    let mut p = 1;
    for _ in 0..half / 2 {
        p *= 10;
    }

    (n / p, n % p)
}
/*
pub struct ListElem {
    value: i128,
    next: Option<Box<Self>>,
}

impl ListElem {
    pub fn new(value: i128, next: Option<Box<Self>>) -> Self {
        Self { value, next }
    }

    pub fn push(&mut self, value: i128) {
        if self.next.is_none() {
            self.next = Some(Box::new(ListElem::new(value, None)));
        } else {
            self.next.as_mut().unwrap().push(value);
        }
    }

    pub fn blink(&mut self) {
        if self.value == 0 {
            self.value = 1;
            if let Some(next) = &mut self.next {
                next.blink();
            }
        } else if num_of_digits(self.value) % 2 == 0 {
            let (a, b) = split_in_two(self.value);
            let mut old_next = self.next.take();
            if let Some(next) = &mut old_next {
                next.blink();
            }
            let next = Self::new(b, old_next);
            self.value = a;
            self.next = Some(Box::new(next));
        } else {
            self.value *= 2024;
            if let Some(next) = &mut self.next {
                next.blink();
            }
        }
    }

    pub fn len(&self) -> usize {
        if self.next.is_none() {
            return 1;
        }
        return 1 + self.next.as_ref().unwrap().len();
    }
}

pub fn solution(reader: BufReader<File>) -> Result<i128, std::io::Error> {
    let row = reader
        .lines()
        .next()
        .expect("No lines")
        .expect("Line read error")
        .split(' ')
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let mut list = ListElem::new(row[0], None);
    for val in row.iter().skip(1) {
        list.push(*val);
    }

    for _ in 0..6 {
        list.blink();
    }

    Ok(list.len() as i128)
}
    */

pub fn blink(vec: &mut Vec<i128>) {
    let mut l = vec.len();
    let mut i = 0;
    while i < l {
        if vec[i] == 0 {
            vec[i] = 1;
        } else if num_of_digits(vec[i]) % 2 == 0 {
            let (a, b) = split_in_two(vec[i]);
            vec[i] = a;
            vec.insert(i + 1, b);
            i += 1;
            l += 1;
        } else {
            vec[i] *= 2024;
        }

        i += 1;
    }
}

pub fn solution(reader: BufReader<File>) -> Result<i128, std::io::Error> {
    let mut row = reader
        .lines()
        .next()
        .expect("No lines")
        .expect("Line read error")
        .split(' ')
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    for _ in 0..25 {
        blink(&mut row);
    }

    Ok(row.len() as i128)
}

/* SOLUTION 2 */

pub fn read_cache_file() -> Option<Vec<Vec<i128>>> {
    let f = File::open("day11_cache.txt");
    if f.is_err() {
        return None;
    }
    let reader = BufReader::new(f.unwrap());

    Some(
        reader
            .lines()
            .map(|x| {
                x.unwrap()
                    .trim()
                    .split(' ')
                    .map(|x| x.parse::<i128>().unwrap())
                    .collect()
            })
            .collect(),
    )
}

pub fn write_cache_file(vals: &[Vec<i128>]) -> io::Result<()> {
    let mut output = File::create("day11_cache.txt")?;
    for val_list in vals {
        for val in val_list {
            write!(output, "{val} ")?;
        }
        writeln!(output)?;
    }
    Ok(())
}

pub fn calculate_number_of_rocks_after_n(val: i128, n: usize, rocks: &[Vec<i128>]) -> i128 {
    if n == 0 {
        return 1;
    }
    if val < 10 {
        return rocks[val as usize][n];
    }

    let mut v = vec![val];
    blink(&mut v);
    let mut s = 0;
    for val in v.iter() {
        s += calculate_number_of_rocks_after_n(*val, n - 1, rocks);
    }
    //println!("{s} {}", v.len());
    s
}

pub fn solution2(reader: BufReader<File>) -> Result<i128, std::io::Error> {
    let mut row = reader
        .lines()
        .next()
        .expect("No lines")
        .expect("Line read error")
        .split(' ')
        .map(|x| x.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let mut rocks;
    if let Some(read_vals) = read_cache_file() {
        rocks = read_vals;
    } else {
        let mut test = (0..10).map(|x| vec![x]).collect::<Vec<Vec<i128>>>();
        rocks = (0..10).map(|_| vec![1; 61]).collect::<Vec<Vec<i128>>>();

        for i in 0..test.len() {
            for j in 1..31 {
                blink(&mut test[i]);
                rocks[i][j] = test[i].len() as i128;
            }
        }
        for j in 1..31 {
            for i in 0..test.len() {
                println!("{i}, {j}");
                let mut sum = 0;
                for t in test[i].iter() {
                    let v = calculate_number_of_rocks_after_n(*t, j, &rocks);
                    //println!("{v}");
                    sum += v;
                }
                rocks[i][j + 30] = sum;
            }
        }
        write_cache_file(&rocks).expect("aaaa");
    }

    println!("Előfeldolgozás kész");

    for _ in 0..15 {
        blink(&mut row);
    }

    let mut s = 0;
    for v in row.iter() {
        s += calculate_number_of_rocks_after_n(*v, 60, &rocks);
    }

    Ok(s)
}
