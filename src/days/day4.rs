use std::{fs::File, io::BufReader};

use crate::helper_lib::utils::{CharMap, To};

const WORD: &str = "XMAS";

pub fn check_xmas(cmap: &CharMap, p: (usize, usize), dir: &To) -> i64 {
    let mut p = p;
    let mut opt_p = Some(p);
    for c in WORD.chars() {
        if opt_p.is_none() {
            return 0;
        }
        p = opt_p.unwrap();
        if !cmap.is_valid_coords(p) || cmap.map[p.0][p.1] != c {
            return 0;
        }

        opt_p = dir.move_to(p);
    }

    1
}

pub fn check_xmas_all(cmap: &CharMap, p: (usize, usize)) -> i64 {
    let dirs = To::all_directions();

    dirs.iter().map(|dir| check_xmas(cmap, p, dir)).sum()
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let mut s = 0;
    for (i, row) in cmap.map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let val = check_xmas_all(&cmap, (i, j));
            /*if val > 0 {
                println!("{i} {j} {val}");
            }*/
            s += val;
        }
    }

    Ok(s)
}

/* SOLUTION 2 */

pub fn is_x_mas(cmap: &CharMap, p: (usize, usize)) -> bool {
    if let Some(c) = cmap.get(p) {
        if c != 'A' {
            return false;
        }
    }

    let ps = To::x_directions().map(|dir| dir.move_to(p));
    if ps.iter().any(|x| x.is_none()) {
        //println!("dir err");
        return false;
    }
    let chars = ps.map(|p| cmap.get(p.unwrap()));
    if chars.iter().any(|x| x.is_none()) {
        //println!("char err");
        return false;
    }
    let chars = chars.map(|x| x.unwrap());

    let left_wing = (chars[0] == 'M' && chars[3] == 'S') || (chars[0] == 'S' && chars[3] == 'M');
    let right_wing = (chars[1] == 'M' && chars[2] == 'S') || (chars[1] == 'S' && chars[2] == 'M');
    //println!("{left_wing} {right_wing}");
    left_wing && right_wing
}

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let mut s = 0;
    for (i, row) in cmap.map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            //println!("{i}, {j}");
            if is_x_mas(&cmap, (i, j)) {
                s += 1;
            }
        }
    }

    Ok(s)
}
