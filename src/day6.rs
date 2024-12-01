
use std::collections::HashMap;

fn parse_input(contents: String) -> HashMap<i64, i64> {
    let mut records = HashMap::<i64, i64>::new();
    let lines: Vec<&str> = contents.lines().collect();
    let times: Vec<i64> = lines[0]
        .split(" ")
        .filter(|c| { c.parse::<i64>().is_ok() })
        .map(|c| { c.parse::<i64>().unwrap() } )
        .collect();
    let distances: Vec<i64> = lines[1]
        .split(" ")
        .filter(|c| { c.parse::<i64>().is_ok() })
        .map(|c| { c.parse::<i64>().unwrap() } )
        .collect();
    assert!(times.len() == distances.len());
    for (i, time) in times.into_iter().enumerate() {
        records.insert(time, distances[i]);
    }
    records
}

fn parse_input_v2(contents: String) -> (i64, i64) {
    let lines: Vec<&str> = contents.lines().collect();
    let time: i64 = lines[0]
        .split(" ")
        .filter(|c| { c.parse::<i64>().is_ok() })
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>().unwrap();
    let distance: i64 = lines[1]
        .split(" ")
        .filter(|c| { c.parse::<i64>().is_ok() })
        .collect::<Vec<&str>>()
        .join("")
        .parse::<i64>().unwrap();
    (time, distance)
}

fn f(x: i64, time: i64, distance: i64) -> i64 {
    (-1 * x.pow(2)) + (time * x) - distance
}

fn fd(x: i64, time: i64, _distance: i64) -> i64 {
    (-2 * x) + time
}

pub fn solve_p1(contents: String) -> i64 {
    let records = parse_input(contents);
    // maximize the function
    // avg speed = distance / time
    // (time - acheived speed ) / time = distance
    // (acheived_speed * 0) + ((time - acheived_speed)*acheived_speed) / time = avg speed
    //
    // speed = distance / time
    // distance = (time - a_s)*a_s
    //
    //
    // distance = time*a_s - a_s^2
    // distance + a_s^2 = time*a_s
    // sqrt(distance) + a_s = time
    // a_s = time - sqrt(distance)
    let mut total: i64 = 1;
    for (time, distance) in &records {
        let time = *time;
        let distance = *distance;
        // println!("{:?} {:?}", time, distance);
        // minimum a_s = 
        // solve (time - a_s)*a_s - distance > 0
        // find the zeros
        // -a_s^2 + time*a_s - distance
        // newton-raphson??
        // f(x) = -x^2 + time*x - distance
        // f'(x) = -2*x + time
        // x = 0
        // x2 = max(time)

        let mut x: i64 = 0;
        let mut x2: i64 = time;
        while f(x, time, distance) < 1 {
            // x = x - (f(x, time, distance) / fd(x, time, distance));
            x += 1;
            // println!("{:?} {:?}", x, f(x, time, distance));
        
        }
        while f(x2, time, distance) < 1 {
            x2 -= 1;
            // println!("{:?} {:?}", x2, f(x2, time, distance));
        }
        total *= 1+ (x2-x);
        // println!("RANGE: {:?}..{:?}", x, x2);
    }
    // println!("total: {total:?}");
    total
}

pub fn solve_p2(contents: String) -> i64 {
    let (time, distance) = parse_input_v2(contents);
    // println!("{:?} {:?}", time, distance);
    // minimum a_s = 
    // solve (time - a_s)*a_s - distance > 0
    // find the zeros
    // -a_s^2 + time*a_s - distance
    // newton-raphson??
    // f(x) = -x^2 + time*x - distance
    // f'(x) = -2*x + time
    // x = 0
    // x2 = max(time)
    let mut x: i64 = 0;
    let mut x2: i64 = time;
    while f(x, time, distance) < 1 {
        x = x - (f(x, time, distance) / fd(x, time, distance));
        x += 1;
        // println!("{:?} {:?}", x, f(x, time, distance));
    }
    while f(x2, time, distance) < 1 {
        x2 -= 1;
        // println!("{:?} {:?}", x2, f(x2, time, distance));
    }
    (x2-x) + 1
}

#[test]
fn test_case_1() {
    let input = vec![
        "Time:      7  15   30",
        "Distance:  9  40  200"
    ];
    let result = solve_p1(input.join("\n"));
    assert!(result == 288);
}

#[test]
fn test_case_2() {
    let input = vec![
        "Time:      7  15   30",
        "Distance:  9  40  200"
    ];
    let (time, distance) = parse_input_v2(input.join("\n"));
    assert!(time == 71530);
    assert!(distance == 940200);
    let result = solve_p2(input.join("\n"));
    assert!(result == 71503);
}

#[test]
fn test_result_1() {
    let input  = vec![
        "Time:        53     83     72     88",
        "Distance:   333   1635   1289   1532"
    ];
    let result = solve_p1(input.join("\n"));
    assert!(result == 140220)
}

#[test]
fn test_result_2() {
    let input  = vec![
        "Time:        53     83     72     88",
        "Distance:   333   1635   1289   1532"
    ];
    let result = solve_p2(input.join("\n"));
    assert!(result == 39570185)
}
