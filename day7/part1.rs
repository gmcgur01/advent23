use std::env;
use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;
use std::iter::zip;

fn main () {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let mut lines: Vec<(String, u32)> = 
        fs::read_to_string(&args[1])
            .unwrap()
            .lines()
            .map(parse_line)
            .collect();


    lines.sort_by(|(hand1, _), (hand2, _)| cmp_hand(hand1, hand2));


    let mut ret: u32 = 0;
    for (i, (_, bid)) in lines.iter().enumerate() {
        ret += bid * ((i as u32) + 1);
    }

    println!("{}", ret);
}

fn parse_line(x: &str) -> (String, u32) {
    let my_split: Vec<&str> = x.split(" ").collect();
    return (String::from(my_split[0]), my_split[1].parse::<u32>().unwrap())
}

fn card_freq(hand: &String) -> Vec<u32> {

    let mut freq: HashMap<char, u32> = HashMap::new();

    for c in hand.chars() {
        if freq.contains_key(&c) {
            freq.insert(c, freq[&c] + 1);
        } else {
            freq.insert(c, 1);
        }
    }

    let mut vec: Vec<u32> = freq.into_values().collect();
    vec.sort_by(|a, b| b.cmp(a));
    return vec;
}


fn cmp_hand(hand1: &String, hand2: &String) -> Ordering {

    let hand1_freq = card_freq(hand1);
    let hand2_freq = card_freq(hand2);

    // handles cases where one hand has more of a kind than another
    if hand1_freq[0] != hand2_freq[0] {
        return hand1_freq[0].cmp(&hand2_freq[0]);
    }

    // handles full house and two pair
    if hand1_freq[1] != hand2_freq[1] {
        return hand1_freq[1].cmp(&hand2_freq[1]);
    }
    
    // handle second ordering rule
    for (c1, c2) in zip(hand1.chars(), hand2.chars()) {
        let val1 = card_to_val(c1);
        let val2 = card_to_val(c2);

        match val1.cmp(&val2) {
            Ordering::Equal => (),
            other => return other
        }
    }

    return Ordering::Equal;
} 

fn card_to_val (card: char) -> u32 {
    return match card {
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => 0
    }
}