
struct Game {
    #[allow(dead_code)]
    i: usize,
    r: Vec<i32>,
    g: Vec<i32>,
    b: Vec<i32>,
    valid: bool,
}

fn parse_game(line: &str) -> Game {
    let game_num = line[5..line.find(":").unwrap()].parse::<usize>().unwrap();
    let line_data: Vec<&str> = line[line.find(":").unwrap()..].split(" ").collect();
    let mut e: i32 = 0;
    let mut r: Vec<i32> = Vec::<i32>::new();
    let mut g: Vec<i32> = Vec::<i32>::new();
    let mut b: Vec<i32> = Vec::<i32>::new();
    let mut valid = true;
    for (i, tok) in line_data.into_iter().enumerate() {
        if i % 2 == 0 {
            match tok.chars().nth(0).unwrap() {
                ':' => {},
                'r' => {
                    r.push(e);
                    if e > 12 {
                        valid = false;
                    }
                },
                'g' => {
                    g.push(e);
                    if e > 13 {
                        valid = false;
                    }
                },
                'b' => {
                    b.push(e);
                    if e > 14 {
                        valid = false;
                    }
                },
                _ => {
                    unreachable!();
                },
            }
        } else {
            if let Some(count) = tok.parse::<i32>().ok() {
                // we have a count now
                e = count;
            } else {
                panic!("This should be a number??? {tok}");
            }
        }
    }
    Game {
        i: game_num,
        r,
        g,
        b,
        valid,
    }
}

pub fn solve_p1(contents: String) -> usize {
    let mut total = 0;
    for (i, line) in contents.lines().enumerate() {
        if parse_game(line).valid {
            total += i + 1;
        }
    }
    total
}

pub fn solve_p2(contents: String) -> i32 {
    let mut total = 0;
    for (_i, line) in contents.lines().enumerate() {
        let game = parse_game(line);

        let r = game.r.iter().max().copied().unwrap_or(0);
        let g = game.g.iter().max().copied().unwrap_or(0);
        let b = game.b.iter().max().copied().unwrap_or(0);
        total += r*b*g;
    }
    total
}


#[test]
fn test_case_1() {
    let input = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    ];

    let total = solve_p1(input.join("\n"));

    assert!(total == 8);
}

#[test]
fn test_case_2() {
    let input = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    ];

    let total = solve_p2(input.join("\n"));
    assert!(total == 2286);
}

#[test]
fn test_result_1() {
    let contents = std::fs::read_to_string("day2_input.txt")
        .expect("Should have the input to solve the problem");
    let p1 = solve_p1(contents.clone());
    assert!(p1 == 2369);
}
#[test]
fn test_result_2() {
    let contents = std::fs::read_to_string("day2_input.txt")
        .expect("Should have the input to solve the problem");
    let p2 = solve_p2(contents.clone());
    assert!(p2 == 66363);
}
