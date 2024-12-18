use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fmt::format,
    fs::File,
    io::{BufRead, BufReader},
};

use helper_lib::utils::{CharMap, To};

const SIZE: usize = 71; // 71

pub fn get_path(pos: (usize, usize), prev: &[Vec<Option<(usize, usize)>>]) -> Vec<(usize, usize)> {
    let mut res = vec![pos];
    let mut pos = pos;

    while let Some(next_pos) = prev[pos.0][pos.1] {
        res.push(next_pos);
        pos = next_pos;
    }

    res.reverse();

    res
}

pub fn dijkstra(cmap: &CharMap, pos: (usize, usize)) -> (Vec<(usize, usize)>, usize) {
    let mut points = BinaryHeap::new();
    points.push((Reverse(0), pos, To::Right));
    let mut visited = cmap.map_to_val(false);
    visited[pos.0][pos.1] = true;
    let mut prev: Vec<Vec<Option<(usize, usize)>>> = cmap.map_to_val(None);

    while !points.is_empty() {
        let (c_val, c_pos, c_dir) = points.pop().unwrap();
        //println!("{c_val:?} {c_pos:?} {c_dir:?}");
        if c_pos == (SIZE - 1, SIZE - 1) {
            return (get_path(c_pos, &prev), c_val.0);
        }

        if let Some(next_pos) = c_dir.move_to(c_pos) {
            if cmap.is_valid_coords(next_pos)
                && !visited[next_pos.0][next_pos.1]
                && cmap.get(next_pos) != Some('#')
            {
                visited[next_pos.0][next_pos.1] = true;
                prev[next_pos.0][next_pos.1] = Some(c_pos);
                points.push((Reverse(c_val.0 + 1), next_pos, c_dir));
            }
        }

        if let Some(next_pos) = c_dir.turn_left_90().move_to(c_pos) {
            if cmap.is_valid_coords(next_pos)
                && !visited[next_pos.0][next_pos.1]
                && cmap.get(next_pos) != Some('#')
            {
                visited[next_pos.0][next_pos.1] = true;
                prev[next_pos.0][next_pos.1] = Some(c_pos);
                points.push((Reverse(c_val.0 + 1), next_pos, c_dir.turn_left_90()));
            }
        }

        if let Some(next_pos) = c_dir.turn_right_90().move_to(c_pos) {
            if cmap.is_valid_coords(next_pos)
                && !visited[next_pos.0][next_pos.1]
                && cmap.get(next_pos) != Some('#')
            {
                visited[next_pos.0][next_pos.1] = true;
                prev[next_pos.0][next_pos.1] = Some(c_pos);
                points.push((Reverse(c_val.0 + 1), next_pos, c_dir.turn_right_90()));
            }
        }
    }

    (Vec::new(), 0)
}

pub fn get_coords_from_line(line: &str) -> (usize, usize) {
    let vals = line.split_once(',').unwrap();

    (vals.0.parse().unwrap(), vals.1.parse().unwrap())
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let lines = reader.lines().map_while(Result::ok);
    let mut cmap = CharMap::from_size_char(SIZE, SIZE, '.');

    for line in lines.take(1024) {
        let coords = get_coords_from_line(&line);
        cmap.map[coords.0][coords.1] = '#';
    }

    let res = dijkstra(&cmap, (0, 0));

    Ok(res.1)
}

/* SOLUTION 2 */

pub fn solution2(reader: BufReader<File>) -> Result<String, std::io::Error> {
    let lines = reader.lines().map_while(Result::ok);
    let mut cmap = CharMap::from_size_char(SIZE, SIZE, '.');

    let mut final_byte = (SIZE, SIZE);
    for (i, line) in lines.enumerate() {
        let coords = get_coords_from_line(&line);
        cmap.map[coords.0][coords.1] = '#';
        if i >= 1024 {
            let res = dijkstra(&cmap, (0, 0));
            if res.1 == 0 {
                final_byte = coords;
                break;
            }
        }
    }

    Ok(format!("{},{}", final_byte.0, final_byte.1))
}
