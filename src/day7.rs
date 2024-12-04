use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct Hand {
    value: String,
    kind: HandKind,
    bet: i64,
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
struct JokerHand {
    value: String,
    kind: HandKind,
    bet: i64,
}

#[derive(Eq, PartialEq, PartialOrd, Debug)]
enum HandKind {
    FiveOfKind,
    FourOfKind,
    FullHouse,
    ThreeOfKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandKind {
    fn get_kind_rank(&self) -> i64 {
        match self {
            Self::FiveOfKind => { 0 },
            Self::FourOfKind => { 1 },
            Self::FullHouse => { 2 },
            Self::ThreeOfKind => { 3 },
            Self::TwoPair => { 4 },
            Self::OnePair => { 5 },
            Self::HighCard => { 6 },
        }
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_kind = self.kind.get_kind_rank();
        let other_kind = other.kind.get_kind_rank();
        if self_kind == other_kind {
            return self.value.cmp(&other.value);
        }
        self_kind.cmp(&other_kind)
    }
}

impl FromStr for JokerHand {
    type Err = ();
    fn from_str(raw: &str) -> Result<Self, ()> {
        let raw: Vec<&str> = raw.split(" ").collect();
        let bet = raw[1].parse::<i64>().unwrap();
        let raw = raw[0];
        let src = "AKQT98765432J";
        let dst = "abcdefghijklm";
        let mut value = String::new();
        let freq_map: &mut [i64] = &mut [0; 13];
        for c in raw.chars() {
            let idx = src.find(c).unwrap();
            freq_map[idx] += 1;
            value.push_str(&dst[idx..idx+1]);
        }
        let jokers = freq_map[12];
        freq_map[12] = 0;
        let mut freq_map = freq_map.to_vec();
        freq_map.sort();
        freq_map[12] += jokers;
        let type_string = freq_map
            .into_iter()
            .filter(|e| { *e != 0 })
            .map(|e| { format!("{}", e) })
            .collect::<Vec<String>>()
            .join("");
        // classify it based off the string
        let kind = match type_string.as_str() {
            "5" => HandKind::FiveOfKind,
            "14" => HandKind::FourOfKind,
            "23" => HandKind::FullHouse,
            "113" => HandKind::ThreeOfKind,
            "122" => HandKind::TwoPair,
            "1112" => HandKind::OnePair,
            "11111" => HandKind::HighCard,
            &_ => panic!("invalid type string"),
        };
        // println!("{type_string}");
        Ok(JokerHand {
            value,
            kind,
            bet,
        })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_kind = self.kind.get_kind_rank();
        let other_kind = other.kind.get_kind_rank();
        if self_kind == other_kind {
            return self.value.cmp(&other.value);
        }
        self_kind.cmp(&other_kind)
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(raw: &str) -> Result<Self, ()> {
        let raw: Vec<&str> = raw.split(" ").collect();
        let bet = raw[1].parse::<i64>().unwrap();
        let raw = raw[0];
        let src = "AKQJT98765432";
        let dst = "abcdefghijklm";
        let mut value = String::new();
        let freq_map: &mut [i64] = &mut [0; 13];
        for c in raw.chars() {
            let idx = src.find(c).unwrap();
            freq_map[idx] += 1;
            value.push_str(&dst[idx..idx+1]);
        }
        let mut freq_map = freq_map.to_vec();
        freq_map.sort();
        let type_string = freq_map
            .into_iter()
            .filter(|e| { *e != 0 })
            .map(|e| { format!("{}", e) })
            .collect::<Vec<String>>()
            .join("");
        // classify it based off the string
        let kind = match type_string.as_str() {
            "5" => HandKind::FiveOfKind,
            "14" => HandKind::FourOfKind,
            "23" => HandKind::FullHouse,
            "113" => HandKind::ThreeOfKind,
            "122" => HandKind::TwoPair,
            "1112" => HandKind::OnePair,
            "11111" => HandKind::HighCard,
            &_ => panic!("invalid type string"),
        };
        // println!("{type_string}");
        Ok(Hand {
            value,
            kind,
            bet,
        })
    }
}

pub fn solve_p1(contents: String) -> i64 {
    let mut hands: Vec<Hand> = vec![];
    for line in contents.lines() {
        let hand: Hand = Hand::from_str(line).expect("Invalid input");
        hands.push(hand);
    }
    hands.sort_by(|a, b| b.cmp(a));
    let mut total: i64 = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        total += (i as i64 + 1) * hand.bet;
    }
    total
}

pub fn solve_p2(contents: String) -> i64 {
    let mut hands: Vec<JokerHand> = vec![];
    for line in contents.lines() {
        let hand: JokerHand = JokerHand::from_str(line).expect("Inavalid input");
        hands.push(hand);
    }
    hands.sort_by(|a, b| b.cmp(a));
    let mut total: i64 = 0;
    for (i, hand) in hands.into_iter().enumerate() {
        total += (i as i64 + 1) * hand.bet;
    }
    total
}

#[test]
fn test_case_1() {
    let input  = vec![
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483"
    ];
    let result = solve_p1(input.join("\n"));
    assert!(result == 6440);
}

#[test]
fn test_case_2() {
    let input  = vec![
        "32T3K 765",
        "T55J5 684",
        "KK677 28",
        "KTJJT 220",
        "QQQJA 483"
    ];
    let result = solve_p2(input.join("\n"));
    assert!(result == 5905);
}

