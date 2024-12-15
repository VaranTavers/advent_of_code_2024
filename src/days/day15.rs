use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use helper_lib::utils::{CharMap, To};

pub fn move_pos(cmap: &mut CharMap, pos: (usize, usize), dir: To) -> (usize, usize) {
    let next_pos = dir.move_to(pos);
    if next_pos.is_none() {
        return pos;
    }
    let next_pos = next_pos.unwrap();
    match cmap.get(next_pos) {
        Some('.') => {
            cmap.map[next_pos.0][next_pos.1] = cmap.map[pos.0][pos.1];
            cmap.map[pos.0][pos.1] = '.';
            next_pos
        }
        Some('O') => {
            move_pos(cmap, next_pos, dir);
            match cmap.get(next_pos) {
                Some('.') => {
                    cmap.map[next_pos.0][next_pos.1] = cmap.map[pos.0][pos.1];
                    cmap.map[pos.0][pos.1] = '.';
                    next_pos
                }
                _ => pos,
            }
        }
        _ => pos,
    }
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let mut lines = reader
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();
    let moves = lines.split_off(lines.iter().position(|x| x.is_empty()).unwrap());

    let mut cmap = CharMap::parse_map_string(lines);

    //println!("Start: ");
    //println!("{cmap}");
    let mut robot_pos = cmap.find_first('@').expect("No robot found");
    for move_line in moves {
        for dir in move_line.chars().flat_map(|c| To::try_from(c)) {
            robot_pos = move_pos(&mut cmap, robot_pos, dir);
            //println!("Move {dir:?}");
            //println!("{cmap}");
        }
    }

    let mut sum = 0;
    for (row, col, c) in cmap.iter() {
        if c == 'O' {
            sum += row * 100 + col;
        }
    }

    Ok(sum as i64)
}

/* SOLUTION 2 */

pub fn check_feasibility(cmap: &mut CharMap, pos: (usize, usize), dir: To) -> bool {
    //println!("CF {pos:?} {dir:?}");
    let left_pos = if cmap.get(pos) == Some('[') {
        pos
    } else {
        (pos.0, pos.1 - 1)
    };
    let right_pos = (left_pos.0, left_pos.1 + 1);

    let next_left_pos = dir.move_to(left_pos).unwrap();
    let next_right_pos = dir.move_to(right_pos).unwrap();

    match cmap.get(next_left_pos) {
        Some('[') | Some(']') => {
            if !check_feasibility(cmap, next_left_pos, dir) {
                return false;
            }
        }
        Some('.') => {}
        _ => {
            return false;
        }
    }
    match cmap.get(next_right_pos) {
        Some('[') | Some(']') => {
            if !check_feasibility(cmap, next_right_pos, dir) {
                return false;
            }
        }
        Some('.') => {}
        _ => {
            return false;
        }
    }

    true
}

pub fn move_crate_updown(cmap: &mut CharMap, pos: (usize, usize), dir: To) {
    //println!("MOV: {pos:?}, {dir:?}");
    let left_pos = if cmap.get(pos) == Some('[') {
        pos
    } else {
        (pos.0, pos.1 - 1)
    };
    let right_pos = (left_pos.0, left_pos.1 + 1);

    let next_left_pos = dir.move_to(left_pos).unwrap();
    let next_right_pos = dir.move_to(right_pos).unwrap();

    match cmap.get(next_left_pos) {
        Some('[') | Some(']') => {
            move_crate_updown(cmap, next_left_pos, dir);
        }
        _ => {}
    }
    match cmap.get(next_right_pos) {
        Some('[') | Some(']') => {
            move_crate_updown(cmap, next_right_pos, dir);
        }
        _ => {}
    }
    //println!("Before move: ");
    //println!("{cmap}");
    cmap.map[next_left_pos.0][next_left_pos.1] = cmap.map[left_pos.0][left_pos.1];
    cmap.map[left_pos.0][left_pos.1] = '.';
    //println!("Afterm move left: ");
    //println!("{cmap}");
    cmap.map[next_right_pos.0][next_right_pos.1] = cmap.map[right_pos.0][right_pos.1];
    cmap.map[right_pos.0][right_pos.1] = '.';
    //println!("After move right: ");
    //println!("{cmap}");
}

pub fn move_crate(cmap: &mut CharMap, pos: (usize, usize), dir: To) -> (usize, usize) {
    //println!("{pos:?}");
    let next_pos = dir.move_to(pos);
    if next_pos.is_none() {
        return pos;
    }
    let next_pos = next_pos.unwrap();
    if dir == To::Right || dir == To::Left {
        return match cmap.get(next_pos) {
            Some('.') => {
                cmap.map[next_pos.0][next_pos.1] = cmap.map[pos.0][pos.1];
                cmap.map[pos.0][pos.1] = '.';
                next_pos
            }
            Some(']') | Some('[') => {
                move_crate(cmap, next_pos, dir);
                match cmap.get(next_pos) {
                    Some('.') => {
                        cmap.map[next_pos.0][next_pos.1] = cmap.map[pos.0][pos.1];
                        cmap.map[pos.0][pos.1] = '.';
                        next_pos
                    }
                    _ => pos,
                }
            }
            _ => pos,
        };
    } else {
        if check_feasibility(cmap, pos, dir) {
            move_crate_updown(cmap, pos, dir);
            return next_pos;
        }
        return pos;
    }
}

pub fn move_pos_2(cmap: &mut CharMap, pos: (usize, usize), dir: To) -> (usize, usize) {
    let next_pos = dir.move_to(pos);
    if next_pos.is_none() {
        return pos;
    }
    let next_pos = next_pos.unwrap();
    match cmap.get(next_pos) {
        Some('.') => {
            cmap.map[next_pos.0][next_pos.1] = cmap.map[pos.0][pos.1];
            cmap.map[pos.0][pos.1] = '.';
            next_pos
        }
        Some('[') | Some(']') => {
            move_crate(cmap, next_pos, dir);
            match cmap.get(next_pos) {
                Some('.') => {
                    cmap.map[next_pos.0][next_pos.1] = cmap.map[pos.0][pos.1];
                    cmap.map[pos.0][pos.1] = '.';
                    next_pos
                }
                _ => pos,
            }
        }
        _ => pos,
    }
}

pub fn duplicate_chars(s: &str) -> String {
    s.replace('.', "..")
        .replace('@', "@.")
        .replace('#', "##")
        .replace('O', "[]")
}

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let mut lines = reader
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();
    let moves = lines.split_off(lines.iter().position(|x| x.is_empty()).unwrap());

    lines = lines.iter().map(|s| duplicate_chars(s)).collect();
    let mut cmap = CharMap::parse_map_string(lines);

    //println!("Start: ");
    //println!("{cmap}");
    let mut robot_pos = cmap.find_first('@').expect("No robot found");
    for move_line in moves {
        for dir in move_line.chars().flat_map(|c| To::try_from(c)) {
            robot_pos = move_pos_2(&mut cmap, robot_pos, dir);
            //println!("Move {dir:?}");
            //println!("{cmap}");
        }
    }

    let mut sum = 0;
    for (row, col, c) in cmap.iter() {
        if c == '[' {
            sum += row * 100 + col;
        }
    }

    Ok(sum as i64)
}
