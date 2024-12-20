use std::{cmp::Reverse, collections::BinaryHeap, fs::File, io::BufReader};

use helper_lib::utils::{CharMap, To};

const MIN_SAVED: usize = 100;
const CHEAT_DIST: i64 = 20;

pub fn dijkstra_all(cmap: &CharMap, pos: (usize, usize)) -> Vec<Vec<Option<usize>>> {
    let mut points = BinaryHeap::new();
    points.push((Reverse(0), pos));
    let mut min_dist = cmap.map_to_val(None);
    min_dist[pos.0][pos.1] = Some(0);
    let mut prev: Vec<Vec<Option<(usize, usize)>>> = cmap.map_to_val(None);

    while !points.is_empty() {
        let (c_val, c_pos) = points.pop().unwrap();
        //println!("{c_val:?} {c_pos:?} {c_dir:?}");

        for n_dir in To::cardinal_directions() {
            if let Some(next_pos) = n_dir.move_to(c_pos) {
                if cmap.is_valid_coords(next_pos)
                    && min_dist[next_pos.0][next_pos.1].is_none()
                    && cmap.get(next_pos) != Some('#')
                {
                    min_dist[next_pos.0][next_pos.1] = Some(c_val.0 + 1);
                    prev[next_pos.0][next_pos.1] = Some(c_pos);
                    points.push((Reverse(c_val.0 + 1), next_pos));
                }
            }
        }
    }

    min_dist
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let start_pos = cmap.find_first('S').expect("No start found");
    let end_pos = cmap.find_first('E').expect("No end found");

    let start_ds = dijkstra_all(&cmap, start_pos);
    let end_ds = dijkstra_all(&cmap, end_pos);

    let mut cheats = Vec::new();
    for (row, col, c) in cmap.iter() {
        if row != 0 && col != 0 && row < cmap.map.len() - 1 && col < cmap.map[row].len() - 1 {
            if c == '#' {
                let mut free_pos = Vec::new();
                for dir in To::cardinal_directions() {
                    let next_pos = dir.move_to((row, col)).unwrap();
                    if cmap.get(next_pos).unwrap() != '#' {
                        free_pos.push(next_pos);
                    }
                }
                if free_pos.len() == 2 {
                    if start_ds[free_pos[0].0][free_pos[0].1].unwrap()
                        < start_ds[free_pos[1].0][free_pos[1].1].unwrap()
                    {
                        if start_ds[free_pos[0].0][free_pos[0].1].unwrap()
                            + 2
                            + end_ds[free_pos[1].0][free_pos[1].1].unwrap()
                            <= start_ds[end_pos.0][end_pos.1].unwrap() - MIN_SAVED
                        {
                            cheats.push((row, col));
                        }
                    } else {
                        if start_ds[free_pos[1].0][free_pos[1].1].unwrap()
                            + 2
                            + end_ds[free_pos[0].0][free_pos[0].1].unwrap()
                            <= start_ds[end_pos.0][end_pos.1].unwrap() - MIN_SAVED
                        {
                            cheats.push((row, col));
                        }
                    }
                }
            }
        }
    }

    Ok(cheats.len())
}

/* SOLUTION 2 */

pub fn solution2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let start_pos = cmap.find_first('S').expect("No start found");
    let end_pos = cmap.find_first('E').expect("No end found");

    let start_ds = dijkstra_all(&cmap, start_pos);
    let end_ds = dijkstra_all(&cmap, end_pos);

    let mut res = 0;
    //println!("{end_ds:?}");
    for (row, col, _c) in cmap.iter() {
        if row != 0 && col != 0 && row < cmap.map.len() - 1 && col < cmap.map[row].len() - 1 {
            if start_ds[row][col].is_some() {
                for i in -CHEAT_DIST..=CHEAT_DIST as i64 {
                    for j in -CHEAT_DIST..=CHEAT_DIST as i64 {
                        let ii = i + row as i64;
                        let jj = j + col as i64;
                        if i.abs() + j.abs() <= 20
                            && ii >= 0
                            && jj >= 0 as i64
                            && (ii as usize) < start_ds.len()
                            && (jj as usize) < start_ds[ii as usize].len()
                        {
                            let ii = ii as usize;
                            let jj = jj as usize;
                            //println!("{row} {col}");
                            if cmap.get((ii, jj)) != Some('#') {
                                if start_ds[row][col].unwrap() < start_ds[ii][jj].unwrap() {
                                    if start_ds[row][col].unwrap()
                                        + i.abs() as usize
                                        + j.abs() as usize
                                        + end_ds[ii][jj].unwrap()
                                        <= start_ds[end_pos.0][end_pos.1].unwrap() - MIN_SAVED
                                    {
                                        res += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /*for (row, col, c) in &cmap {
        if col == 0 {
            println!();
        }
        if cheats.contains(&(row, col)) {
            print!("O");
        } else {
            print!("{c}")
        }
    }
    println!();*/

    Ok(res)
}
