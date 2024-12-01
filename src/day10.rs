use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(contents: String) -> ((i64, i64), HashMap<(i64, i64), Vec<(i64, i64)>>) {
    // 0,  1,    2,    3
    // up, down, left, right
    // F -> down, right
    // 7 -> down, left
    // - -> left, right
    // | -> up, down
    // J -> up, left
    // L -> up, right
    let mut ops_map = HashMap::<(i64, i64), Vec<i64>>::new();
    let mut start: (i64, i64) = (-1, -1);
    for (y, line) in contents.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let id = (x as i64, y as i64);
            let mut ops = Vec::<i64>::new();
            match c {
                'S' => {
                    start = (x as i64, y as i64);
                    ops = vec![0,1,2,3];
                }
                'F' => {
                    ops = vec![1,3];
                },
                '7' => {
                    ops = vec![1,2];
                },
                '-' => {
                    ops = vec![2,3];
                },
                '|' => {
                    ops = vec![0,1];
                },
                'J' => {
                    ops = vec![0,2];
                },
                'L' => {
                    ops = vec![0,3];
                },
                '.' => {},
                _ => unreachable!("unrecognized character!"),
            }
            ops_map.insert(id, ops);
        }
    }
    let mut map = HashMap::<(i64, i64), Vec<(i64, i64)>>::new();
    for (node, ops) in ops_map.clone().into_iter() {
        let mut neighbors = Vec::<(i64, i64)>::new();
        for op in ops {
            match op {
                0 => {
                    // go up
                    if ops_map.contains_key(&(node.0, node.1 - 1)) {
                        let neighbor_ops = ops_map.get(&(node.0, node.1 - 1)).unwrap();
                        if neighbor_ops.contains(&1) {
                            // put an edge in the graph from this to up
                            neighbors.push((node.0, node.1-1));
                        }
                    }
                },
                1 => {
                    // go down
                    if ops_map.contains_key(&(node.0, node.1 + 1)) {
                        let neighbor_ops = ops_map.get(&(node.0, node.1 + 1)).unwrap();
                        if neighbor_ops.contains(&0) {
                            // put an edge in the graph from this to down
                            neighbors.push((node.0, node.1+1));
                        }
                    }
                },
                2 => {
                    // go left
                    if ops_map.contains_key(&(node.0 - 1, node.1)) {
                        let neighbor_ops = ops_map.get(&(node.0 - 1, node.1)).unwrap();
                        if neighbor_ops.contains(&3) {
                            // put an edge in the graph from this to down
                            neighbors.push((node.0-1, node.1));
                        }
                    }
                },
                3 => {
                    // go right
                    if ops_map.contains_key(&(node.0 + 1, node.1)) {
                        let neighbor_ops = ops_map.get(&(node.0 + 1, node.1)).unwrap();
                        if neighbor_ops.contains(&2) {
                            // put an edge in the graph from this to down
                            neighbors.push((node.0+1, node.1));
                        }
                    }
                },
                _ => unreachable!("unknown operation"),
            }
        }
        map.insert(node, neighbors);
    }
    (start, map)
}

pub fn solve_p1(contents: String) -> i64 {
    let (start, map) = parse_input(contents);
    // println!("start: {start:?},\nmap: {map:?}");
    let mut current: (i64, i64) = start;
    let mut visited = HashSet::<(i64, i64)>::new();
    let mut queue = Vec::<(i64, i64)>::new();

    // push start into visited
    visited.insert(current);
    for n in map.get(&current).unwrap() {
        queue.push(*n);
    }

    while queue.len() != 0 {
        current = queue.pop().unwrap();
        for n in map.get(&current).unwrap() {
            if !visited.contains(&n) {
                queue.push(*n);
            }
        }
        visited.insert(current);
    }
    visited.len() as i64 / 2
}

#[test]
fn test_case_1() {
    let input = vec![
        ".....",
        ".S-7.",
        ".|.|.",
        ".L-J.",
        "....."
    ];
    let input1 = vec![
        "..F7.",
        ".FJ|.",
        "SJ.L7",
        "|F--J",
        "LJ..."
    ];
    let result = solve_p1(input.join("\n"));
    println!("{result:?}");
    assert!(result == 4);
    let result1 = solve_p1(input1.join("\n"));
    println!("{result1:?}");
    assert!(result1 == 8);
}

#[test]
fn test_result_1() {
    let contents = std::fs::read_to_string("day10_input.txt")
        .expect("Should have the input to solve the problem");
    let result = solve_p1(contents);
    assert!(result == 6768);
}

pub fn solve_p2(contents: String) -> i64 {
    let (start, map) = parse_input(contents);
    // println!("start: {start:?},\nmap: {map:?}");
    let mut current: (i64, i64) = start;
    let mut visited = HashSet::<(i64, i64)>::new();
    let mut queue = Vec::<(i64, i64)>::new();

    // push start into visited
    visited.insert(current);
    for n in map.get(&current).unwrap() {
        queue.push(*n);
    }

    while queue.len() != 0 {
        current = queue.pop().unwrap();
        for n in map.get(&current).unwrap() {
            if !visited.contains(&n) {
                queue.push(*n);
            }
        }
        visited.insert(current);
    }

    let mut visited_ord = Vec::<(i64, i64)>::new();
    // push start into visited
    let mut current: (i64, i64) = start;

    let mut i = 0;
    while i < visited.len() {
        visited_ord.push(current);
        i += 1;
        let neighbors = map.get(&current).unwrap();
        for n in neighbors {
            if visited.contains(&n) && !visited_ord.contains(&n) {
                current = *n;
                break;
            }
        }
    }
    // println!("{:?}", visited_ord);
    // A = i + b / 2 - 1
    // Area = i + visited.len() / 2 - 1
    let visited = visited_ord;
    // let v_last = visited.len()-1;
    // abs(sum i: j%n: 0->n ( det(pi, pi+1) ))
    let mut total: i64 = 0;
    let mut i = 0;
    while i < visited.len() {
        let n0 = visited[i];
        let n1 = visited[(i+1)%visited.len()];
        let det = (n0.0 * n1.1) - (n1.0 * n0.1);
        // println!("{:?}x{:?} = ({}*{} - {}*{}) = {det}", n0, n1, n0.0, n1.1, n0.1, n1.0);
        total += det;
        i += 1;
    }

    // println!("2*area: {}, border: {}", total.abs(), visited.len());
    (total.abs() / 2) - (visited.len() as i64 / 2) + 1
}

#[test]
fn test_case_2() {
    let input1 = vec![
        ".....",
        ".S-7.",
        ".|.|.",
        ".L-J.",
        "....."
    ];
    let result = solve_p2(input1.join("\n"));
    println!("{result:?}");
    assert!(result == 1);
    let input = vec![
        "FF7FSF7F7F7F7F7F---7",
        "L|LJ||||||||||||F--J",
        "FL-7LJLJ||||||LJL-77",
        "F--JF--7||LJLJ7F7FJ-",
        "L---JF-JLJ.||-FJLJJ7",
        "|F|F-JF---7F7-L7L|7|",
        "|FFJF7L7F-JF7|JL---7",
        "7-L-JL7||F7|L7F-7F7|",
        "L.L7LFJ|||||FJL7||LJ",
        "L7JLJL-JLJLJL--JLJ.L",
    ];
    let result = solve_p2(input.join("\n"));
    println!("{result:?}");
    assert!(result == 10);
}

#[test]
fn test_result_2() {
    let contents = std::fs::read_to_string("day10_input.txt")
        .expect("Should have the input to solve the problem");
    let result = solve_p2(contents);
    assert!(result == 351);
}
