
type Manual = Vec<String>;

fn parse_manual(raw: String) -> Manual {
    let mut manual: Manual = Vec::new();
    for (_i, line) in raw.lines().enumerate() {
        manual.push(line.to_string());
    }
    manual
}

fn contains_symbol(neighbors: &str) -> bool {
    let mut sym_found = false;
    for c in neighbors.chars() {
        if c != '.' && !c.is_numeric() {
            sym_found = true;
        }
    }
    sym_found
}

pub fn solve_p1(contents: String) -> i32 {
    let manual: Manual = parse_manual(contents);
    let mut total = 0;
    for (k, row) in manual.clone().into_iter().enumerate() {
        let mut i = 0;
        let mut j = 1;
        loop {
            while !row[i..j].parse::<i32>().is_ok() && j < row.len() {
                i += 1;
                j += 1;
            }
            while j < row.len() && row[i..j+1].parse::<i32>().is_ok() {
                j += 1;
            }

            let mut valid = false;

            let mut n_i = i;
            if i > 0 {
                n_i -= 1;
                let neighbor = &row[n_i..i];
                valid = valid || contains_symbol(neighbor);
            }
            let mut n_j = j;
            if j < row.len() {
                n_j += 1;
                let neighbor = &row[j..n_j];
                valid = valid || contains_symbol(neighbor);
            }

            if k > 1 {
                // check row before neighbors
                let neighbors = &manual.get(k-1).unwrap()[n_i..n_j];
                valid = valid || contains_symbol(neighbors);
            }
            if k < manual.len() - 1 {
                // check row after neighbors
                let neighbors = &manual.get(k+1).unwrap()[n_i..n_j];
                valid = valid || contains_symbol(neighbors);
            }

            if valid {
                total += row[i..j].parse::<i32>().unwrap();
            }

            if j == row.len() {
                break;
            } else {
                i = j;
                j = i + 1;
            }
        }
    }
    total
}

pub fn parse_overlapping_ints(row: String, s_idx: usize, e_idx: usize) -> Vec<i32> {
    let mut members = Vec::<i32>::new();
    let mut i = 0;
    let mut j = 1;
    loop {
        while j < row.len() && !row[i..j].parse::<i32>().is_ok() {
            i += 1;
            j += 1;
        }
        while j < row.len() && row[i..j+1].parse::<i32>().is_ok() {
            j += 1;
        }
        if row[i..j].parse::<i32>().is_ok() && ((i <= e_idx && i >= s_idx) || (j <= e_idx && j > s_idx)) {
            members.push(row[i..j].parse::<i32>().unwrap())
        }
        if j == row.len() {
            break;
        } else {
            i = j;
            j = i + 1;
        }
    }
    members
}

fn get_numeric_neighbors(manual: Manual, row: usize, col: usize) -> Vec<i32> {
    let mut s_idx = col;
    let mut neighbors = Vec::<i32>::new();
    if col > 0 {
        s_idx -= 1;
    }

    let mut e_idx = col;
    if col < manual.get(row).unwrap().len() {
        e_idx += 1;
    }

    // if row > 0 {
        // println!("{}", manual.get(row-1).unwrap().clone());
    // }
    // println!("{}", manual.get(row).unwrap().clone());
    // println!("{} < {}", row, manual.len());
    // if row+1 < manual.len() {
        // println!("{}", manual.get(row+1).unwrap().clone());
    // }

    if row > 0 {
        let na = parse_overlapping_ints(manual.get(row-1).unwrap().clone(), s_idx, e_idx);
        neighbors.extend_from_slice(&na);
    }
    if row+1 < manual.len() {
        let nb = parse_overlapping_ints(manual.get(row+1).unwrap().clone(), s_idx, e_idx);
        neighbors.extend_from_slice(&nb);
    }
    let nw = parse_overlapping_ints(manual.get(row).unwrap().clone(), s_idx, e_idx);
    neighbors.extend_from_slice(&nw);
    neighbors
}

pub fn solve_p2(contents: String) -> i32 {
    let manual: Manual = parse_manual(contents);
    let mut total = 0;
    for (i, row) in manual.clone().into_iter().enumerate() {
        let mut j = 0;
        while let Some(idx) = row[j..].find("*") {
            let idx = idx + j;
            let neighbors = get_numeric_neighbors(manual.clone(), i, idx);
            if neighbors.len() == 2 {
                total += neighbors.get(0).unwrap() * neighbors.get(1).unwrap();
            }
            j = idx + 1;
        }
    }
    total
}

#[test]
fn test_case_1() {
    let lines = vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598.."
    ];

    let result = solve_p1(lines.join("\n"));
    assert!(result == 4361);
}

#[test]
fn test_case_2() {
    let lines = vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598.."
    ];

    let result = solve_p2(lines.join("\n"));
    assert!(result == 467835);
}

#[test]
fn test_case_3() {
    let lines = vec![
        ".....154",
        ".560*...",
    ];
    let result = solve_p2(lines.join("\n"));
    assert!(result == (560*154));
}

#[test]
fn test_parse_overlapping_ints() {
    let lines = vec![
        "467..114..",
        "...*......",
        "..35..633.",
        "......#...",
        "617*......",
        ".....+.58.",
        "..592.....",
        "......755.",
        "...$.*....",
        ".664.598.."
    ];

    let input = String::from(*lines.get(lines.len()-1).unwrap());

    let result = parse_overlapping_ints(input, 3, 5);
    assert!(*result.get(0).unwrap() == 664);
    assert!(*result.get(1).unwrap() == 598);
}
