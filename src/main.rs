use std::fs;
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 && args.get(1).expect("Command line arg").parse::<i32>().is_ok() {
        let day = args.get(1).expect("Command line arg").parse::<i32>().unwrap();
        match day {
            1 => {
                println!("{}", day1::solve_p1());
                println!("{}", day1::solve_p2());
            },
            2 => {
                let contents = fs::read_to_string("day2_input.txt")
                    .expect("Should have the input to solve the problem");
                println!("{}", day2::solve_p1(contents.clone()));
                println!("{}", day2::solve_p2(contents));
            }, 
            3 => {
                let contents = fs::read_to_string("day3_input.txt")
                    .expect("Should have the input to solve the problem");
                println!("{}", day3::solve_p1(contents.clone()));
                println!("{}", day3::solve_p2(contents));
            },
            4 => {
                let contents = fs::read_to_string("day4_input.txt")
                    .expect("Should have the input to solve the problem");
                println!("{}", day4::solve_p1(contents.clone()));
                println!("{}", day4::solve_p2(contents));
            },
            5 => {
                let contents = fs::read_to_string("day5_input.txt")
                    .expect("Should have the input to solve the problem");
                println!("{}", day5::solve_p1(contents.clone()));
                println!("{}", day5::solve_p2(contents));
            },
            6 => {
                let contents = fs::read_to_string("day6_input.txt")
                    .expect("Should have the input to solve the problem");
                println!("{}", day6::solve_p1(contents.clone()));
                println!("{}", day6::solve_p2(contents));
            },
            7 => {
                let contents = fs::read_to_string("day7_input.txt")
                    .expect("Should have the input to solve the problem");
                println!("{}", day7::solve_p1(contents.clone()));
                println!("{}", day7::solve_p2(contents));
            },
            8 => {
                let contents = fs::read_to_string("day8_input.txt")
                    .expect("Should have the input to solve the problem");
                println!("{}", day8::solve_p1(contents.clone()));
                println!("{}", day8::solve_p2(contents));
            },
            9 => {
                let contents = fs::read_to_string("day9_input.txt")
                    .expect("Should have the input to solve the problem");
                println!("{}", day9::solve_p1(contents.clone()));
                println!("{}", day9::solve_p2(contents));
            },
            10 => {
                let contents = fs::read_to_string("day10_input.txt")
                    .expect("Should have the input to solve the problem");
                println!("{}", day10::solve_p1(contents.clone()));
                println!("{}", day10::solve_p2(contents));
            },
            _ => {
                panic!("Solution for specified day does not exist");
            }
        }
    } else if args.len() == 1 {
        // the day we're working on now
        println!("Day 11!");
        let contents = fs::read_to_string("day11_input.txt")
            .expect("Should have the input to solve the problem");
        println!("{}", solve_p1(contents.clone()));
        println!("{}", solve_p2(contents));

    } else {
        println!("arg should specify the day");
    }
}

pub fn solve_p1(_contents: String) -> i64 {
    0
}

#[test]
fn test_case_1() {
    let input = vec![
        "",
    ];
    let result = solve_p1(input.join("\n"));
    println!("{result}");
    assert!(result == 0);
}

pub fn solve_p2(_contents: String) -> i64 {
    0
}

#[test]
fn test_case_2() {
    let input = vec![
        "",
    ];
    let result = solve_p2(input.join("\n"));
    println!("{result}");
    assert!(result == 0);
}
