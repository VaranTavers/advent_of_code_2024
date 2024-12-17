use std::{
    fs::File,
    io::{self, BufReader},
};

use crate::days::{
    day1, day10, day11, day12, day13, day14, day15, day16,
    day17, /*        day18, day19, day20, day21,
           day22, day23, day24, day25,       */
    day2, day3, day4, day5, day6, day7, day8, day9,
};
extern crate helper_lib;
mod days;

fn main() -> Result<(), io::Error> {
    let day = 17;
    let part = 2;

    let f = File::open(format!("inputs/input{day}.txt"))?;
    let reader = BufReader::new(f);

    print!("Day {day} (Part: {part}): ");
    match part {
        1 => {
            match day {
                1 => println!("{}", day1::solution(reader).unwrap()),
                2 => println!("{}", day2::solution(reader).unwrap()),
                3 => println!("{}", day3::solution(reader).unwrap()),
                4 => println!("{}", day4::solution(reader).unwrap()),
                5 => println!("{}", day5::solution(reader).unwrap()),
                6 => println!("{}", day6::solution(reader).unwrap()),
                7 => println!("{}", day7::solution(reader).unwrap()),
                8 => println!("{}", day8::solution(reader).unwrap()),
                9 => println!("{}", day9::solution(reader).unwrap()),
                10 => println!("{}", day10::solution(reader).unwrap()),
                11 => println!("{}", day11::solution(reader).unwrap()),
                12 => println!("{}", day12::solution(reader).unwrap()),
                13 => println!("{}", day13::solution(reader).unwrap()),
                14 => println!("{}", day14::solution(reader).unwrap()),
                15 => println!("{}", day15::solution(reader).unwrap()),
                16 => println!("{}", day16::solution(reader).unwrap()),
                17 => println!("{}", day17::solution(reader).unwrap()),
                /*18 => println!("{}", day18::solution(reader).unwrap()),
                19 => println!("{}", day19::solution(reader).unwrap()),
                20 => println!("{}", day20::solution(reader).unwrap()),
                21 => println!("{}", day21::solution(reader).unwrap()),
                22 => println!("{}", day22::solution(reader).unwrap()),
                23 => println!("{}", day23::solution(reader).unwrap()),
                24 => println!("{}", day24::solution(reader).unwrap()),
                25 => println!("{}", day25::solution(reader).unwrap()),*/
                _ => println!("What?"),
            };
        }
        _ => {
            match day {
                1 => println!("{}", day1::solution2(reader).unwrap()),
                2 => println!("{}", day2::solution2(reader).unwrap()),
                3 => println!("{}", day3::solution2(reader).unwrap()),
                4 => println!("{}", day4::solution2(reader).unwrap()),
                5 => println!("{}", day5::solution2(reader).unwrap()),
                6 => println!("{}", day6::solution2(reader).unwrap()),
                7 => println!("{}", day7::solution2(reader).unwrap()),
                8 => println!("{}", day8::solution2(reader).unwrap()),
                9 => println!("{}", day9::solution2(reader).unwrap()),
                10 => println!("{}", day10::solution2(reader).unwrap()),
                11 => println!("{}", day11::solution2(reader).unwrap()),
                12 => println!("{}", day12::solution2(reader).unwrap()),
                13 => println!("{}", day13::solution2(reader).unwrap()),
                14 => println!("{}", day14::solution2(reader).unwrap()),
                15 => println!("{}", day15::solution2(reader).unwrap()),
                16 => println!("{}", day16::solution2(reader).unwrap()),
                17 => println!("{}", day17::solution2(reader).unwrap()),
                /*18 => println!("{}", day18::solution_2(reader).unwrap()),
                19 => println!("{}", day19::solution_2(reader).unwrap()),
                20 => println!("{}", day20::solution_2(reader).unwrap()),
                21 => println!("{}", day21::solution_2(reader).unwrap()),
                22 => println!("{}", day22::solution_2(reader).unwrap()),
                23 => println!("{}", day23::solution_2(reader).unwrap()),*/
                _ => println!("What?"),
            };
        }
    };

    Ok(())
}
