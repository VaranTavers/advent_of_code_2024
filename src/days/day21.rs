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
    if options.len() == 0 {
        //println!("Baj van!");
        return;
    }
    if k == options.len() {
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

    //println!("{movements:?}");

    let mut nums = movements.iter().map(|_x| 0).collect();
    backtrack(0, &mut res, &movements, &mut nums);

    res
}

pub fn get_fastest_route_to_path(code: &[char], add_a: bool) -> Vec<Vec<To>> {
    let mut res = Vec::new();
    let cmap: CharMap = WAY_PAD.into();
    //println!("{cmap}");

    let mut chars: Vec<char> = code.iter().cloned().collect();
    if add_a {
        chars.insert(0, 'A');
    }

    if code.len() == 1 {
        return vec![vec![To::TopLeft]];
    }

    let mut movements: Vec<Vec<Vec<To>>> = Vec::new();

    //println!("{chars:?}");
    for (a, b) in chars.iter().zip(chars.iter().skip(1)) {
        let start_pos = cmap.find_first(*a).unwrap();
        let end_pos = cmap.find_first(*b).unwrap();

        let dirs = dijkstra(&cmap, start_pos, end_pos);
        //println!("{dirs:?}");
        movements.push(dirs);
    }

    let mut nums = movements.iter().map(|_x| 0).collect();
    backtrack(0, &mut res, &movements, &mut nums);
    //println!("REEESS: {res:?}");
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

pub fn run_calculation(dirs: &[To], k: usize) -> usize {
    let mut dirs_chars = vec![tos_to_chars(dirs)];

    //println!("0: {dirs_chars:?}");
    for i in 0..k {
        let dirs_2 = dirs_chars
            .iter()
            .map(|x| {
                get_fastest_route_to_path(&x, i != 0)
                    .iter()
                    .map(|x| tos_to_chars(x))
                    .collect::<Vec<Vec<char>>>()
            })
            .flatten()
            .collect::<Vec<Vec<char>>>();
        dirs_chars = dirs_2;

        //println!("{}: {dirs_chars:?}", i + 1);
    }
    let shortest = dirs_chars.iter().map(|x| x.len()).min().unwrap();

    shortest
}

pub fn get_costs(k: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![10000; 5]; 5];
    let mut dirs = To::cardinal_directions().to_vec();
    dirs.push(To::TopLeft);
    for a in &dirs {
        for b in &dirs {
            res[a.to_index()][b.to_index()] = run_calculation(&[*a, *b], k);
        }
    }

    res
}

pub fn get_single_cost(dirs: &[To], costs: &[Vec<usize>]) -> usize {
    let mut sum = 0;
    sum += costs[4][dirs[0].to_index()];
    for (a, b) in dirs.iter().zip(dirs.iter().skip(1)) {
        sum += costs[a.to_index()][b.to_index()];
    }
    sum
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut sum = 0;
    let costs = get_costs(2);
    //println!("{costs:?}");
    for line in reader.lines().map_while(Result::ok) {
        //println!("{line}");
        let dirs_1 = get_fastest_route_to_code(&line);
        //println!("{dirs_1:?}");

        let num_val = line.replace("A", "").parse::<usize>().unwrap();
        let shortest = dirs_1
            .iter()
            .map(|x| get_single_cost(x, &costs))
            .min()
            .unwrap();
        //println!("{} * {} = {}", shortest, num_val, shortest * num_val);
        sum += shortest * num_val;
    }
    Ok(sum)
}

/* SOLUTION 2 */

pub fn calc_short(vec: &[To]) -> usize {
    let mut s = vec.len() * 100;
    for (a, b) in vec.iter().zip(vec.iter().skip(1)) {
        if a != b {
            s += 1;
        }
    }

    s
}

pub fn run_calculation_2(dirs_orig: &[To], k: usize, from_a: bool) -> (usize, Vec<To>) {
    let mut dirs = vec![dirs_orig.to_vec()];

    //println!("0: {dirs_chars:?}");
    for i in 0..k {
        let dirs_chars = dirs
            .iter()
            .map(|x| tos_to_chars(x))
            .collect::<Vec<Vec<char>>>();
        let dirs_2 = dirs_chars
            .iter()
            .map(|x| get_fastest_route_to_path(&x, from_a || i != 0))
            .flatten()
            .collect::<Vec<Vec<To>>>();
        dirs = dirs_2;

        //println!("{}: {dirs_chars:?}", i + 1);
    }
    /*if dirs_orig[0] == To::TopLeft && dirs_orig[1] == To::Top {
        println!("dirs: {:?}", dirs);
        let vals = dirs
            .iter()
            .map(|x| (calc_short(x), x.clone()))
            .collect::<Vec<(usize, Vec<To>)>>();
        println!("vals: {:?}", vals);
    }*/
    let mut shortest = dirs
        .iter()
        .map(|x| (calc_short(x), x.clone()))
        .min()
        .unwrap();

    shortest.0 /= 100;

    shortest
}

pub fn get_costs_2(k: usize, from_a: bool) -> Vec<Vec<(usize, Vec<To>)>> {
    let mut res = vec![vec![(10000, Vec::new()); 5]; 5];
    let mut dirs = To::cardinal_directions().to_vec();
    dirs.push(To::TopLeft);
    for a in &dirs {
        for b in &dirs {
            res[a.to_index()][b.to_index()] = run_calculation_2(&[*a, *b], k, from_a);
        }
    }

    res
}

pub fn calculate_num(k: usize, vecs: &[Vec<(usize, Vec<To>)>], vec: &[To]) -> usize {
    let mut s = 0;

    for (i, val) in vec.iter().enumerate() {
        if k == 1 {
            if i == 0 {
                s += vecs[4][val.to_index()].0;
            } else {
                s += vecs[vec[i - 1].to_index()][val.to_index()].0;
            }
        } else {
            if i == 0 {
                s += calculate_num(k - 1, vecs, &vecs[4][val.to_index()].1)
            } else {
                s += calculate_num(k - 1, vecs, &vecs[vec[i - 1].to_index()][val.to_index()].1)
            }
        }
    }

    s
}

pub fn solution2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut sum = 0;

    //CALCULATION
    let costs = get_costs_2(1, false);
    let costs_a = get_costs_2(1, false);

    let costs_b = costs_a
        .iter()
        .map(|x| x.iter().map(|y| y.0).collect())
        .collect::<Vec<Vec<usize>>>();

    println!("TWO:{:?}", costs[4]);
    println!();
    println!("A:  {:?}", costs_a);

    /*    //println!("{}", calculate_num(1, &costs_a, &costs[0][1].1));
        let mut res = vec![vec![10000; 5]; 5];
        for i in 0..5 {
            for j in 0..5 {
                res[i][j] = calculate_num(20, &costs_a, &costs[i][j].1);
                println!("{i} {j} {}", res[i][j]);
            }
        }
    */
    /*
    // RESULTS 25
    let costs: Vec<Vec<usize>> = vec![
        vec![1, 17411466119, 13075741541, 11948123406, 8135210660],
        vec![14541180821, 1, 8135210661, 8135210660, 14541180822],
        vec![15960515423, 12661998643, 1, 12661998642, 8437063956],
        vec![8437063956, 12661998642, 8135210660, 1, 13591086449],
        vec![12661998642, 17411466120, 11948123406, 15635719885, 1],
    ];

    // RESULTS 24
    let costs: Vec<Vec<usize>> = vec![
        vec![1, 6882163277, 5168398097, 4722688758, 3215573368],
        vec![5747636461, 1, 3215573369, 3215573368, 5747636462],
        vec![6308651143, 5004859249, 1, 5004859248, 3334885690],
        vec![3334885690, 5004859248, 3215573368, 1, 5372096411],
        vec![5004859248, 6882163278, 4722688758, 6180270419, 1],
    ];
    println!("{costs:?}");*/
    for line in reader.lines().map_while(Result::ok) {
        println!("{line}");
        let dirs_1 = get_fastest_route_to_code(&line);
        //println!("{dirs_1:?}");

        let num_val = line.replace("A", "").parse::<usize>().unwrap();
        let shortest = dirs_1
            .iter()
            .map(|x| calculate_num(1, &costs_a, x))
            .min()
            .unwrap();
        println!("{} * {} = {}", shortest, num_val, shortest * num_val);
        sum += shortest * num_val;
    }

    Ok(sum)
}
