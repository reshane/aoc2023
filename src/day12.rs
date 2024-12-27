
fn parse_input(contents: String) -> Vec<(String, Vec<usize>)> {
    contents.lines()
        .filter_map(|line| {
            if let Some((row, groups)) = line.split_once(" ") {
                return Some((row.to_string(), 
                    groups.split(",")
                        .map(|g| { g.parse::<usize>().unwrap() })
                        .collect::<Vec<usize>>()));
            }
            None
        }).collect::<Vec<(String, Vec<usize>)>>()
}

#[allow(dead_code)]
fn solve_row(row: String, groups: Vec<usize>) -> i64 {
    // println!("{row}, {groups:?}");
    if row.is_empty() && groups.is_empty() {
        return 1;
    }
    if row.is_empty() {
        return 0;
    }
    if groups.is_empty() {
        if row.contains("#") {
            return 0;
        }
        return 1;
    }
    // if the first character is a .
    //      we can't place a group here
    //      check the next position in the string
    if &row[0..1] == "." {
        // println!("matched . - we can't place a group here");
        let ans = solve_row(row[1..].to_string(), groups.clone());
        // println!("here: {row}, {groups:?} => {ans}");
        return ans;
    }
    // if the current character is a #
    //      if we can place the current group
    //          solve the row as if we did
    if &row[0..1] == "#" {
        if groups[0] <= row.len() && (groups[0] == row.len() || &row[groups[0]..groups[0]+1] == "." || &row[groups[0]..groups[0]+1] == "?") {
            let all_broken = row[0..groups[0]].chars().all(|c| { c == '#' || c == '?' });
            if !all_broken {
                // println!("matched # - we can't place a group here though");
                return 0;
            }
            // we can place a groupd here
            // println!("matched # - we can place a group here");
            let new_str = if groups[0] == row.len() {
                format!("")
            } else {
                format!(".{}", &row[groups[0]+1..])
            };
            // println!("{new_str}");
            let mut reduced_groups = groups.clone();
            reduced_groups.remove(0);
            // println!("{reduced_groups:?}");
            let ans = solve_row(new_str, reduced_groups);
            // println!("here2: {row}, {groups:?} => {ans}");
            return ans;
        }
    }
    if &row[0..1] == "?" {
        let not_placed = solve_row(row[1..].to_string(), groups.clone());
        if groups[0] <= row.len() && (groups[0] == row.len() || &row[groups[0]..groups[0]+1] == "." || &row[groups[0]..groups[0]+1] == "?") {
            // we can place a group here
            let all_broken = row[0..groups[0]].chars().all(|c| { c == '#' || c == '?' });
            if !all_broken {
                // println!("matched ? - we can't place a group here though");
                return not_placed;
            }
            // println!("matched ? - we can place a group here");
            let new_str = if groups[0] == row.len() {
                format!("")
            } else {
                format!(".{}", &row[groups[0]+1..])
            };
            // println!("{new_str}");
            let mut reduced_groups = groups.clone();
            reduced_groups.remove(0);
            // println!("{reduced_groups:?}");
            let ans = solve_row(new_str, reduced_groups) + not_placed;
            // println!("{row}, {groups:?} => {ans}");
            return ans;
        }
        return not_placed;
    }
    0
}

pub fn solve_p1(contents: String) -> i64 {
    let rows = parse_input(contents);
    let mut total = 0;
    let mut cache = HashMap::new();
    for (row, groups) in rows.into_iter() {
        let row_total = solve_row_mem(row.clone(), groups.clone(), &mut cache);
        // println!("solved row: {row}, {groups:?} => {row_total}");
        total += row_total;
    }
    total
}

#[test]
fn test_case_1() {
    let input = vec![
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];
    let result = solve_p1(input.join("\n"));
    println!("{result}");
    assert!(result == 21);
}

fn parse_input2(contents: String) -> Vec<(String, Vec<usize>)> {
    contents.lines()
        .filter_map(|line| {
            if let Some((row, groups)) = line.split_once(" ") {
                let groups = format!("{groups},{groups},{groups},{groups},{groups}");
                return Some((
                    format!("{row}?{row}?{row}?{row}?{row}"), 
                    groups.split(",")
                        .map(|g| { g.parse::<usize>().unwrap() })
                        .collect::<Vec<usize>>()));
            }
            None
        }).collect::<Vec<(String, Vec<usize>)>>()
}

use std::collections::HashMap;

fn solve_row_mem(row: String, groups: Vec<usize>, mem: &mut HashMap<(String, Vec<usize>), i64>) -> i64 {
    if let Some(cached) = mem.get(&(row.clone(), groups.clone())) {
        return *cached;
    }
    if row.is_empty() && groups.is_empty() {
        return 1;
    }
    if row.is_empty() {
        return 0;
    }
    if groups.is_empty() {
        if row.contains("#") {
            return 0;
        }
        return 1;
    }
    if &row[0..1] == "." {
        let ans = solve_row_mem(row[1..].to_string(), groups.clone(), mem);
        mem.insert((row.clone(), groups.clone()), ans);
        return ans;
    }
    if &row[0..1] == "#" {
        if groups[0] <= row.len() && (groups[0] == row.len() || &row[groups[0]..groups[0]+1] == "." || &row[groups[0]..groups[0]+1] == "?") {
            let all_broken = row[0..groups[0]].chars().all(|c| { c == '#' || c == '?' });
            if !all_broken {
                return 0;
            }
            let new_str = if groups[0] == row.len() {
                format!("")
            } else {
                format!(".{}", &row[groups[0]+1..])
            };
            let mut reduced_groups = groups.clone();
            reduced_groups.remove(0);
            let ans = solve_row_mem(new_str, reduced_groups, mem);
            mem.insert((row.clone(), groups.clone()), ans);
            return ans;
        }
    }
    if &row[0..1] == "?" {
        let not_placed = solve_row_mem(row[1..].to_string(), groups.clone(), mem);
        if groups[0] <= row.len() && (groups[0] == row.len() || &row[groups[0]..groups[0]+1] == "." || &row[groups[0]..groups[0]+1] == "?") {
            let all_broken = row[0..groups[0]].chars().all(|c| { c == '#' || c == '?' });
            if !all_broken {
                return not_placed;
            }
            let new_str = if groups[0] == row.len() {
                format!("")
            } else {
                format!(".{}", &row[groups[0]+1..])
            };
            let mut reduced_groups = groups.clone();
            reduced_groups.remove(0);
            let ans = solve_row_mem(new_str, reduced_groups, mem) + not_placed;
            mem.insert((row.clone(), groups.clone()), ans);
            return ans;
        }
        mem.insert((row.clone(), groups.clone()), not_placed);
        return not_placed;
    }
    0
}

pub fn solve_p2(contents: String) -> i64 {
    let rows = parse_input2(contents);
    let mut total = 0;
    let mut cache = HashMap::new();
    for (row, groups) in rows.into_iter() {
        let row_total = solve_row_mem(row.clone(), groups.clone(), &mut cache);
        total += row_total;
    }
    total
}

#[test]
fn test_case_2() {
    let input = vec![
        "???.### 1,1,3",
        ".??..??...?##. 1,1,3",
        "?#?#?#?#?#?#?#? 1,3,1,6",
        "????.#...#... 4,1,1",
        "????.######..#####. 1,6,5",
        "?###???????? 3,2,1",
    ];
    let result = solve_p2(input.join("\n"));
    println!("{result}");
    assert!(result == 525152);
}
