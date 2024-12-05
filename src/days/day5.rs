use std::{
    fs::File,
    io::{BufRead, BufReader},
};

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
