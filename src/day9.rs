
fn parse_input(contents: String) -> Vec<Vec<i64>> {
    let mut sequences = Vec::<Vec<i64>>::new();
    for line in contents.lines() {
        let seq: Vec<i64> = line.split(" ")
            .filter(|t| { t.len() > 0 })
            .map(|t| { t.parse::<i64>().unwrap() })
            .collect();
        sequences.push(seq);
    }
    return sequences;
}

pub fn solve_p1(contents: String) -> i64 {
    let seqs: Vec<Vec<i64>> = parse_input(contents);
    let mut sums: Vec<i64> = vec![0; seqs.len()];
    let mut k = 0;
    for seq in seqs {
        // println!("{:?}", seq);
        let mut done = false;
        let mut derivs: Vec<Vec<i64>> = Vec::<Vec<i64>>::new();
        sums[k] += seq[seq.len()-1];
        derivs.push(seq);
        while !done {
            let mut i = 0;
            let mut diffs = Vec::<i64>::new();
            while i < derivs[0].len() - 1 {
                let d = derivs[0][i+1] - derivs[0][i];
                diffs.push(d);
                i += 1;
            }
            done = true;
            for e in &diffs {
                if *e != 0 {
                    done = false;
                }
            }
            // println!("{:?}", diffs);
            sums[k] += diffs[diffs.len()-1];
            derivs.insert(0, diffs);
            // println!("{:?}", sums);
        }
        k += 1;
    }
    sums.into_iter().sum()
}

pub fn solve_p2(contents: String) -> i64 {
    let seqs: Vec<Vec<i64>> = parse_input(contents);
    let mut sums: Vec<i64> = vec![0; seqs.len()];
    let mut stack: Vec<i64> = vec![];
    let mut k = 0;
    for seq in seqs {
        // println!("{:?}", seq);
        let mut done = false;
        let mut derivs: Vec<Vec<i64>> = Vec::<Vec<i64>>::new();
        stack.push(seq[0]);
        derivs.push(seq);
        while !done {
            let mut i = 0;
            let mut diffs = Vec::<i64>::new();
            while i < derivs[0].len() - 1 {
                let d = derivs[0][i+1] - derivs[0][i];
                diffs.push(d);
                i += 1;
            }
            done = true;
            for e in &diffs {
                if *e != 0 {
                    done = false;
                }
            }
            // println!("{:?}", diffs);
            stack.push(diffs[0]);
            derivs.insert(0, diffs);
        }
        // println!("{:?}", sums);
        // println!("stack: {:?}", stack);
        let mut sum = stack.pop().unwrap();
        while !stack.is_empty() {
            sum = stack.pop().unwrap() - sum;
        }
        // stack.clear();
        sums[k] = sum;
        k += 1;
    }
    sums.into_iter().sum()
}

#[test]
fn test_case_1() {
    let input = vec![
        "0 3 6 9 12 15",
        "1 3 6 10 15 21",
        "10 13 16 21 30 45"
    ];
    let result = solve_p1(input.join("\n"));
    println!("{result}");
    assert!(result == 114);
}

#[test]
fn test_case_2() {
    let input = vec![
        "0 3 6 9 12 15",
        "1 3 6 10 15 21",
        "10 13 16 21 30 45"
    ];
    let result = solve_p2(input.join("\n"));
    println!("{result}");
    assert!(result == 2);
}
