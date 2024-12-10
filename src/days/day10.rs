use std::{collections::HashSet, fs::File, io::BufReader};

use helper_lib::utils::{CharMap, To};

const TRAIL_CHARS: [char; 10] = ['9', '8', '7', '6', '5', '4', '3', '2', '1', '0'];

fn backtrack(
    k: usize,
    way: &mut Vec<(usize, usize)>,
    dirs: &[To],
    cmap: &CharMap,
    result: &mut HashSet<(usize, usize)>,
) {
    if k == TRAIL_CHARS.len() - 1 {
        result.insert(way.last().unwrap().clone());
        return;
    }
    for dir in dirs {
        let next_pos = dir.move_to(way.last().unwrap().clone());
        if let Some(next_pos) = next_pos {
            if cmap.is_valid_coords(next_pos) && cmap.get(next_pos).unwrap() == TRAIL_CHARS[k + 1] {
                way.push(next_pos);
                backtrack(k + 1, way, dirs, cmap, result);
                way.pop();
            }
        }
    }
}

pub fn find_zeros_accessible(cmap: &CharMap, (i, j): (usize, usize)) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    if cmap.get((i, j)).unwrap() == '9' {
        let mut way = Vec::new();
        way.push((i, j));
        backtrack(0, &mut way, &To::cardinal_directions(), cmap, &mut result);
        println!("{i} {j} {result:?}");
    }
    result
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let cmap = CharMap::parse_map(reader);
    let mut zeros_map = cmap.map_to_val(0 as i64);

    for (i, j, _) in cmap.iter() {
        let zs = find_zeros_accessible(&cmap, (i, j));
        for (row, col) in zs {
            zeros_map[row][col] += 1;
        }
    }

    Ok(zeros_map.iter().map(|x| x.iter().sum::<i64>()).sum::<i64>() as i64)
}

/* SOLUTION 2 */

fn backtrack_2(
    k: usize,
    way: &mut Vec<(usize, usize)>,
    dirs: &[To],
    cmap: &CharMap,
    result: &mut Vec<(usize, usize)>,
) {
    if k == TRAIL_CHARS.len() - 1 {
        result.push(way.last().unwrap().clone());
        return;
    }
    for dir in dirs {
        let next_pos = dir.move_to(way.last().unwrap().clone());
        if let Some(next_pos) = next_pos {
            if cmap.is_valid_coords(next_pos) && cmap.get(next_pos).unwrap() == TRAIL_CHARS[k + 1] {
                way.push(next_pos);
                backtrack_2(k + 1, way, dirs, cmap, result);
                way.pop();
            }
        }
    }
}

pub fn find_zeros_accessible_2(cmap: &CharMap, (i, j): (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    if cmap.get((i, j)).unwrap() == '9' {
        let mut way = Vec::new();
        way.push((i, j));
        backtrack_2(0, &mut way, &To::cardinal_directions(), cmap, &mut result);
        println!("{i} {j} {result:?}");
    }
    result
}

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let cmap = CharMap::parse_map(reader);
    let mut zeros_map = cmap.map_to_val(0 as i64);

    for (i, j, _) in cmap.iter() {
        let zs = find_zeros_accessible_2(&cmap, (i, j));
        for (row, col) in zs {
            zeros_map[row][col] += 1;
        }
    }

    Ok(zeros_map.iter().map(|x| x.iter().sum::<i64>()).sum::<i64>() as i64)
}
