use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

use helper_lib::utils::{CharMap, To};

const NUM_PAD: &str = r#"789
456
123
.0A"#;

const WAY_PAD: &str = r#".^A
<v>"#;

pub fn dijkstra(cmap: &CharMap, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<To>> {
    let mut points = BinaryHeap::new();
    points.push((Reverse(0), start, Vec::new()));

    let mut res: Vec<Vec<To>> = Vec::new();

    while !points.is_empty() {
        let (c_val, c_pos, prev) = points.pop().unwrap();
        //println!("{prev:?}");
        if !res.is_empty() && res[0].len() < c_val.0 {
            //println!("BREAK: {:?} {c_val:?} {:?}", res, prev);
            //println!("{:?}", points);
            break;
        }
        if c_pos == end {
            if res.is_empty() || res[0].len() == c_val.0 {
                //println!("YES: {} {}", res.len(), c_val.0);
                res.push(prev);
            }
        } else {
            for n_dir in To::cardinal_directions() {
                if let Some(next_pos) = n_dir.move_to(c_pos) {
                    if cmap.is_valid_coords(next_pos) && cmap.get(next_pos) != Some('.') {
                        let mut prev_copy = prev.clone();
                        prev_copy.push(n_dir);
                        points.push((Reverse(c_val.0 + 1), next_pos, prev_copy));
                    }
                }
            }
        }
    }

    res
}

pub fn backtrack(
    k: usize,
    res: &mut Vec<Vec<To>>,
    options: &[Vec<Vec<To>>],
    nums: &mut Vec<usize>,
) {
    if k == options.len() - 1 {
        let mut res_l = Vec::new();
        for (i, x) in nums.iter().enumerate() {
            res_l.append(&mut options[i][*x].clone());
            res_l.push(To::TopLeft);
        }
        res.push(res_l);
        return;
    }
    for i in 0..options[k].len() {
        nums[k] = i;
        backtrack(k + 1, res, options, nums);
    }
}

// Repurpose TO:TopLeft to pushing
pub fn get_fastest_route_to_code(code: &str) -> Vec<Vec<To>> {
    let mut res = Vec::new();
    let cmap: CharMap = NUM_PAD.into();
    //println!("{cmap}");

    let mut chars: Vec<char> = code.chars().collect();
    chars.insert(0, 'A');

    let mut movements: Vec<Vec<Vec<To>>> = Vec::new();

    for (a, b) in chars.iter().zip(chars.iter().skip(1)) {
        let start_pos = cmap.find_first(*a).unwrap();
        let end_pos = cmap.find_first(*b).unwrap();

        let dirs = dijkstra(&cmap, start_pos, end_pos);
        movements.push(dirs);
    }

    println!("{movements:?}");

    let mut nums = movements.iter().map(|_x| 0).collect();
    backtrack(0, &mut res, &movements, &mut nums);

    res
}

pub fn get_fastest_route_to_path(code: &[char]) -> Vec<Vec<To>> {
    let mut res = Vec::new();
    let cmap: CharMap = WAY_PAD.into();
    //println!("{cmap}");

    let mut chars: Vec<char> = code.iter().cloned().collect();
    chars.insert(0, 'A');

    let mut movements: Vec<Vec<Vec<To>>> = Vec::new();

    //println!("{chars:?}");
    for (a, b) in chars.iter().zip(chars.iter().skip(1)) {
        let start_pos = cmap.find_first(*a).unwrap();
        let end_pos = cmap.find_first(*b).unwrap();

        let dirs = dijkstra(&cmap, start_pos, end_pos);
        movements.push(dirs);
    }

    let mut nums = movements.iter().map(|_x| 0).collect();
    backtrack(0, &mut res, &movements, &mut nums);

    res
}

pub fn to_to_char(to: &To) -> char {
    match to {
        To::Bottom => 'v',
        To::Left => '<',
        To::Top => '^',
        To::Right => '>',
        To::TopRight => '?',
        To::TopLeft => 'A',
        To::BottomRight => '?',
        To::BottomLeft => '?',
    }
}

pub fn tos_to_chars(vals: &[To]) -> Vec<char> {
    vals.iter().map(to_to_char).collect()
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut sum = 0;
    for line in reader.lines().map_while(Result::ok) {
        println!("{line}");
        let dirs_1 = get_fastest_route_to_code(&line)
            .iter()
            .map(|x| tos_to_chars(x))
            .collect::<Vec<Vec<char>>>();
        println!("{dirs_1:?}");
        let dirs_2 = dirs_1
            .iter()
            .map(|x| {
                get_fastest_route_to_path(&x)
                    .iter()
                    .map(|x| tos_to_chars(x))
                    .collect::<Vec<Vec<char>>>()
            })
            .flatten()
            .collect::<Vec<Vec<char>>>();
        //println!("{dirs_2:?}");
        let dirs_3 = dirs_2
            .iter()
            .map(|x| {
                get_fastest_route_to_path(&x)
                    .iter()
                    .map(|x| tos_to_chars(x))
                    .collect::<Vec<Vec<char>>>()
            })
            .flatten()
            .collect::<Vec<Vec<char>>>();
        //println!("{dirs_3:?}");
        let num_val = line.replace("A", "").parse::<usize>().unwrap();
        let shortest = dirs_3.iter().map(|x| x.len()).min().unwrap();
        println!("{} * {} = {}", shortest, num_val, shortest * num_val);
        sum += shortest * num_val;
    }
    Ok(sum)
}

/* SOLUTION 2 */

pub fn solution2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    Ok(0)
}
