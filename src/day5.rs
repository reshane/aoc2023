
// [50..50+2] -> add 98-50
// [52..52+48] -> add 50-52
//
// [0..0+37] -> add 15-0
// [37..37+2] -> add 52-37
//
// for x in rows
//      range = [ax..ax+cx] 
//      op = bx-ax

use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Mapping {
    start: i64,
    end: i64,
    op: i64,
}

impl Mapping {
    fn apply(&self, x: i64) -> i64 {
        let mut y = x;
        if self.start <= x && x < self.end {
            y += self.op;
        }
        y
    }
}

impl FromStr for Mapping {
    type Err = ();
    fn from_str(raw: &str) -> Result<Self, ()> {
        let toks: Vec<&str> = raw.split(" ").collect();
        if toks.len() != 3 {
            return Err(());
        }

        // 50 98 2
        // [98..98+2] -> 50-98
        // [bx..bx+cx] -> ax-bx
        let ax = toks[0] // destination range start
            .parse::<i64>().map_err(|_e| return ()).unwrap();
        let bx = toks[1] // source range start
            .parse::<i64>().map_err(|_e| return ()).unwrap();
        let cx = toks[2] // mapping length
            .parse::<i64>().map_err(|_e| return ()).unwrap();
        Ok(Self {
            start: bx,
            end: bx+cx,
            op: ax-bx,
        })
    }
}

#[derive(Debug, Clone)]
struct Map {
    src: String,
    dst: String,
    mappings: Vec<Mapping>,
}

impl Map {
    fn apply(&self, x: i64) -> i64 {
        let mut y = x;
        for mapping in &self.mappings {
            if mapping.start <= x && x < mapping.end {
                y = mapping.apply(x);
            }
        }
        y
    }
}

impl FromStr for Map {
    type Err = ();
    fn from_str(raw: &str) -> Result<Self, ()> {
        let raw: Vec<&str> = raw.lines().collect();
        let toks: Vec<&str> = raw[0].split("-").collect();
        if toks.len() != 3 {
            return Err(());
        }
        let src = toks[0].to_string();
        let dst = toks[2];
        let dst = dst[0..dst.find(" ").unwrap()].to_string();
        let mut mappings = Vec::<Mapping>::new();
        let mut i = 1;
        loop {
            if i >= raw.len() {
                break;
            }
            let mapping = Mapping::from_str(raw[i]).map_err(|_e| return ()).unwrap();
            mappings.push(mapping);
            i += 1;
        }
        Ok(Self {
            src,
            dst,
            mappings,
        })
    }
}

fn parse_mappings(contents: String) -> (Vec<i64>, HashMap<String, Map>) {
    let mut map_strs: Vec<&str> = contents.split("\n\n").collect();
    let seeds = map_strs.remove(0);
    let seeds: Vec<i64> = seeds[seeds.find(" ").expect("should have space")..]
        .split(" ")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    let mut maps = HashMap::<String, Map>::new();
    for map_str in map_strs {
        let map = Map::from_str(map_str).unwrap();
        maps.insert(map.src.clone(), map);
    }
    (seeds, maps)
}

pub fn solve_p1(contents: String) -> i64 {
    let (seeds, maps) = parse_mappings(contents);
    let mut min = i64::MAX;
    for seed in seeds {
        let mut target = "seed".to_string();
        let mut y = seed;
        while maps.contains_key(&target) {
            let current_map = maps.get(&target).unwrap();
            y = current_map.apply(y);
            target = current_map.dst.clone();
        }
        if y < min {
            min = y;
        }
    }
    min
}

fn calculate_location(maps: &HashMap<String, Map>, x: i64) -> i64 {
    let mut y = x;
    let mut target = "seed".to_string();
    while maps.contains_key(&target) {
        let current_map = maps.get(&target).unwrap();
        y = current_map.apply(y);
        target = current_map.dst.clone();
    }
    y
}

fn find_min(maps: &HashMap<String, Map>, start: i64, end: i64) -> i64 {
    let length = end - start;
    if length == 1 {
        return std::cmp::min(calculate_location(maps, start), 
            calculate_location(maps, start+1));
    }
    let middle = start + (length / 2);
    let mapped_start = calculate_location(maps, start);
    let mapped_middle = calculate_location(maps, middle);
    let mapped_end = calculate_location(maps, end);
    let mut min = mapped_start;
    if mapped_start + (length / 2) != mapped_middle {
        min = find_min(maps, start, middle);
    }
    if mapped_middle + (length - middle) != mapped_end {
        min = std::cmp::min(min, find_min(maps, middle, end));
    }
    min
}

pub fn solve_p2(contents: String) -> i64 {
    // map each range
    let (seeds, maps) = parse_mappings(contents);
    let mut min = i64::MAX;
    let mut i = 0;
    while i + 1 < seeds.len() {
        let y = find_min(&maps, seeds[i], seeds[i]+seeds[i+1]);
        if y < min {
            min = y;
        }
        i += 2;
    }
    min
}

#[test]
fn test_map_parsing() {
    let input = vec![
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48"
    ];
    let map = Map::from_str(&input.join("\n")).unwrap();
    println!("{:?}", map);
    let x = 99 as i64;
    let y = 51 as i64;
    println!("{x} -> {}", map.apply(x));
    assert!(y == map.apply(x));
    let x = 98 as i64;
    let y = 50 as i64;
    println!("{x} -> {}", map.apply(x));
    assert!(y == map.apply(x));
}

#[test]
fn test_case_1() {
    let input = vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4"
    ];
    let result = solve_p1(input.join("\n"));
    assert!(result == 35);
}

#[test]
fn test_result_1() {
    let contents = std::fs::read_to_string("day5_input.txt")
        .expect("Should have the input to solve the problem");
    assert!(solve_p1(contents.clone()) == 1181555926);
}

#[test]
fn test_case_2() {
    let input = vec![
        "seeds: 79 14 55 13",
        "",
        "seed-to-soil map:",
        "50 98 2",
        "52 50 48",
        "",
        "soil-to-fertilizer map:",
        "0 15 37",
        "37 52 2",
        "39 0 15",
        "",
        "fertilizer-to-water map:",
        "49 53 8",
        "0 11 42",
        "42 0 7",
        "57 7 4",
        "",
        "water-to-light map:",
        "88 18 7",
        "18 25 70",
        "",
        "light-to-temperature map:",
        "45 77 23",
        "81 45 19",
        "68 64 13",
        "",
        "temperature-to-humidity map:",
        "0 69 1",
        "1 0 69",
        "",
        "humidity-to-location map:",
        "60 56 37",
        "56 93 4"
    ];
    let result = solve_p2(input.join("\n"));
    println!("{result}");
    assert!(result == 46);
}

#[test]
fn test_result_2() {
    let contents = std::fs::read_to_string("day5_input.txt")
        .expect("Should have the input to solve the problem");
    assert!(solve_p2(contents) == 37806486);
}
