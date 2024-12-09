use std::{collections::HashMap, fs::File, io::BufReader};

use helper_lib::utils::CharMap;

fn calculate_antinodes(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
) -> (Option<(usize, usize)>, Option<(usize, usize)>) {
    let (x1, y1) = (x1 as i64, y1 as i64);
    let (x2, y2) = (x2 as i64, y2 as i64);

    let dx = x2 - x1;
    let dy = y2 - y1;

    let pos1 = (x1 - dx, y1 - dy);
    let pos2 = (x2 + dx, y2 + dy);

    let p1;
    let p2;
    if pos1.0 >= 0 && pos1.1 >= 0 {
        p1 = Some((pos1.0 as usize, pos1.1 as usize));
    } else {
        p1 = None;
    }
    if pos2.0 >= 0 && pos2.1 >= 0 {
        p2 = Some((pos2.0 as usize, pos2.1 as usize));
    } else {
        p2 = None;
    }

    (p1, p2)
}

fn get_letter_positions(cmap: &CharMap) -> HashMap<char, Vec<(usize, usize)>> {
    let mut res = HashMap::new();

    for (row, col, c) in cmap.iter() {
        if c != '.' {
            // Either insert a new vec for char, or push to the existing one
            res.entry(c).or_insert(Vec::new()).push((row, col));
        }
    }

    res
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let mut antinodes = cmap.map_to_val(false);
    let letter_pos = get_letter_positions(&cmap);

    for positions in letter_pos.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let (p1, p2) = calculate_antinodes(positions[i], positions[j]);
                if let Some(p1) = p1 {
                    if cmap.is_valid_coords(p1) {
                        antinodes[p1.0][p1.1] = true;
                    }
                }
                if let Some(p2) = p2 {
                    if cmap.is_valid_coords(p2) {
                        antinodes[p2.0][p2.1] = true;
                    }
                }
            }
        }
    }
    /*
    println!();
    for i in 0..antinodes.len() {
        for j in 0..antinodes[i].len() {
            if antinodes[i][j] {
                print!("#");
            } else {
                print!("{}", cmap.get((i, j)).unwrap());
            }
        }
        println!();
    } */

    Ok(antinodes
        .iter()
        .map(|x| x.iter().filter(|y| **y).count())
        .sum::<usize>() as i64)
}

/* SOLUTION 2 */

fn calculate_antinodes_neg(
    (x1, y1): (i64, i64),
    (dx, dy): (i64, i64),
    (maxx, maxy): (i64, i64),
) -> Vec<(usize, usize)> {
    let mut x1 = x1;
    let mut y1 = y1;
    let mut res = Vec::new();

    while x1 >= 0 && y1 >= 0 && x1 < maxx && y1 < maxy {
        res.push((x1 as usize, y1 as usize));
        x1 -= dx;
        y1 -= dy;
    }

    res
}

fn calculate_antinodes_pos(
    (x1, y1): (i64, i64),
    (dx, dy): (i64, i64),
    (maxx, maxy): (i64, i64),
) -> Vec<(usize, usize)> {
    let mut x1 = x1;
    let mut y1 = y1;
    let mut res = Vec::new();

    while x1 >= 0 && y1 >= 0 && x1 < maxx && y1 < maxy {
        res.push((x1 as usize, y1 as usize));
        x1 += dx;
        y1 += dy;
    }

    res
}

fn calculate_antinodes_all(
    (x1, y1): (usize, usize),
    (x2, y2): (usize, usize),
    (maxx, maxy): (usize, usize),
) -> Vec<(usize, usize)> {
    let (x1, y1) = (x1 as i64, y1 as i64);
    let (x2, y2) = (x2 as i64, y2 as i64);

    let dx = x2 - x1;
    let dy = y2 - y1;

    let mut negs = calculate_antinodes_neg((x1, y1), (dx, dy), (maxx as i64, maxy as i64));
    let mut poss = calculate_antinodes_pos((x2, y2), (dx, dy), (maxx as i64, maxy as i64));

    negs.append(&mut poss);

    negs
}

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let mut antinodes = cmap.map_to_val(false);
    let letter_pos = get_letter_positions(&cmap);

    for positions in letter_pos.values() {
        for i in 0..positions.len() {
            for j in (i + 1)..positions.len() {
                let v1 = calculate_antinodes_all(
                    positions[i],
                    positions[j],
                    (cmap.map.len(), cmap.map[0].len()),
                );

                for (row, col) in v1.iter() {
                    antinodes[*row][*col] = true;
                }
            }
        }
    }
    /*
    println!();
    for i in 0..antinodes.len() {
        for j in 0..antinodes[i].len() {
            if antinodes[i][j] {
                print!("#");
            } else {
                print!("{}", cmap.get((i, j)).unwrap());
            }
        }
        println!();
    } */

    Ok(antinodes
        .iter()
        .map(|x| x.iter().filter(|y| **y).count())
        .sum::<usize>() as i64)
}
