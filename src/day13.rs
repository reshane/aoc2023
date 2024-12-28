
fn parse_pattern(contents: &str) -> (Vec<String>, Vec<String>) {
    let rows = contents.lines()
        .filter_map(|line| {
            if !line.is_empty() {
                return Some(line.to_string());
            }
            None
        }).collect::<Vec<String>>();
    let mut cols = vec![String::default(); rows[0].len()];
    contents.lines()
        .for_each(|line| {
            line.chars().enumerate()
                .for_each(|(i, c)| {
                    cols[i].push_str(&format!("{c}"));
                });
        });
    (rows, cols)
}

fn parse_input(contents: String) -> Vec<(Vec<String>, Vec<String>)> {
    contents.split("\n\n")
        .map(|pattern| {
            parse_pattern(pattern)
        })
        .collect::<Vec<(Vec<String>, Vec<String>)>>()
}

pub fn solve_p1(contents: String) -> i64 {
    let patterns = parse_input(contents);
    let mut total = 0;
    for (rows, cols) in patterns.into_iter() {
        // rows.iter().for_each(|row| { println!("{row}"); });
        // cols.iter().for_each(|col| { println!("{col}"); });
        let mut horizontal = vec![];
        for i in 1..rows.len() {
            if rows[i-1] == rows[i] {
                horizontal.push(i);
            }
        }
        let mut vertical = vec![];
        for i in 1..cols.len() {
            if cols[i-1] == cols[i] {
                vertical.push(i);
            }
        }
        let horizontal = horizontal.into_iter().filter(|h| {
            let mut h1 = h-1;
            let mut h2 = *h;
            while h1 != 0 && h2 < rows.len()-1 {
                h1 -= 1;
                h2 += 1;
                if rows[h1] != rows[h2] {
                    // println!("{h}: rows[{h1}] ({}) != rows[{h2}] ({})", rows[h1], rows[h2]);
                    return false;
                }
            }
            true
        }).collect::<Vec<usize>>();
        let vertical = vertical.into_iter().filter(|h| {
            let mut h1 = h-1;
            let mut h2 = *h;
            while h1 != 0 && h2 < cols.len()-1 {
                h1 -= 1;
                h2 += 1;
                if cols[h1] != cols[h2] {
                    // println!("{h}: cols[{h1}] ({}) != cols[{h2}] ({})", cols[h1], cols[h2]);
                    return false;
                }
            }
            true
        }).collect::<Vec<usize>>();
        assert!(horizontal.is_empty() || vertical.is_empty());
        assert!(!horizontal.is_empty() || !vertical.is_empty());
        if horizontal.is_empty() {
            total += vertical[0] as i64;
        } else {
            total += (horizontal[0] * 100) as i64;
        }
        // assert!(horizontal != vertical, "{rows:?}, {cols:?}");
        // total += std::cmp::max(horizontal*100, vertical);
    }
    total
}

#[test]
fn test_case_1() {
    let input = vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
        "",
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ];
    let result = solve_p1(input.join("\n"));
    println!("{result}");
    assert!(result == 405);
}

#[test]
fn test_case_1b() {
    let input = vec![
        "..#####", 
        "..#....", 
        "..#####", 
        "##...#.", 
        ".#.....", 
        ".####.#", 
        ".####.#", 
        "##.....", 
        "##...#.", 
        "..#####", 
        "..#....", 
        "..#####", 
        "..#####",
    ];
    let result = solve_p1(input.join("\n"));
    println!("{result}");
    assert!(result == 1200);
}

fn string_diff(a: &String, b: &String) -> usize {
    assert!(a.len() == b.len());
    let mut diff = 0;
    for i in 0..a.len() {
        if a[i..i+1] != b[i..i+1] {
            diff += 1;
        }
    }
    diff
}

pub fn solve_p2(contents: String) -> i64 {
    let patterns = parse_input(contents);
    let mut total = 0;
    for (rows, cols) in patterns.into_iter() {
        // rows.iter().for_each(|row| { println!("{row}"); });
        // cols.iter().for_each(|col| { println!("{col}"); });
        let mut horizontal = vec![];
        for i in 1..rows.len() {
            if string_diff(&rows[i-1], &rows[i]) == 1 {
                horizontal.push((i, true));
            }
            if rows[i-1] == rows[i] {
                horizontal.push((i, false));
            }
        }
        let mut vertical = vec![];
        for i in 1..cols.len() {
            if string_diff(&cols[i-1], &cols[i]) == 1 {
                vertical.push((i, true));
            }
            if cols[i-1] == cols[i] {
                vertical.push((i,false));
            }
        }
        let horizontal = horizontal.into_iter().filter(|(h, edited)| {
            let mut h1 = h-1;
            let mut h2 = *h;
            let mut edited = *edited;
            while h1 != 0 && h2 < rows.len()-1 {
                h1 -= 1;
                h2 += 1;
                if rows[h1] != rows[h2] {
                    if !edited && string_diff(&rows[h1], &rows[h2]) == 1 {
                        edited = true;
                        continue;
                    }
                    // println!("{h}: rows[{h1}] ({}) != rows[{h2}] ({})", rows[h1], rows[h2]);
                    return false;
                }
            }
            edited
        }).map(|(h,_)| { h }).collect::<Vec<usize>>();
        let vertical = vertical.into_iter().filter(|(h, edited)| {
            let mut h1 = h-1;
            let mut h2 = *h;
            let mut edited = *edited;
            while h1 != 0 && h2 < cols.len()-1 {
                h1 -= 1;
                h2 += 1;
                if cols[h1] != cols[h2] {
                    if !edited && string_diff(&cols[h1], &cols[h2]) == 1 {
                        edited = true;
                        continue;
                    }
                    // println!("{h}: cols[{h1}] ({}) != cols[{h2}] ({})", cols[h1], cols[h2]);
                    return false;
                }
            }
            edited
        }).map(|(h,_)| { h }).collect::<Vec<usize>>();
        assert!(horizontal.is_empty() || vertical.is_empty(), "{horizontal:?}, {vertical:?}");
        assert!(!horizontal.is_empty() || !vertical.is_empty());
        if horizontal.is_empty() {
            total += vertical[0] as i64;
        } else {
            total += (horizontal[0] * 100) as i64;
        }
        // assert!(horizontal != vertical, "{rows:?}, {cols:?}");
        // total += std::cmp::max(horizontal*100, vertical);
    }
    total
}

#[test]
fn test_case_2() {
    let input = vec![
        "#.##..##.",
        "..#.##.#.",
        "##......#",
        "##......#",
        "..#.##.#.",
        "..##..##.",
        "#.#.##.#.",
        "",
        "#...##..#",
        "#....#..#",
        "..##..###",
        "#####.##.",
        "#####.##.",
        "..##..###",
        "#....#..#",
    ];
    let result = solve_p2(input.join("\n"));
    println!("{result}");
    assert!(result == 400);
}
