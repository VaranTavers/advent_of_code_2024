use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, Lines},
    str::FromStr,
};

const LOG: bool = true;

macro_rules! log {
    ($($x:tt)*) => { if LOG { print!($($x)*) } }
}

macro_rules! logln {
    ($($x:tt)*) => { if LOG { println!($($x)*) } }
}

pub fn calculate_result(hmap: &HashMap<String, Option<bool>>, c: char) -> usize {
    let mut zs: Vec<&String> = hmap.keys().filter(|x| x.starts_with(c)).collect();

    zs.sort();
    logln!("{zs:?}");
    let mut p = 1;
    let mut res = 0;
    for key in zs.iter() {
        if hmap[*key].unwrap() {
            log!("{p} +");
            res += p;
        }
        p *= 2;
    }

    logln!("");

    res
}

pub fn read_one_initial(line: &str) -> (String, bool) {
    let parts = line.split_once(": ").unwrap();

    (parts.0.to_owned(), parts.1 == "1")
}

pub fn read_starts_into_initial_hmap(
    lines: &mut Lines<BufReader<File>>,
) -> HashMap<String, Option<bool>> {
    let mut res = HashMap::new();

    while let Some(line_r) = lines.next() {
        if let Ok(line) = line_r {
            if line.is_empty() {
                break;
            }
            let (k, v) = read_one_initial(&line);
            res.insert(k, Some(v));
        }
    }

    res
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Xor,
    Or,
    And,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

impl Operation {
    pub fn apply(&self, a1: bool, a2: bool) -> bool {
        match self {
            Operation::Xor => a1 ^ a2,
            Operation::Or => a1 || a2,
            Operation::And => a1 && a2,
        }
    }
}

pub fn read_gate(line: &str) -> ((String, String), (Operation, String)) {
    let eq_sides = line.split_once(" -> ").expect("Bad line");

    let output = eq_sides.1.to_owned();

    let l_parts = eq_sides.0.split(" ").collect::<Vec<&str>>();

    (
        (l_parts[0].to_owned(), l_parts[2].to_owned()),
        (l_parts[1].parse().unwrap(), output),
    )
}

pub fn read_connections(
    mut lines: Lines<BufReader<File>>,
) -> (
    Vec<String>,
    HashMap<(String, String), Vec<(Operation, String)>>,
) {
    let mut res_set = Vec::new();
    let mut res_gates = HashMap::new();
    while let Some(line_r) = lines.next() {
        if let Ok(line) = line_r {
            if line.is_empty() {
                break;
            }
            let ((in1, in2), (op, out)) = read_gate(&line);
            res_gates
                .entry((in1.clone(), in2.clone()))
                .or_insert(Vec::new())
                .push((op, out.clone()));
            res_set.push(out);
        }
    }

    (res_set, res_gates)
}

pub fn solution(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    let mut lines = reader.lines();

    let mut hmap = read_starts_into_initial_hmap(&mut lines);
    let (_other_wires, gates) = read_connections(lines);

    let mut known: Vec<String> = hmap.keys().cloned().collect();

    let mut i = 1;
    while i < known.len() {
        logln!("{}", known[i]);
        for j in 0..=i {
            logln!("\t{}", known[j]);
            let k1 = known[i].clone();
            let k2 = known[j].clone();
            if let Some(list) = gates.get(&(k1, k2)) {
                for (op, val) in list {
                    logln!("{} {:?} {} -> {}", known[i], op, known[j], val);
                    hmap.insert(
                        val.clone(),
                        Some(op.apply(hmap[&known[i]].unwrap(), hmap[&known[j]].unwrap())),
                    );
                    known.push(val.clone());
                }
            }
            let k1 = known[i].clone();
            let k2 = known[j].clone();
            if let Some(list) = gates.get(&(k2, k1)) {
                for (op, val) in list {
                    logln!("{} {:?} {} -> {}", known[i], op, known[j], val);
                    hmap.insert(
                        val.clone(),
                        Some(op.apply(hmap[&known[i]].unwrap(), hmap[&known[j]].unwrap())),
                    );
                    known.push(val.clone());
                }
            }
        }
        i += 1;
    }

    println!("{}", calculate_result(&hmap, 'x'));
    println!("{}", calculate_result(&hmap, 'y'));
    Ok(calculate_result(&hmap, 'z'))
}

/* SOLUTION 2 */

pub struct ZeroPadded(pub usize);

impl Display for ZeroPadded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 < 10 {
            write!(f, "0")?;
        }
        write!(f, "{}", self.0)
    }
}

pub fn log_adder_x_bytes(x: usize) {
    logln!("");
    for i in 0..x {
        logln!("x{}: 0", ZeroPadded(i));
    }
    for i in 0..x {
        logln!("y{}: 0", ZeroPadded(i));
    }
    logln!();
    logln!("x00 AND y00 -> ca00");
    logln!("x00 XOR y00 -> z00");

    for i in 0..(x - 1) {
        let j = ZeroPadded(i + 1);
        let i = ZeroPadded(i);
        logln!("x{j} AND y{j} -> pc{j}");
        logln!("x{j} XOR y{j} -> re{j}");
        logln!("re{j} AND ca{i} -> cc{j}");
        logln!("re{j} XOR ca{i} -> z{j}");
        if i.0 != x - 2 {
            logln!("pc{j} OR cc{j} -> ca{j}");
        } else {
            logln!("pc{j} OR cc{j} -> z{}", ZeroPadded(j.0 + 1));
        }
    }
}

pub fn insert_if_possible(names: &mut HashMap<String, String>, key: &str, val: String) {
    if !names.contains_key(key) {
        names.insert(key.to_owned(), val);
    } else {
        let orig_vals = names.get(key).unwrap();
        if *orig_vals != val {
            println!(
                "Name collision: {key} should be {val} but is {} instead",
                names[key]
            )
        }
    }
}

pub fn insert_if_possible_pair(
    names: &mut HashMap<String, (String, String)>,
    key: &str,
    val1: String,
    val2: String,
) {
    if !names.contains_key(key) {
        names.insert(key.to_owned(), (val1, val2));
    } else {
        let orig_vals = names.get(key).unwrap();
        if *orig_vals.0 != *val1 || *orig_vals.1 != *val2 {
            println!(
                "Name collision: {key} should be {val1} or {val2} but is {:?} instead",
                names[key]
            )
        }
    }
}

pub fn solution2(reader: BufReader<File>) -> Result<usize, std::io::Error> {
    //log_adder_x_bytes(45);
    let mut lines = reader.lines();

    let _hmap = read_starts_into_initial_hmap(&mut lines);
    let (other_wires, gates) = read_connections(lines);

    let mut names: HashMap<String, String> = HashMap::new();

    // Identifiing "partial carry"-s and "remainders"
    for ((in1, in2), list) in gates.iter() {
        if (in1.starts_with('x') && in2.starts_with('y'))
            || (in2.starts_with('x') && in1.starts_with('y'))
        {
            let num = in1.split_at(1).1.parse::<usize>().unwrap();
            for (op, out) in list {
                match op {
                    Operation::And => {
                        if num != 0 {
                            insert_if_possible(&mut names, out, format!("pc{}", ZeroPadded(num)));
                        } else {
                            insert_if_possible(&mut names, out, format!("ca{}", ZeroPadded(num)));
                        }
                    }
                    Operation::Xor => {
                        if num != 0 {
                            insert_if_possible(&mut names, out, format!("re{}", ZeroPadded(num)));
                        } else {
                            insert_if_possible(&mut names, out, format!("z{}", ZeroPadded(num)));
                        }
                    }
                    _ => println!("Something is wrong"),
                }
            }
        }
    }
    // Identifiing cc and ca-s
    for ((in1, in2), list) in gates.iter() {
        if names.contains_key(in1) {
            let new_name = &names[in1];
            if new_name.starts_with("pc") {
                let num = new_name.split_at(2).1.parse::<usize>().unwrap();
                for (op, out) in list {
                    if *op == Operation::Or {
                        insert_if_possible(&mut names, in2, format!("cc{}", ZeroPadded(num)));
                        insert_if_possible(&mut names, out, format!("ca{}", ZeroPadded(num)));
                    } else {
                        println!("{in1}: pc can only be part of an OR operation, however, it is a part of: {in1} {op:?} {in2} -> {out}");
                    }
                }
            }
        }
        if names.contains_key(in2) {
            let new_name = &names[in2];
            if new_name.starts_with("pc") {
                let num = new_name.split_at(2).1.parse::<usize>().unwrap();
                for (op, out) in list {
                    if *op == Operation::Or {
                        insert_if_possible(&mut names, in1, format!("cc{}", ZeroPadded(num)));
                        insert_if_possible(&mut names, out, format!("ca{}", ZeroPadded(num)));
                    } else {
                        println!("{in2}: pc can only be part of an OR operation, however, it is a part of: {in1} {op:?} {in2} -> {out}");
                    }
                }
            }
        }
    }
    let mut possible_names: HashMap<String, (String, String)> = HashMap::new();
    for ((in1, in2), list) in gates.iter() {
        for (op, out) in list {
            if out.starts_with('z') {
                let num = out.split_at(1).1.parse::<usize>().unwrap();
                if num != 0 {
                    if *op == Operation::Xor {
                        insert_if_possible_pair(
                            &mut possible_names,
                            in1,
                            format!("re{}", ZeroPadded(num)),
                            format!("ca{}", ZeroPadded(num - 1)),
                        );
                        insert_if_possible_pair(
                            &mut possible_names,
                            in2,
                            format!("re{}", ZeroPadded(num)),
                            format!("ca{}", ZeroPadded(num - 1)),
                        );
                    } else {
                        if num != 45 {
                            println!("We can only get a z by XOR-ing, however, we go one from: {in1} {op:?} {in2} -> {out}");
                        }
                    }
                }
            }
        }
    }
    // Find cc-s working backwards
    for ((in1, in2), list) in gates.iter() {
        for (op, out) in list {
            if let Some(new_name) = names.get(out) {
                if new_name.starts_with("cc") {
                    let num = new_name.split_at(2).1.parse::<usize>().unwrap();
                    if num != 0 {
                        if *op == Operation::And {
                            insert_if_possible_pair(
                                &mut possible_names,
                                in1,
                                format!("re{}", ZeroPadded(num)),
                                format!("ca{}", ZeroPadded(num - 1)),
                            );
                            insert_if_possible_pair(
                                &mut possible_names,
                                in2,
                                format!("re{}", ZeroPadded(num)),
                                format!("ca{}", ZeroPadded(num - 1)),
                            );
                        } else {
                            println!("We can only get a cc by AND-ing, however, we go one from: {in1} {op:?} {in2} -> {out}");
                        }
                    }
                }
            }
        }
    }

    for (k, v) in names.iter() {
        if possible_names.contains_key(k) {
            let possibilities = &possible_names[k];
            if possibilities.0 != *v && possibilities.1 != *v {
                println!(
                    "{k} has incompatible names: should be {v} but is one of {possibilities:?}"
                );
            }
        }
    }
    for v in other_wires {
        if !names.contains_key(&v) && !possible_names.contains_key(&v) && !v.starts_with('z') {
            println!("Unknown variable: {v}");
        }
    }
    for (k1, name1) in &names {
        for (k2, name2) in &names {
            if k1 != k2 && name1 == name2 {
                println!("Duplicate values: {k1} != {k2} but their values are equal {name1}");
            }
        }
    }
    println!("{:?}", names);
    println!("{:?}", possible_names);

    Ok(0)
}
