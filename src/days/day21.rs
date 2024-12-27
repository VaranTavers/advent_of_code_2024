use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    fs::File,
    io::{BufRead, BufReader},
};

const LOG: bool = false;
use helper_lib::logln;

use helper_lib::utils::{CharMap, Direction};

const NUM_PAD: &str = r"789
456
123
.0A";

const WAY_PAD: &str = r".^A
<v>";

pub fn dijkstra(cmap: &CharMap, start: (usize, usize), end: (usize, usize)) -> Vec<Vec<Direction>> {
    let mut points = BinaryHeap::new();
    points.push((Reverse(0), start, Vec::new()));

    let mut res: Vec<Vec<Direction>> = Vec::new();

    while !points.is_empty() {
        let (c_val, c_pos, prev) = points.pop().unwrap();
        //logln!("{prev:?}");
        if !res.is_empty() && res[0].len() < c_val.0 {
            //logln!("BREAK: {:?} {c_val:?} {:?}", res, prev);
            //logln!("{:?}", points);
            break;
        }
        if c_pos == end {
            if res.is_empty() || res[0].len() == c_val.0 {
                //logln!("YES: {} {}", res.len(), c_val.0);
                res.push(prev);
            }
        } else {
            for n_dir in Direction::cardinal_directions() {
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
    res: &mut Vec<Vec<Direction>>,
    options: &[Vec<Vec<Direction>>],
    nums: &mut Vec<usize>,
) {
    if options.len() == 0 {
        //logln!("Baj van!");
        return;
    }
    if k == options.len() {
        let mut res_l = Vec::new();
        for (i, x) in nums.iter().enumerate() {
            res_l.append(&mut options[i][*x].clone());
            res_l.push(Direction::TopLeft);
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
pub fn get_fastest_route_to_code(code: &str) -> Vec<Vec<Direction>> {
    let mut res = Vec::new();
    let cmap: CharMap = NUM_PAD.into();
    //logln!("{cmap}");

    let mut chars: Vec<char> = code.chars().collect();
    chars.insert(0, 'A');

    let mut movements: Vec<Vec<Vec<Direction>>> = Vec::new();

    for (a, b) in chars.iter().zip(chars.iter().skip(1)) {
        let start_pos = cmap.find_first(*a).unwrap();
        let end_pos = cmap.find_first(*b).unwrap();

        let dirs = dijkstra(&cmap, start_pos, end_pos);
        movements.push(dirs);
    }

    //logln!("{movements:?}");

    let mut nums = movements.iter().map(|_x| 0).collect();
    backtrack(0, &mut res, &movements, &mut nums);

    res
}

pub fn get_fastest_route_to_path(code: &[char], add_a: bool) -> Vec<Vec<Direction>> {
    let mut res = Vec::new();
    let cmap: CharMap = WAY_PAD.into();
    //logln!("{cmap}");

    let mut chars: Vec<char> = code.iter().copied().collect();
    if add_a {
        chars.insert(0, 'A');
    }

    if code.len() == 1 {
        return vec![vec![Direction::TopLeft]];
    }

    let mut movements: Vec<Vec<Vec<Direction>>> = Vec::new();

    //logln!("{chars:?}");
    for (a, b) in chars.iter().zip(chars.iter().skip(1)) {
        let start_pos = cmap.find_first(*a).unwrap();
        let end_pos = cmap.find_first(*b).unwrap();

        let dirs = dijkstra(&cmap, start_pos, end_pos);
        //logln!("{dirs:?}");
        movements.push(dirs);
    }

    let mut nums = movements.iter().map(|_x| 0).collect();
    backtrack(0, &mut res, &movements, &mut nums);
    //logln!("REEESS: {res:?}");
    res
}

pub fn to_to_char(to: &Direction) -> char {
    match to {
        Direction::Bottom => 'v',
        Direction::Left => '<',
        Direction::Top => '^',
        Direction::Right => '>',
        Direction::TopRight => '?',
        Direction::TopLeft => 'A',
        Direction::BottomRight => '?',
        Direction::BottomLeft => '?',
    }
}

pub fn tos_to_chars(vals: &[Direction]) -> Vec<char> {
    vals.iter().map(to_to_char).collect()
}

pub fn run_calculation(dirs: &[Direction], k: usize) -> usize {
    let mut dirs_chars = vec![tos_to_chars(dirs)];

    //logln!("0: {dirs_chars:?}");
    for i in 0..k {
        let dirs_2 = dirs_chars
            .iter()
            .flat_map(|x| {
                get_fastest_route_to_path(x, i != 0)
                    .iter()
                    .map(|x| tos_to_chars(x))
                    .collect::<Vec<Vec<char>>>()
            })
            .collect::<Vec<Vec<char>>>();
        dirs_chars = dirs_2;

        //logln!("{}: {dirs_chars:?}", i + 1);
    }
    let shortest = dirs_chars.iter().map(Vec::len).min().unwrap();

    shortest
}

pub fn get_costs(k: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![10000; 5]; 5];
    let mut dirs = Direction::cardinal_directions().to_vec();
    dirs.push(Direction::TopLeft);
    for a in &dirs {
        for b in &dirs {
            res[a.to_index()][b.to_index()] = run_calculation(&[*a, *b], k);
        }
    }

    res
}

pub fn get_single_cost(dirs: &[Direction], costs: &[Vec<usize>]) -> usize {
    let mut sum = 0;
    sum += costs[4][dirs[0].to_index()];
    logln!("");
    logln!("{sum} (TopLeft {:?})", dirs[0]);
    for (a, b) in dirs.iter().zip(dirs.iter().skip(1)) {
        logln!("+ {} ({a:?} {b:?})", costs[a.to_index()][b.to_index()]);
        sum += costs[a.to_index()][b.to_index()];
    }
    sum
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut sum = 0;
    let costs = get_costs(3);
    //logln!("{costs:?}");
    for line in reader.lines().map_while(Result::ok) {
        //logln!("{line}");
        let dirs_1 = get_fastest_route_to_code(&line);
        //logln!("{dirs_1:?}");

        let num_val = line.replace('A', "").parse::<usize>().unwrap();
        let shortest = dirs_1
            .iter()
            .map(|x| get_single_cost(x, &costs))
            .min()
            .unwrap();
        logln!("{} * {} = {}", shortest, num_val, shortest * num_val);
        sum += shortest * num_val;
    }
    Ok(sum)
}

/* SOLUTION 2 */

// Some inspiration might be taken from: https://www.reddit.com/r/adventofcode/comments/1hj8380/2024_day_21_part_2_i_need_help_three_days_in_row/
// however, i did not understand the how, so that is custom

pub fn calc_short(vec: &[Direction]) -> usize {
    let mut s = vec.len() * 100;
    for (a, b) in vec.iter().zip(vec.iter().skip(1)) {
        if a != b {
            s += 1;
        }
    }

    s
}

pub fn run_calculation_2(
    dirs_orig: &[Direction],
    k: usize,
    from_a: bool,
) -> (usize, Vec<Direction>) {
    let mut dirs = vec![dirs_orig.to_vec()];

    //logln!("0: {dirs_chars:?}");
    for i in 0..k {
        let dirs_chars = dirs
            .iter()
            .map(|x| tos_to_chars(x))
            .collect::<Vec<Vec<char>>>();
        let dirs_2 = dirs_chars
            .iter()
            .flat_map(|x| get_fastest_route_to_path(x, from_a || i != 0))
            .collect::<Vec<Vec<Direction>>>();
        dirs = dirs_2;

        //logln!("{}: {dirs_chars:?}", i + 1);
    }
    /*if dirs_orig[0] == To::TopLeft && dirs_orig[1] == To::Top {
        logln!("dirs: {:?}", dirs);
        let vals = dirs
            .iter()
            .map(|x| (calc_short(x), x.clone()))
            .collect::<Vec<(usize, Vec<To>)>>();
        logln!("vals: {:?}", vals);
    }*/
    let mut shortest = dirs
        .iter()
        .map(|x| (calc_short(x), x.clone()))
        .min()
        .unwrap();

    shortest.0 /= 100;

    shortest
}

pub fn get_costs_2(k: usize, from_a: bool) -> Vec<Vec<(usize, Vec<Direction>)>> {
    let mut res = vec![vec![(10000, Vec::new()); 5]; 5];
    let mut dirs = Direction::cardinal_directions().to_vec();
    dirs.push(Direction::TopLeft);
    for a in &dirs {
        for b in &dirs {
            res[a.to_index()][b.to_index()] = run_calculation_2(&[*a, *b], k, from_a);
        }
    }

    res
}

pub fn calculate_num(
    k: usize,
    vecs: &[Vec<(usize, Vec<Direction>)>],
    all_costs: &mut [Vec<Vec<Option<usize>>>],
    vec: &[Direction],
) -> usize {
    let mut s = 0;

    let level = "\t".repeat(k);
    logln!("{level} Working on {vec:?}");

    for (i, val) in vec.iter().enumerate() {
        if k == 1 {
            if i == 0 {
                s += vecs[4][val.to_index()].0;
            } else {
                s += vecs[vec[i - 1].to_index()][val.to_index()].0;
            }
        } else if i == 0 {
            logln!("{level} Working on first value {val:?}");
            if let Some(x) = all_costs[k][4][val.to_index()] {
                logln!("Cached: {}", x);
                s += x;
            } else {
                let res = calculate_num(k - 1, vecs, all_costs, &vecs[4][val.to_index()].1);
                all_costs[k][4][val.to_index()] = Some(res);
                s += res;
            }
        } else {
            logln!(
                "{level} Working on not first value {:?} -> {val:?}",
                vec[i - 1]
            );
            if let Some(x) = all_costs[k][vec[i - 1].to_index()][val.to_index()] {
                logln!("Cached: {}", x);
                s += x;
            } else {
                let res = calculate_num(
                    k - 1,
                    vecs,
                    all_costs,
                    &vecs[vec[i - 1].to_index()][val.to_index()].1,
                );
                all_costs[k][vec[i - 1].to_index()][val.to_index()] = Some(res);
                s += res;
            }
        }
    }
    logln!("{level} Returning: {s}");

    s
}

pub fn solution2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut sum = 0;

    //CALCULATION
    let mut costs = get_costs_2(1, false);
    costs[4][Direction::Bottom.to_index()] = (
        3,
        vec![Direction::Left, Direction::Bottom, Direction::TopLeft],
    );
    logln!("{costs:?}");
    let mut all_costs: Vec<Vec<Vec<Option<usize>>>> = vec![vec![vec![None; 5]; 5]; 26];

    for (i, v) in costs.iter().enumerate() {
        for (j, val) in v.iter().enumerate() {
            all_costs[1][i][j] = Some(val.0);
        }
    }

    for line in reader.lines().map_while(Result::ok) {
        logln!("{line}");
        let dirs_1 = get_fastest_route_to_code(&line);
        logln!("{dirs_1:?}");

        let dirs_transformed: Vec<Vec<Direction>> = dirs_1
            .iter()
            .map(|x| {
                let mut vec = costs[4][x[0].to_index()].1.clone();

                let mut vec2 = x
                    .windows(2)
                    .map(|x| costs[x[0].to_index()][x[1].to_index()].1.clone())
                    .flatten()
                    .collect::<Vec<Direction>>();
                vec.append(&mut vec2);

                vec
            })
            .collect();
        logln!("{dirs_transformed:?}");

        let shortest = dirs_transformed
            .iter()
            .map(|x| calculate_num(24, &costs, &mut all_costs, x))
            .min()
            .unwrap();

        let num_val = line.replace('A', "").parse::<usize>().unwrap();

        logln!("{} * {} = {}", shortest, num_val, shortest * num_val);

        sum += shortest * num_val;
    }

    Ok(sum)
}
