use std::fs;

pub fn solve_p2() -> i32 {
    // grab all the lines from the file
    let contents = fs::read_to_string("day1_input.txt")
        .expect("Should have the input to solve the problem");
    let mut results = Vec::<i32>::new();
    for line in contents.lines() {
        results.push(solve_line_p2(line));
    }
    results.iter().sum::<i32>()
}

fn solve_line_p2(line: &str) -> i32 {
    // one, two, three, four, five, six, seven, eight, nine, zero
    // 1, 2, 3, 4, 5, 6, 7, 8, 9, 0
    // if numeric, parse the single character
    // if alpha, parse out a single word, if not

    let tokens = vec!["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
                      "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut firsts: [i32; 10] = [i32::MAX; 10];
    let mut lasts: [i32; 10] = [i32::MIN; 10];
    // let mut first: Option<i32> = None;
    // let mut last: Option<i32> = None;
    for (i, tok) in tokens.into_iter().enumerate() {
        // let mut first_found = false;
        let mut start_idx = 0;
        while let Some(idx) = line[start_idx..].find(tok) {
            if firsts[i%10] > start_idx as i32 + idx as i32 {
                firsts[i%10] = start_idx as i32 + idx as i32;
                // first_found = true;
            }
            if lasts[i%10] < start_idx as i32 + idx as i32 {
                lasts[i%10] = start_idx as i32 + idx as i32;
            }
            start_idx = start_idx + idx + tok.len();
            if start_idx > line.len() {
                break;
            }
            // println!("{line}: {tok} at {idx}, s_i: {start_idx}");
        }
    }
    // println!("{:?}", firsts);
    // println!("{:?}", lasts);
    let mut min_first = i32::MAX;
    let mut min_index: i32 = -1;
    for (i, e) in firsts.into_iter().enumerate() {
        // if i < min_first, min_first = e
        if e < min_first {
            min_index = i as i32;
            min_first = e;
        }
    }
    let mut max_last = i32::MIN;
    let mut max_index: i32 = -1;
    for (i, e) in lasts.into_iter().enumerate() {
        if e > max_last {
            max_index = i as i32;
            max_last = e;
        }
    }
    assert!(max_index != -1);
    assert!(min_index != -1);
    // println!("{line}: {}", format!("{}{}", min_index, max_index));
    format!("{}{}", min_index, max_index).parse::<i32>().unwrap()
}

pub fn solve_p1() -> i32 {
    // grab all the lines from the file
    let contents = fs::read_to_string("day1_input.txt")
        .expect("Should have the input to solve the problem");
    let mut results = Vec::<i32>::new();
    for line in contents.lines() {
        results.push(solve_line_p1(line));
    }
    results.iter().sum::<i32>()
}

fn solve_line_p1(line: &str) -> i32 {
    let mut first: Option<i32> = None;
    let mut last: Option<i32> = None;
    for c in line.chars() {
        if c.is_numeric() {
            match first {
                None => {
                    first = Some(String::from(c).parse::<i32>().unwrap());
                },
                Some(_) => {},
            }
            last = Some(String::from(c).parse::<i32>().unwrap());
        }
    }
    format!("{}{}", first.unwrap(), last.unwrap()).parse::<i32>().unwrap()
}

#[test]
fn test_case_1() {

    let lines: Vec::<&'static str> = vec!["1abc2",
        "pqr3stu8vwx",
        "a1b2c3d4e5f",
        "treb7uchet"];

    let mut results = Vec::<i32>::new();

    for line in lines {
        results.push(solve_line_p1(line));
    }

    assert!(results.iter().sum::<i32>() == 142);
}
#[test]
fn test_case_2() {

    let lines: Vec::<&'static str> = vec![
        "two1nine",
        "eightwothree",
        "eightwo",
        "abcone2threexyz",
        "xtwone3four",
        "4nineeightseven2",
        "zoneight234",
        "7pqrstsixteen",
        "425six14two46",
        "zpsevenfouronexftsphg731",
        "513five"
    ];

    let mut results = Vec::<i32>::new();

    for line in lines {
        results.push(solve_line_p2(line));
    }

    println!("{}", results.iter().sum::<i32>());

    assert!(results.iter().sum::<i32>() == (281 + 55 + 46 + 71 + 82));
}
