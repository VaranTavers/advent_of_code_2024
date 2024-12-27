use std::{fs::File, io::BufReader};

use helper_lib::utils::{CharMap, Direction};

pub fn get_region(cmap: &CharMap, (i, j): (usize, usize)) -> (char, Vec<(usize, usize)>, usize) {
    let mut cmap = cmap.clone();
    let c = cmap.get((i, j)).unwrap();
    let mut res = Vec::new();
    let mut perim = 0;
    //println!("{i},{j}");
    let mut points = vec![(i, j)];
    while !points.is_empty() {
        let mut neigh = 0;
        let p = points.pop().unwrap();
        if cmap.map[p.0][p.1] != '.' {
            cmap.map[p.0][p.1] = '.';
            for dir in Direction::cardinal_directions() {
                if let Some(next_p) = dir.move_to(p) {
                    if cmap.is_valid_coords(next_p) {
                        if cmap.get(next_p).unwrap() == c {
                            //println!("NEW POINT {next_p:?}, {}", cmap.get(next_p).unwrap());
                            points.push(next_p);
                        } else if cmap.get(next_p).unwrap() != '.' {
                            neigh += 1;
                        }
                    } else {
                        neigh += 1;
                    }
                } else {
                    neigh += 1;
                }
            }
            perim += neigh;
            res.push(p);
        }
    }

    (c, res, perim)
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut cmap = CharMap::parse_map(reader);
    let mut regions = Vec::new();
    for i in 0..cmap.map.len() {
        for j in 0..cmap.map[i].len() {
            if cmap.map[i][j] != '/' {
                let new_region = get_region(&cmap, (i, j));
                for (ii, jj) in &new_region.1 {
                    cmap.map[*ii][*jj] = '/';
                }
                regions.push(new_region);
            }
        }
    }

    let mut sum = 0;
    for (_c, region, perim) in regions {
        //println!("{region:?}");
        //println!("{c}: {} {}", region.len(), perim);
        sum += region.len() * perim;
    }

    Ok(sum)
}

/* SOLUTION 2 */

/*
pub fn turn_90(a: &[i32]) -> [i32; 8] {
    [a[1], a[3], a[0], a[2], a[6], a[4], a[7], a[5]]
}

pub fn mirror(a: &[i32]) -> [i32; 8] {
    [a[0], a[2], a[1], a[3], a[5], a[4], a[7], a[6]]
}

pub fn are_eq(a: &[i32], b: &[i32]) -> bool {
    let mut b = turn_90(b);
    for _i in 0..4 {
        let d = a
            .iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .sum::<i32>();
        //println!("{a:?} {b:?}, {d} {i}");
        if d == 0 {
            return true;
        }
        b = turn_90(&b);
    }
    b = mirror(&b);
    for _i in 0..4 {
        let d = a
            .iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).abs())
            .sum::<i32>();
        //println!("{a:?} {b:?}, {d} {i}");
        if d == 0 {
            return true;
        }
        b = turn_90(&b);
    }

    return false;
}

const PENINSULA: [i32; 8] = [1, 1, 1, 0, 1, 1, 0, 0];
const CORNER: [i32; 8] = [1, 1, 0, 0, 1, 1, 1, 0];
const INSIDE_CORNER: [i32; 8] = [0, 0, 0, 0, 1, 0, 0, 0];
const INSIDE_OUTSIDE_CORNER: [i32; 8] = [0, 0, 1, 1, 1, 0, 0, 1];
const INSIDE_CORNER_P_FLAT_1: [i32; 8] = [0, 1, 0, 0, 0, 0, 1, 1];
const INSIDE_CORNER_P_FLAT_2: [i32; 8] = [0, 1, 0, 0, 1, 0, 1, 1];
const ALL_SIDES: [i32; 8] = [1, 1, 1, 1, 1, 1, 1, 1];
const ALMOST_PENINSULA: [i32; 8] = [1, 1, 1, 0, 1, 1, 1, 0];

pub fn get_lines(vals: &[i32]) -> i64 {
    if are_eq(vals, &PENINSULA) || are_eq(vals, &INSIDE_OUTSIDE_CORNER) {
        return 3;
    }
    if are_eq(vals, &CORNER)
        || are_eq(vals, &INSIDE_CORNER)
        || are_eq(vals, &INSIDE_CORNER_P_FLAT_1)
        || are_eq(vals, &INSIDE_CORNER_P_FLAT_2)
    {
        return 1;
    }
    if are_eq(vals, &ALL_SIDES) {
        return 4;
    }
    if are_eq(vals, &ALMOST_PENINSULA) {
        return 2;
    }
    0
}
    */

pub fn sum_lines(horiz: &[Vec<bool>], vert: &[Vec<bool>]) -> i32 {
    let mut sum_h = 0;

    for (i, line) in horiz.iter().enumerate() {
        let mut j = 0;
        while j < line.len() {
            while j < line.len() && !line[j] {
                j += 1;
            }
            if j < line.len() {
                sum_h += 1;
                // The strange 3rd case:
                // X X X|O
                //   --- -
                // X|O O|X
                //   --- X
                // X X X X

                println!("HORIZ: {i} {j}");

                while j < line.len()
                    && line[j]
                    && (i == 0 || i > horiz.len() - 2 || !vert[i - 1][j + 1] || !vert[i][j + 1])
                {
                    j += 1;
                }
                if j < line.len()
                    && i != 0
                    && i < horiz.len() - 1
                    && vert[i - 1][j + 1]
                    && vert[i][j + 1]
                {
                    j += 1;
                }
            }
        }
    }
    let mut sum_v = 0;
    for i in 0..vert[0].len() {
        let mut j = 0;
        while j < vert.len() {
            while j < vert.len() && !vert[j][i] {
                j += 1;
            }
            if j < vert.len() {
                sum_v += 1;
                // The strange 3rd case:
                // X X X|O
                //   --- -
                // X|O O|X
                //   --- X
                // X X X X

                println!("VERT: {i} {j}");
                while j < vert.len()
                    && vert[j][i]
                    && (i == 0 || i > vert[0].len() - 2 || !horiz[j + 1][i - 1] || !horiz[j + 1][i])
                {
                    j += 1;
                }
                if j < vert.len()
                    && i != 0
                    && i < vert[0].len() - 1
                    && horiz[j + 1][i - 1]
                    && horiz[j + 1][i]
                {
                    j += 1;
                }
            }
        }
    }

    println!("{sum_h} {sum_v}");
    sum_h + sum_v
}

pub fn get_region_2(cmap: &CharMap, (i, j): (usize, usize)) -> (char, Vec<(usize, usize)>, usize) {
    let mut cmap = cmap.clone();
    let mut horiz_lines = cmap.map_to_val(false);
    horiz_lines.push(horiz_lines[0].clone());
    let mut vert_lines = cmap.map_to_val(false);
    for l in &mut vert_lines {
        l.push(false);
    }

    let c = cmap.get((i, j)).unwrap();
    let mut res = Vec::new();
    //println!("{i},{j}");
    let mut points = vec![(i, j)];
    while !points.is_empty() {
        let p = points.pop().unwrap();
        if cmap.map[p.0][p.1] != '.' {
            cmap.map[p.0][p.1] = '.';
            for dir in Direction::cardinal_directions() {
                if let Some(next_p) = dir.move_to(p) {
                    if cmap.is_valid_coords(next_p) {
                        if cmap.get(next_p).unwrap() == c {
                            //println!("NEW POINT {next_p:?}, {}", cmap.get(next_p).unwrap());
                            points.push(next_p);
                            //println!("{next_p:?} PUSHED")
                        } else if cmap.get(next_p).unwrap() != '.' {
                            if dir == Direction::Top {
                                horiz_lines[p.0][p.1] = true;

                                //println!("{p:?} TOP");
                            } else if dir == Direction::Left {
                                vert_lines[p.0][p.1] = true;

                                //println!("{p:?} LEFT");
                            } else if dir == Direction::Bottom {
                                horiz_lines[p.0 + 1][p.1] = true;

                                //println!("{p:?} BOTTOM");
                            } else {
                                vert_lines[p.0][p.1 + 1] = true;

                                //println!("{p:?} RIGHT");
                            }
                        }
                    } else if dir == Direction::Bottom {
                        horiz_lines[p.0 + 1][p.1] = true;

                        //println!("{p:?} BOTTOM_MAP");
                    } else {
                        vert_lines[p.0][p.1 + 1] = true;

                        //println!("{p:?} RIGHT_MAP");
                    }
                } else if dir == Direction::Top {
                    horiz_lines[p.0][p.1] = true;
                    //println!("{p:?} TOP_MAP");
                } else {
                    vert_lines[p.0][p.1] = true;
                    //println!("{p:?} LEFT_MAP");
                }
            }

            res.push(p);
        }
    }

    for i in 0..(horiz_lines.len() - 1) {
        print!(" ");
        for j in 0..horiz_lines[i].len() {
            if horiz_lines[i][j] {
                print!("- ");
            } else {
                print!("  ");
            }
        }
        println!();
        for j in 0..vert_lines[i].len() {
            if vert_lines[i][j] {
                print!("| ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
    print!(" ");
    for j in &horiz_lines[horiz_lines.len() - 1] {
        if *j {
            print!("- ");
        } else {
            print!("  ");
        }
    }
    println!(); /**/

    (c, res, sum_lines(&horiz_lines, &vert_lines) as usize)
}

pub fn solution2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut cmap = CharMap::parse_map(reader);
    let mut regions = Vec::new();
    for i in 0..cmap.map.len() {
        for j in 0..cmap.map[i].len() {
            if cmap.map[i][j] != '/' {
                let new_region = get_region_2(&cmap, (i, j));
                for (ii, jj) in &new_region.1 {
                    cmap.map[*ii][*jj] = '/';
                }
                regions.push(new_region);
            }
        }
    }

    let mut sum = 0;
    for (c, region, perim) in regions {
        println!("{region:?}");
        println!("{c}: {} {}", region.len(), perim);
        sum += region.len() * perim;
    }

    Ok(sum)
}
