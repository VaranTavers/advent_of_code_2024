use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_all_computers(vec_start: &[String], vec_end: &[String]) -> Vec<String> {
    let set_start: HashSet<String> = HashSet::from_iter(vec_start.iter().cloned());
    let set_end: HashSet<String> = HashSet::from_iter(vec_end.iter().cloned());

    let set_comb: HashSet<&String> = set_start.union(&set_end).collect();

    set_comb.iter().cloned().cloned().collect::<Vec<String>>()
}

pub fn solution(reader: BufReader<File>) -> Result<u64, std::io::Error> {
    let lines = reader.lines().map_while(Result::ok);
    let mut vec_start = Vec::new();
    let mut vec_end = Vec::new();

    for line in lines {
        let parts = line.split_once('-').expect("Bad line");
        vec_start.push(parts.0.to_owned());
        vec_end.push(parts.1.to_owned());
    }

    let all_computers = get_all_computers(&vec_start, &vec_end);

    let mut inc_mat = (0..all_computers.len())
        .map(|_| vec![false; all_computers.len()])
        .collect::<Vec<Vec<bool>>>();

    for (a, b) in vec_start.iter().zip(vec_end.iter()) {
        let a_i = all_computers.iter().position(|x| a == x).unwrap();
        let b_i = all_computers.iter().position(|x| b == x).unwrap();

        inc_mat[a_i][b_i] = true;
        inc_mat[b_i][a_i] = true;
    }

    let mut sum = 0;

    for i in 0..all_computers.len() {
        for j in i + 1..all_computers.len() {
            if inc_mat[i][j] {
                for k in j + 1..all_computers.len() {
                    if inc_mat[j][k]
                        && inc_mat[k][i]
                        && (all_computers[i].starts_with('t')
                            || all_computers[j].starts_with('t')
                            || all_computers[k].starts_with('t'))
                    {
                        sum += 1;
                    }
                }
            }
        }
    }
    Ok(sum)
}

/* SOLUTION 2 */
pub fn is_clique(list: &[usize], inc_mat: &[Vec<bool>]) -> bool {
    for i in list.iter() {
        for j in list.iter() {
            if i != j && !inc_mat[*i][*j] {
                return false;
            }
        }
    }

    true
}

pub fn get_list_from_vals_and_list(vals: &[bool], list: &[usize]) -> Vec<usize> {
    list.iter()
        .zip(vals)
        .filter(|(_x, a)| **a)
        .map(|(x, _a)| *x)
        .collect::<Vec<usize>>()
}

pub fn is_ok(vals: &[bool], list: &[usize], inc_mat: &[Vec<bool>]) -> bool {
    let list2 = get_list_from_vals_and_list(vals, list);

    is_clique(&list2, inc_mat)
}

pub fn backtrack_largest_clique(
    k: usize,
    list: &[usize],
    vals: &mut [bool],
    vals_res: &mut Option<Vec<usize>>,
    inc_mat: &[Vec<bool>],
) {
    if k == vals.len() {
        //println!("{:?} {:?}", vals, list);
        if is_ok(vals, list, inc_mat) {
            let next_res = get_list_from_vals_and_list(vals, list);
            let vals_res_taken = vals_res.take();
            if vals_res_taken.is_none() || next_res.len() > vals_res_taken.as_ref().unwrap().len() {
                vals_res.replace(next_res.clone());
            } else {
                vals_res.replace(vals_res_taken.unwrap());
            }
        }
        return;
    }

    vals[k] = true;
    backtrack_largest_clique(k + 1, list, vals, vals_res, inc_mat);
    vals[k] = false;
    backtrack_largest_clique(k + 1, list, vals, vals_res, inc_mat);
}
pub fn solution2(reader: BufReader<File>) -> Result<String, std::io::Error> {
    let lines = reader.lines().map_while(Result::ok);
    let mut vec_start = Vec::new();
    let mut vec_end = Vec::new();

    for line in lines {
        let parts = line.split_once('-').expect("Bad line");
        vec_start.push(parts.0.to_owned());
        vec_end.push(parts.1.to_owned());
    }

    let all_computers = get_all_computers(&vec_start, &vec_end);

    let mut inc_mat = (0..all_computers.len())
        .map(|_| vec![false; all_computers.len()])
        .collect::<Vec<Vec<bool>>>();

    for (a, b) in vec_start.iter().zip(vec_end.iter()) {
        let a_i = all_computers.iter().position(|x| a == x).unwrap();
        let b_i = all_computers.iter().position(|x| b == x).unwrap();

        inc_mat[a_i][b_i] = true;
        inc_mat[b_i][a_i] = true;
    }

    let mut connections = inc_mat
        .iter()
        .enumerate()
        .map(|(i, x)| {
            (
                x.iter().filter(|x| **x).count(),
                x.iter()
                    .enumerate()
                    .filter(|(_i, x)| **x)
                    .map(|(i, _)| i)
                    .collect(),
                i,
            )
        })
        .collect::<Vec<(usize, Vec<usize>, usize)>>();

    connections.sort();

    let mut max_inds = Vec::new();
    let mut max_len = 0;
    for (val, list, i) in connections.iter().rev() {
        if max_len > *val {
            break;
        }

        let mut vals_res = None;
        let mut list: Vec<usize> = list.iter().cloned().collect();
        list.push(*i);

        let mut vals: Vec<bool> = list.iter().map(|_| false).collect();
        backtrack_largest_clique(0, &list, &mut vals, &mut vals_res, &inc_mat);
        if let Some(vals_res) = vals_res {
            if vals_res.len() > max_len {
                max_len = vals_res.len();
                max_inds = vals_res.clone();
            }
        }
    }

    let mut names: Vec<String> = max_inds.iter().map(|x| all_computers[*x].clone()).collect();
    names.sort();

    Ok(names.join(","))
}
