use std::collections::HashSet;
type Point = (i64, i64);

fn parse_input(contents: String) -> (Vec<Point>, i64, i64) {
    let mut pts = Vec::<Point>::new();
    let mut rows: i64 = 0;
    let mut cols: i64 = 0;
    for (y, line) in contents.lines().enumerate() {
        rows += 1;
        cols = line.len() as i64;
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                },
                '#' => {
                    pts.push((x as i64, y as i64));
                },
                _ => unreachable!("unexpected character in input"),
            }
        }
    }
    (pts, rows, cols)
}

pub fn solve_p1(contents: String) -> i64 {
    let mut total = 0;
    let (pts, rows, cols) = parse_input(contents);
    let mut expand_rows = HashSet::<i64>::new();
    let mut expand_cols = HashSet::<i64>::new();
    for i in 0..rows {
        if !pts.iter().any(|pt| { pt.0 == i as i64 }) {
            expand_cols.insert(i as i64);
        }
    }

    for i in 0..cols {
        if !pts.iter().any(|pt| { pt.1 == i as i64 }) {
            expand_rows.insert(i as i64);
        }
    }

    for i in 0..pts.len()-1 {
        for j in i+1..pts.len() {
            let start = pts[i];
            let end = pts[j];
            let x0 = std::cmp::min(start.0, end.0);
            let x1 = std::cmp::max(start.0, end.0);
            let y0 = std::cmp::min(start.1, end.1);
            let y1 = std::cmp::max(start.1, end.1);
            let x_exp = expand_cols.iter().filter(|c| { **c > x0 && **c < x1 }).count();
            let y_exp = expand_rows.iter().filter(|c| { **c > y0 && **c < y1 }).count();
            let dx = (x1 - x0) + x_exp as i64;
            let dy = (y1 - y0) + y_exp as i64;
            total += dx + dy;
        }
    }
    total
}

#[test]
fn test_case_1() {
    let input = vec![
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#....."
    ];
    let result = solve_p1(input.join("\n"));
    println!("{result}");
    assert!(result == 374);
}

pub fn solve_p2(contents: String) -> i64 {
    let mut total = 0;
    let (pts, rows, cols) = parse_input(contents);
    let mut expand_rows = HashSet::<i64>::new();
    let mut expand_cols = HashSet::<i64>::new();
    for i in 0..rows {
        if !pts.iter().any(|pt| { pt.0 == i as i64 }) {
            expand_cols.insert(i as i64);
        }
    }

    for i in 0..cols {
        if !pts.iter().any(|pt| { pt.1 == i as i64 }) {
            expand_rows.insert(i as i64);
        }
    }

    for i in 0..pts.len()-1 {
        for j in i+1..pts.len() {
            let start = pts[i];
            let end = pts[j];
            let x0 = std::cmp::min(start.0, end.0);
            let x1 = std::cmp::max(start.0, end.0);
            let y0 = std::cmp::min(start.1, end.1);
            let y1 = std::cmp::max(start.1, end.1);
            let x_exp = expand_cols.iter().filter(|c| { **c > x0 && **c < x1 }).count();
            let y_exp = expand_rows.iter().filter(|c| { **c > y0 && **c < y1 }).count();
            let dx = (x1 - x0) + (x_exp as i64 * 999999);
            let dy = (y1 - y0) + (y_exp as i64 * 999999);
            total += dx + dy;
        }
    }
    total
}

#[test]
fn test_case_2() {
    let input = vec![
        "...#......",
        ".......#..",
        "#.........",
        "..........",
        "......#...",
        ".#........",
        ".........#",
        "..........",
        ".......#..",
        "#...#....."
    ];
    let result = solve_p2(input.join("\n"));
    println!("{result}");
    assert!(result == 82000210);
}
