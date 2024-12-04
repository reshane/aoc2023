use std::collections::HashMap;

// we have a directed graph
// we have to follow the directions
// next it'll ask us the shorted route from AAA -> ZZZ

fn parse_input(contents: String) -> (String, HashMap<String, (String, String)>) {
    let mut map_strs: Vec<&str> = contents.split("\n").collect();
    // println!("{:?}", map_strs);
    let moves = map_strs.remove(0).to_string();
    // println!("{:?}", map_strs);
    let _ = map_strs.remove(0);
    // println!("{:?}", map_strs);
    let mut map = HashMap::<String, (String, String)>::new();
    for map_str in map_strs {
        if map_str.len() == 0 {
            continue;
        }
        // println!("{map_str}");
        let key = &map_str[..map_str.find(" ").unwrap()];
        let left = &map_str[map_str.find("(").unwrap()+1..map_str.find(",").unwrap()];
        let right = &map_str[map_str.find(", ").unwrap()+2..map_str.find(")").unwrap()];
        // println!("{}: {}, {}", key, left, right);
        map.insert(key.to_string(), (left.to_string(), right.to_string()));
    }
    (moves, map)
}


pub fn solve_p1(contents: String) -> i64 {
    let (moves, map) = parse_input(contents);
    let mut current = "AAA".to_string();
    let mut total: i64 = 0;
    // println!("L of CCC: {:?}", map.get("CCC"));
    while current != "ZZZ" {
        for c in moves.chars() {
            total += 1;
            match c {
                'R' => {
                    // println!("R of {current}");
                    current = map.get(&current).unwrap().1.clone();
                },
                'L' => {
                    // println!("L of {current}");
                    current = map.get(&current).unwrap().0.clone();
                },
                _ => panic!("UNEXPECTED MOVE CHARACTER"),
            }
            if current == "ZZZ" {
                // println!("REACHED ZZZ {current}");
                break;
            }
            //total += 1;
        }
        // total += moves.len() as i64;
    }
    // println!("{total:?}");
    total
}

fn solve_path(start: String, moves: String, map: HashMap<String, (String, String)>) -> i64 {
    let mut current = start;
    let mut total: i64 = 0;
    while &current[2..] != "Z" {
        for c in moves.chars() {
            total += 1;
            match c {
                'R' => {
                    // println!("R of {current}");
                    current = map.get(&current).unwrap().1.clone();
                },
                'L' => {
                    // println!("L of {current}");
                    current = map.get(&current).unwrap().0.clone();
                },
                _ => panic!("UNEXPECTED MOVE CHARACTER"),
            }
            if &current[2..] == "Z" {
                // println!("REACHED **Z {current}");
                break;
            }
            //total += 1;
        }
        // total += moves.len() as i64;
    }
    total
}

fn find_lcm(x1: i64, x2: i64) -> i64 {
    let max = std::cmp::max(x1, x2);
    let min = std::cmp::min(x1, x2);
    let mut lcm = max;
    while lcm % min != 0 {
        lcm += max;
    }
    lcm
}

fn lcm_vec(mut vals: Vec<i64>) -> i64 {
    let mut lcm = 1;
    while vals.len() > 0 {
        lcm = find_lcm(lcm, vals.pop().expect("There should be a value to pop"));
    }
    lcm
}

pub fn solve_p2(contents: String) -> i64 {
    let (moves, map) = parse_input(contents);
    // find all the keys that end with A
    // solve their paths
    // take lcm of the solved paths
    let starts: Vec<String> = map.keys()
        .filter(|k| { &k[2..] == "A" })
        .map(|s| s.clone())
        .collect();
    // println!("{:?}", starts);
    let mut path_szs: Vec<i64> = vec![];
    for start in starts {
        let solved_path = solve_path(start, moves.clone(), map.clone());
        // println!("{solved_path}");
        path_szs.push(solved_path);
    }
    lcm_vec(path_szs)
}

#[test]
fn test_case_1() {
    let input = vec![
        "RL",
        "",
        "AAA = (BBB, CCC)",
        "BBB = (DDD, EEE)",
        "CCC = (ZZZ, GGG)",
        "DDD = (DDD, DDD)",
        "EEE = (EEE, EEE)",
        "GGG = (GGG, GGG)",
        "ZZZ = (ZZZ, ZZZ)"
    ];
    let result = solve_p1(input.join("\n"));
    assert!(result == 2);
}

#[test]
fn test_case_1a() {
    let input = vec![
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ];
    let result = solve_p1(input.join("\n"));
    assert!(result == 6);
}

#[test]
fn test_case_2() {
    let input = vec![
        "LR",
        "",
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)"
    ];
    let result = solve_p2(input.join("\n"));
    assert!(result == 6);
}

