use std::{
    fs::File,
    io::{BufRead, BufReader, Lines},
};

pub fn read_rules(lines: &mut Lines<BufReader<File>>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for line in lines {
        if let Ok(line) = line {
            if let Some((a, b)) = line.split_once('|') {
                res.push((a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()));
            } else {
                return res;
            }
        }
    }
    res
}

pub fn read_updates(lines: &mut Lines<BufReader<File>>) -> Vec<(Vec<usize>, [usize; 100])> {
    let mut res = Vec::new();

    for line in lines {
        if let Ok(line) = line {
            let vals = line
                .split(',')
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let mut indices = [1000; 100];

            for (i, val) in vals.iter().enumerate() {
                indices[*val] = i;
            }
            res.push((vals, indices));
        }
    }

    res
}

pub fn is_rule_valid_on_update((a, b): &(usize, usize), vals: &[usize]) -> bool {
    if vals[*a] == 1000 || vals[*b] == 1000 {
        return true;
    }

    vals[*a] < vals[*b]
}

pub fn validate_all_rules(rules: &[(usize, usize)], vals: &[usize]) -> bool {
    rules.iter().all(|rule| is_rule_valid_on_update(rule, vals))
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let mut lines = reader.lines();
    let rules = read_rules(&mut lines);
    let updates = read_updates(&mut lines);

    let mut sum = 0;

    for (update, vals) in updates {
        if validate_all_rules(&rules, &vals) {
            sum += update[update.len() / 2];
        }
    }

    Ok(sum as i64)
}
/* SOLUTION 2 */
pub fn sort_by_rules(rules: &[(usize, usize)], update: &[usize]) -> Vec<usize> {
    let mut vec = update.to_vec();
    let mut changed = true;
    while changed {
        //println!("{vec:?}");
        changed = false;
        for i in 0..vec.len() {
            for j in (i + 1)..vec.len() {
                let rule = rules
                    .iter()
                    .filter(|(a, b)| (*a == vec[j] && *b == vec[i]))
                    .last();
                if let Some(_) = rule {
                    vec.swap(i, j);
                    changed = true;
                }
            }
        }
    }
    //println!("{vec:?}");
    vec
}

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let mut lines = reader.lines();
    let rules = read_rules(&mut lines);
    let updates = read_updates(&mut lines);

    let mut sum = 0;

    for (update, vals) in updates {
        if !validate_all_rules(&rules, &vals) {
            let res = sort_by_rules(&rules, &update);
            sum += res[res.len() / 2];
        }
    }

    Ok(sum as i64)
}
