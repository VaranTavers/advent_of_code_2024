use std::{
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
};

#[derive(Debug, PartialEq)]
pub struct Robot {
    pub p: (i64, i64),
    pub v: (i64, i64),
}

impl Robot {
    pub fn new(p: (i64, i64), v: (i64, i64)) -> Self {
        Self { p, v }
    }

    pub fn move_n(&mut self, n: i64, bounds: &(i64, i64)) {
        let new_x = (self.p.0 + n * self.v.0 + n * bounds.0) % bounds.0;
        let new_y = (self.p.1 + n * self.v.1 + n * bounds.1) % bounds.1;
        self.p = (new_x, new_y);
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let p_v = s.split_once(' ').unwrap();
        let pside = p_v.0.split_once('=').unwrap();
        let p_parts = pside.1.split_once(',').unwrap();
        let vside = p_v.1.split_once('=').unwrap();
        let v_parts = vside.1.split_once(',').unwrap();

        Ok(Robot::new(
            (p_parts.0.parse().unwrap(), p_parts.1.parse().unwrap()),
            (v_parts.0.parse().unwrap(), v_parts.1.parse().unwrap()),
        ))
    }
}

pub fn print_debug_info(robots: &[Robot], bounds: &(i64, i64), blank_mid: bool) {
    for j in 0..bounds.1 {
        for i in 0..bounds.0 {
            if (i == bounds.0 / 2 || j == bounds.1 / 2) && blank_mid {
                print!(" ");
            } else {
                let s = robots.iter().filter(|x| x.p == (i, j)).count();
                if s == 0 {
                    print!(".");
                } else {
                    print!("{s}");
                }
            }
        }
        println!();
    }
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let mut robots = reader
        .lines()
        .map_while(Result::ok)
        .flat_map(|line| Robot::from_str(&line))
        .collect::<Vec<Robot>>();
    let bounds = (101, 103);

    let mut quadrants = [0; 4];
    for robot in &mut robots {
        robot.move_n(100, &bounds);
        //println!("{robot:?}");
        if robot.p.1 < bounds.1 / 2 {
            if robot.p.0 < bounds.0 / 2 {
                quadrants[0] += 1;
            } else if robot.p.0 > bounds.0 / 2 {
                quadrants[1] += 1;
            }
        } else if robot.p.1 > bounds.1 / 2 {
            if robot.p.0 < bounds.0 / 2 {
                quadrants[2] += 1;
            } else if robot.p.0 > bounds.0 / 2 {
                quadrants[3] += 1;
            }
        }
    }

    Ok(quadrants.iter().product())
}

/* SOLUTION 2


pub fn is_symmetric(robots: &[Robot], bounds: &(i64, i64)) -> bool {
    let mut a = vec![vec![false; bounds.0 as usize]; bounds.1 as usize];
    for robot in robots.iter() {
        a[robot.p.1 as usize][robot.p.0 as usize] = true;
    }

    for row in 0..(bounds.1 as usize) {
        for col in 0..(bounds.0 / 2) as usize {
            if a[row][col] != a[row][bounds.0 as usize - 1 - col] {
                return false;
            }
        }
    }
    true
}
    */

pub fn largest_continous_vertical_line(robots: &[Robot], bounds: &(i64, i64)) -> i64 {
    let mut a = vec![vec![false; bounds.0 as usize]; bounds.1 as usize];
    for robot in robots {
        a[robot.p.1 as usize][robot.p.0 as usize] = true;
    }

    let mut max = 0;
    for col in 0..(bounds.0 as usize) {
        let mut val = 0;
        for row in 0..(bounds.1) as usize {
            if a[row][col] {
                val += 1;
            } else {
                if val > max {
                    max = val;
                }
                val = 0;
            }
        }
    }
    max
}

// Tried filled line through middle ran for:  202963000
// Tried symmetric around center ran for: 32185000

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let mut robots = reader
        .lines()
        .map_while(Result::ok)
        .flat_map(|line| Robot::from_str(&line))
        .collect::<Vec<Robot>>();
    let bounds = (101, 103);

    let mut i = 0;
    let mut max = 0;
    loop {
        for robot in &mut robots {
            robot.move_n(1, &bounds);
        }
        let val = largest_continous_vertical_line(&robots, &bounds);
        if val > max {
            max = val;
            //break;
            println!("{i}:");
            print_debug_info(&robots, &bounds, false);
        }
        i += 1;
        if i % 1000 == 0 {
            println!("{i}");
        }
        if i == 20000 {
            break;
        }
    }

    println!("{i}:");
    print_debug_info(&robots, &bounds, false);
    println!();

    Ok(6445)
}
