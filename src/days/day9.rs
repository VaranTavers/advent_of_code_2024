use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

pub fn get_num_char(reader: BufReader<File>) -> Vec<u8> {
    reader
        .lines()
        .next()
        .expect("No lines")
        .expect("Line read error")
        .chars()
        .map(|x| (x as u8 - b'0'))
        .collect()
}

pub fn get_largest_id(nums: &[u8]) -> u64 {
    ((nums.len() - 1) / 2) as u64
}

pub fn naive_solution(nums: &[u8]) -> u64 {
    let mut v = VecDeque::new();
    let mut id = 0;
    for (i, n) in nums.iter().enumerate() {
        if i % 2 == 1 {
            for _ in 0..*n {
                v.push_back(None);
            }
        } else {
            for _ in 0..*n {
                v.push_back(Some(id));
            }
            id += 1;
        }
    }
    let mut sum = 0;
    let mut pos = 0;

    println!("{v:?}");
    //println!("{}", v.len());
    while !v.is_empty() {
        //println!("{}", v.len());
        if v[0].is_some() {
            println!("{} * {pos}", v[0].unwrap());
            sum += v.pop_front().unwrap().unwrap() * pos;
        } else {
            while !v.is_empty() && v[v.len() - 1].is_none() {
                v.pop_back();
            }
            if v.is_empty() {
                break;
            }
            println!("{} * {pos}", v[v.len() - 1].unwrap());
            sum += v.pop_back().unwrap().unwrap() * pos;
            v.pop_front();
        }

        pos += 1;
    }

    sum
}

pub fn smart_solution(nums: &[u8]) -> u64 {
    let mut nums = nums.to_vec();
    let mut begin = 0;
    let mut pos = 0;
    let mut end = nums.len() - 1;
    if end % 2 == 1 {
        end -= 1;
    }
    let mut start_id = 0;
    let mut end_id = get_largest_id(&nums);
    let mut sum: u64 = 0;

    while begin <= end {
        //println!("{begin} {end}");
        if begin % 2 == 1 {
            if begin != end {
                for _ in 0..nums[begin].min(nums[end]) {
                    print!("({end_id}) * {pos} + ");
                    sum += pos * end_id;
                    pos += 1;
                    //println!("{k}: {} : {}", nums[end], nums[begin]);
                    nums[end] -= 1;
                    nums[begin] -= 1;
                }
                if nums[end] == 0 {
                    print!("||");
                    end_id -= 1;
                    end -= 2;
                }
                if nums[begin] == 0 {
                    begin += 1;
                    println!();
                }
            } else {
                println!("{begin} {end}EEEE");
                begin += 1;
            }
        } else {
            for _ in 0..nums[begin] {
                print!("({start_id}) * {pos} + ");
                sum += pos * start_id;
                pos += 1;
                nums[begin] -= 1;
            }
            println!();
            start_id += 1;
            begin += 1;
        }
    }

    sum
}

pub fn solution(reader: BufReader<File>) -> Result<u64, std::io::Error> {
    let nums = get_num_char(reader);

    Ok(smart_solution(&nums))
}

/* SOLUTION 2 */

fn find_space_at_least_size_lt_j(
    v: &VecDeque<Option<u64>>,
    size: usize,
    j: usize,
) -> Option<usize> {
    let mut k = 0;
    //println!("AAA");
    loop {
        //println!("{k}, {j}, {size}");
        while k < j && v[k].is_some() {
            k += 1;
        }
        if k >= j {
            return None;
        }
        let mut i = 0;
        while v[k + i].is_none() {
            i += 1;
        }
        if i >= size {
            return Some(k);
        }

        k += i;
    }
}

pub fn naive_solution_2(nums: &[u8]) -> u64 {
    let mut v = VecDeque::new();
    let mut id = 0;
    for (i, n) in nums.iter().enumerate() {
        if i % 2 == 1 {
            for _ in 0..*n {
                v.push_back(None);
            }
        } else {
            for _ in 0..*n {
                v.push_back(Some(id));
            }
            id += 1;
        }
    }

    let mut i = v.len() - 1;

    while i > 0 {
        // Move to next not free space
        while i > 0 && v[i].is_none() {
            i -= 1;
        }
        if i == 0 {
            break;
        }
        // Calculate size
        let mut j = i;
        while j > 0 && v[j].is_some() && v[j].unwrap() == v[i].unwrap() {
            j -= 1;
        }
        j += 1;

        // Calculate leftmost free space
        let place = find_space_at_least_size_lt_j(&v, i - j + 1, j);
        // Move
        if let Some(k) = place {
            for ii in 0..=(i - j) {
                v.swap(k + ii, j + ii);
            }
        }
        i = j - 1;
    }
    let mut sum = 0;

    //println!("{:?}", v);
    //println!("{}", v.len());
    for (pos, x) in v.iter().enumerate() {
        if let Some(val) = x {
            sum += pos as u64 * val;
        }
    }

    sum
}

pub fn solution2(reader: BufReader<File>) -> Result<u64, std::io::Error> {
    let nums = get_num_char(reader);

    Ok(naive_solution_2(&nums))
}
