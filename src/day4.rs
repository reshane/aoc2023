
use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    id: i32,
    winner: HashSet<i32>,
    actual: HashSet<i32>,
}

fn parse_cards(contents: String) -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::<Card>::new();
    for (_i, line) in contents.lines().enumerate() {
        // println!("{line}");
        let toks: Vec<&str> = line.split(" ").collect();
        // println!("{toks:?}");
        let mut colon_parsed = false;
        let mut pipe_parsed = false;
        let mut id: i32 = -1;
        let mut winner: HashSet<i32> = HashSet::<i32>::new();
        let mut actual: HashSet<i32> = HashSet::<i32>::new();
        for tok in toks {
            if tok.len() == 0 {
                // pass
            } else if !colon_parsed && !pipe_parsed {
                if &tok[tok.len()-1..] == ":" {
                    colon_parsed = true;
                    id = tok[0..tok.len()-1].parse::<i32>().unwrap();
                } else {
                    assert!(tok == "Card");
                }
            }
            else if colon_parsed && !pipe_parsed {
                if tok.parse::<i32>().is_ok() {
                    // winning number
                    winner.insert(tok.parse::<i32>().unwrap());
                } else if tok == "|" {
                    // pipe parsed
                    pipe_parsed = true;
                } else {
                    println!("{tok}: Only numbers or pipes allowed after colon");
                    unreachable!();
                }
            }
            else if colon_parsed && pipe_parsed {
                if tok.parse::<i32>().is_ok() {
                    // actual number
                    actual.insert(tok.parse::<i32>().unwrap());
                } else {
                    println!("{tok}: Should only get numbers after colon & pipe parsed");
                    unreachable!();
                }
            }
            else if !colon_parsed && pipe_parsed {
                println!("{tok}: Can't parse a pipe before a colon");
                unreachable!();
            }

        }
        assert!(id != -1);
        let card = Card {
            id,
            winner,
            actual,
        };
        // println!("{card:?}");
        cards.push(card);
    }
    cards
}

pub fn solve_p1(contents: String) -> i32 {
    let cards = parse_cards(contents);
    let mut total = 0;
    for card in cards {
        let mut card_score = 0;
        for num in card.actual {
            if card.winner.contains(&num) {
                if card_score == 0 {
                    card_score = 1;
                } else {
                    card_score *= 2;
                }
            }
        }
        // println!("{}: {}", card.id, card_score);
        total += card_score;
    }
    total
}

pub fn solve_p2(contents: String) -> i32 {
    let cards = parse_cards(contents);
    let cards_len = cards.len();
    let mut cards_count = vec![1; cards_len];
    // println!("{cards_count:?}");
    for card in cards {
        let mut card_winners = 0;
        for num in card.actual {
            if card.winner.contains(&num) {
                card_winners += 1;
            }
        }
        // println!("{card_winners}");
        let mut j = card.id as usize;
        while j < cards_len && j - (card.id as usize) < card_winners {
            cards_count[j] += cards_count[card.id as usize - 1];
            j += 1;
        }
        // println!("{cards_count:?}");
    }
    // println!("{cards_count:?}");
    cards_count.iter().sum::<i32>()
}

#[test]
fn test_case_1() {
    let input = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    ];

    let result = solve_p1(input.join("\n"));
    assert!(result == 13);
}

#[test]
fn test_case_2() {
    let input = vec![
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    ];

    let result = solve_p2(input.join("\n"));
    assert!(result == 30);
}
