use std::{cmp::Reverse, collections::BinaryHeap, fs::File, io::BufReader};

use helper_lib::utils::{CharMap, To};

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
        if cmap.get(c_pos) == Some('E') {
            return (get_path(c_pos, &prev), c_val.0);
        }

        let next_pos = c_dir
            .move_to(c_pos)
            .expect("Something bad happened, went off map");
        if !visited[next_pos.0][next_pos.1] && cmap.get(next_pos) != Some('#') {
            visited[next_pos.0][next_pos.1] = true;
            prev[next_pos.0][next_pos.1] = Some(c_pos);
            points.push((Reverse(c_val.0 + 1), next_pos, c_dir));
        }

        let next_pos = c_dir
            .turn_left_90()
            .move_to(c_pos)
            .expect("Something bad happened, went off map");
        if !visited[next_pos.0][next_pos.1] && cmap.get(next_pos) != Some('#') {
            visited[next_pos.0][next_pos.1] = true;
            prev[next_pos.0][next_pos.1] = Some(c_pos);
            points.push((Reverse(c_val.0 + 1001), next_pos, c_dir.turn_left_90()));
        }

        let next_pos = c_dir
            .turn_right_90()
            .move_to(c_pos)
            .expect("Something bad happened, went off map");
        if !visited[next_pos.0][next_pos.1] && cmap.get(next_pos) != Some('#') {
            visited[next_pos.0][next_pos.1] = true;
            prev[next_pos.0][next_pos.1] = Some(c_pos);
            points.push((Reverse(c_val.0 + 1001), next_pos, c_dir.turn_right_90()));
        }
    }

    (Vec::new(), 0)
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let start_pos = cmap.find_first('S').expect("No start found");

    let (_, res_len) = dijkstra(&cmap, start_pos);

    Ok(res_len)
}

/* SOLUTION 2 */

/*
pub fn has_multiple_ways(cmap: &CharMap, pos: (usize, usize)) -> bool {
    let mut s = 0;
    for dir in To::cardinal_directions() {
        let next_pos = dir.move_to(pos).unwrap();
        if cmap.get(next_pos) == Some('.') {
            s += 1;
        }
    }
    s > 2
}

pub fn dijkstra_all_best(
    cmap: &CharMap,
    start_pos: (usize, usize),
) -> Vec<(Vec<(usize, usize)>, usize)> {
    let mut res: Vec<(Vec<(usize, usize)>, usize)> = Vec::new();

    let base_best = dijkstra(cmap, start_pos);

    res.push(base_best.clone());

    for pos in base_best.0 {
        if has_multiple_ways(cmap, pos) {
            let mut c_cmap = cmap.clone();
            for dir in To::cardinal_directions() {
                let next_pos = dir.move_to(pos).unwrap();
                if cmap.get(next_pos) == Some('.') {
                    c_cmap.map[next_pos.0][next_pos.1] = '#';
                    let other_route = dijkstra(&c_cmap, start_pos);
                    if other_route.1 == base_best.1 {
                        res.push(other_route);
                    }
                    c_cmap.map[next_pos.0][next_pos.1] = cmap.map[next_pos.0][next_pos.1];
                }
            }
        }
    }

    res
}
 */

//Maybe all dists from all dirs?
pub fn dijkstra_all_dists(cmap: &CharMap, pos: (usize, usize)) -> Vec<Vec<[Option<usize>; 4]>> {
    let mut points = BinaryHeap::new();
    let mut dist = cmap.map_to(|_| [None; 4]);
    dist[pos.0][pos.1] = [Some(0); 4];

    if cmap.get(pos) == Some('S') {
        dist[pos.0][pos.1][To::Top.to_index()] = Some(1000);
        dist[pos.0][pos.1][To::Right.to_index()] = Some(0);
        points.push((Reverse(0), pos, To::Right));
        if cmap.get(To::Right.move_to(pos).unwrap()) == Some('.') {
            points.push((Reverse(1), To::Right.move_to(pos).unwrap(), To::Right));
        }
        if cmap.get(To::Top.move_to(pos).unwrap()) == Some('.') {
            points.push((Reverse(1001), To::Top.move_to(pos).unwrap(), To::Top));
        }
    } else {
        let next_dir = To::Left;
        let next_pos = next_dir.move_to(pos).unwrap();
        if cmap.get(next_pos) == Some('.') {
            dist[next_pos.0][next_pos.1][next_dir.to_index()] = Some(1);
            points.push((Reverse(1), next_pos, To::Left));
        }
        let next_dir = To::Bottom;
        let next_pos = next_dir.move_to(pos).unwrap();
        if cmap.get(next_pos) == Some('.') {
            dist[next_pos.0][next_pos.1][next_dir.to_index()] = Some(1);
            points.push((Reverse(1), next_pos, To::Bottom));
        }
    }

    while !points.is_empty() {
        let (c_val, c_pos, c_dir) = points.pop().unwrap();
        println!("{c_val:?} {c_pos:?} {c_dir:?}");

        let next_dir = c_dir;
        let next_pos = next_dir
            .move_to(c_pos)
            .expect("Something bad happened, went off map");
        if dist[next_pos.0][next_pos.1][next_dir.to_index()].is_none()
            && cmap.get(next_pos) != Some('#')
        {
            dist[next_pos.0][next_pos.1][next_dir.to_index()] = Some(c_val.0 + 1);
            points.push((Reverse(c_val.0 + 1), next_pos, next_dir));
            if next_pos.0 == 11 && next_pos.1 == 2 {
                println!(
                    " FRONT {points:?} {:?}",
                    dist[next_pos.0][next_pos.1][next_dir.to_index()]
                )
            }
        }

        let next_dir = c_dir.turn_left_90();
        let next_pos = next_dir
            .move_to(c_pos)
            .expect("Something bad happened, went off map");
        if dist[next_pos.0][next_pos.1][next_dir.to_index()].is_none()
            && cmap.get(next_pos) != Some('#')
        {
            dist[next_pos.0][next_pos.1][next_dir.to_index()] = Some(c_val.0 + 1001);
            points.push((Reverse(c_val.0 + 1001), next_pos, next_dir));
            if next_pos.0 == 11 && next_pos.1 == 2 {
                println!(
                    " LEFT {points:?} {:?}",
                    dist[next_pos.0][next_pos.1][next_dir.to_index()]
                )
            }
        }

        let next_dir = c_dir.turn_right_90();
        let next_pos = next_dir
            .move_to(c_pos)
            .expect("Something bad happened, went off map");
        if dist[next_pos.0][next_pos.1][next_dir.to_index()].is_none()
            && cmap.get(next_pos) != Some('#')
        {
            dist[next_pos.0][next_pos.1][next_dir.to_index()] = Some(c_val.0 + 1001);
            points.push((Reverse(c_val.0 + 1001), next_pos, next_dir));
            if next_pos.0 == 11 && next_pos.1 == 2 {
                println!(
                    " RIG_HT {points:?} {:?}",
                    dist[next_pos.0][next_pos.1][next_dir.to_index()]
                )
            }
        }
    }

    println!("POOOS:{:?} {:?}", pos, dist[pos.0 - 1][pos.1]);

    dist
}

pub fn is_on_road(dist_ss: &[Option<usize>], dist_es: &[Option<usize>], res_len: usize) -> bool {
    for (i, dist_s) in dist_ss.iter().enumerate() {
        if let Some(dist_s) = dist_s {
            println!("FROM_START: {dist_ss:?}");
            println!("FROM_END: {dist_es:?}");
            for (j, dist_e) in dist_es.iter().enumerate() {
                if let Some(dist_e) = dist_e {
                    if i == To::from_number(j + 1).turn_180().to_index() {
                        println!("{} = {} + {}", dist_s + dist_e, dist_s, dist_e);
                        if dist_s + dist_e == res_len {
                            return true;
                        }
                    } else if i != j {
                        if dist_s + dist_e + 1000 == res_len {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

pub fn solution2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let start_pos = cmap.find_first('S').expect("No start found");
    let end_pos = cmap.find_first('E').expect("No end found");

    let (_, res_len) = dijkstra(&cmap, start_pos);

    let all_dists_s = dijkstra_all_dists(&cmap, start_pos);
    let all_dists_e = dijkstra_all_dists(&cmap, end_pos);

    let mut s = 0;
    for (row, col, c) in &cmap {
        if c != '#' {
            let dist_s = all_dists_s[row][col];
            let dist_e = all_dists_e[row][col];
            println!("{row} {col}");
            if is_on_road(&dist_s, &dist_e, res_len) {
                print!("O");
                s += 1;
            } else {
                print!(".");
            }
        } else {
            print!("#");
        }
        if col == cmap.map[0].len() - 1 {
            println!();
        }
    }

    Ok(s)
}
