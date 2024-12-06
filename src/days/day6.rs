use std::{fs::File, io::BufReader};

use crate::utils::{CharMap, To};

pub fn get_next_position_and_dir(
    cmap: &CharMap,
    position: (usize, usize),
    direction: To,
) -> Option<((usize, usize), To)> {
    let mut direction = direction;
    let next_pos = direction.move_to(position);
    if next_pos.is_none() || !cmap.is_valid_coords(next_pos.unwrap()) {
        return None;
    }
    let mut next_pos = next_pos.unwrap();
    // It can turn at max 2 times in the same place
    if cmap.get(next_pos) == Some('#') {
        direction = direction.turn_right_90();
        let next_t_pos = direction.move_to(position);
        if next_t_pos.is_none() || !cmap.is_valid_coords(next_t_pos.unwrap()) {
            return None;
        }
        next_pos = next_t_pos.unwrap();
        if cmap.get(next_pos) == Some('#') {
            direction = direction.turn_right_90();
            let next_t_pos = direction.move_to(position);
            if next_t_pos.is_none() || !cmap.is_valid_coords(next_t_pos.unwrap()) {
                return None;
            }
            next_pos = next_t_pos.unwrap();
        }
    }

    Some((next_pos, direction))
}

pub fn generate_visited_dir(cmap: &CharMap) -> Vec<Vec<Option<To>>> {
    let mut visited_dir: Vec<Vec<Option<To>>> = cmap.map_to_val(None);

    let mut position = cmap.find_first('^').expect("No guard found");
    let mut direction = To::Top;

    while visited_dir[position.0][position.1].is_none()
        || visited_dir[position.0][position.1].unwrap() != direction
    {
        visited_dir[position.0][position.1] = Some(direction);

        let next_opt = get_next_position_and_dir(cmap, position, direction);
        if next_opt.is_none() {
            break;
        }
        (position, direction) = next_opt.unwrap();
    }

    visited_dir
}

pub fn solution(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let visited_dir = generate_visited_dir(&cmap);

    Ok(visited_dir
        .iter()
        .map(|x| x.iter().filter(|y| y.is_some()).count())
        .sum::<usize>() as i64)
}

/* SOLUTION 2 */

pub fn get_next_position_and_dir_plus_block(
    cmap: &CharMap,
    position: (usize, usize),
    direction: To,
    blocker: (usize, usize),
) -> Option<((usize, usize), To)> {
    let mut direction = direction;
    let next_pos = direction.move_to(position);
    if next_pos.is_none() || !cmap.is_valid_coords(next_pos.unwrap()) {
        return None;
    }
    let mut next_pos = next_pos.unwrap();
    // It can turn at max 2 times in the same place
    if cmap.get(next_pos) == Some('#') || next_pos == blocker {
        direction = direction.turn_right_90();
        let next_t_pos = direction.move_to(position);
        if next_t_pos.is_none() || !cmap.is_valid_coords(next_t_pos.unwrap()) {
            return None;
        }
        next_pos = next_t_pos.unwrap();
        if cmap.get(next_pos) == Some('#') || next_pos == blocker {
            direction = direction.turn_right_90();
            let next_t_pos = direction.move_to(position);
            if next_t_pos.is_none() || !cmap.is_valid_coords(next_t_pos.unwrap()) {
                return None;
            }
            next_pos = next_t_pos.unwrap();
        }
    }

    Some((next_pos, direction))
}

pub fn does_guard_leave(cmap: &CharMap, blocker: (usize, usize)) -> bool {
    let mut visited_dir: Vec<Vec<[bool; 4]>> = cmap.map_to_val([false; 4]);

    let mut position = cmap.find_first('^').expect("No guard found");
    let mut direction = To::Top;

    while !visited_dir[position.0][position.1][direction.to_number() - 1] {
        visited_dir[position.0][position.1][direction.to_number() - 1] = true;

        let next_opt = get_next_position_and_dir_plus_block(cmap, position, direction, blocker);
        if next_opt.is_none() {
            return true;
        }
        (position, direction) = next_opt.unwrap();
    }

    false
}

pub fn solution2(reader: BufReader<File>) -> Result<i64, std::io::Error> {
    let cmap = CharMap::parse_map(reader);

    let mut visited_dir: Vec<Vec<Option<To>>> = cmap.map_to_val(None);

    let mut position = cmap.find_first('^').expect("No guard found");
    let mut direction = To::Top;
    let mut sum = 0;
    /* Move one up to skip the start point */
    let next_opt: Option<((usize, usize), To)> =
        get_next_position_and_dir(&cmap, position, direction);
    (position, direction) = next_opt.unwrap();

    while visited_dir[position.0][position.1].is_none()
        || visited_dir[position.0][position.1].unwrap() != direction
    {
        if visited_dir[position.0][position.1].is_none() {
            if !does_guard_leave(&cmap, position) {
                sum += 1;
            }
        }
        visited_dir[position.0][position.1] = Some(direction);

        let next_opt = get_next_position_and_dir(&cmap, position, direction);
        if next_opt.is_none() {
            break;
        }
        (position, direction) = next_opt.unwrap();
    }

    Ok(sum)
}
