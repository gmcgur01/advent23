use std::env;
use std::fs;

fn main () {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let lines: Vec<String> = 
        fs::read_to_string(&args[1])
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

    let mut time: u64 = 0;
    let mut distance: u64 = 0;

    for c in lines[0][11..].chars() {
        if let Some(digit) = c.to_digit(10) {
            time = time * 10 + (digit as u64);
        }
    }

    for c in lines[1][11..].chars() {
        if let Some(digit) = c.to_digit(10) {
            distance = distance * 10 + (digit as u64);
        }
    }

    println!("{}", calc_num_wins((time, distance)));
    
}


fn calc_num_wins(input: (u64, u64)) -> u32 {
    let (t, d) = input;

    let right: f64 = ((u64::pow(t, 2) - (4 * d)) as f64).sqrt();

    let start: f64 = ((t as f64) - right) / 2.0;
    let end:   f64 = ((t as f64) + right) / 2.0;

    let mut new_start: f64 = start.ceil();

    if new_start == start {
        new_start += 1.0;
    }

    let mut new_end: f64 = end.floor();

    if new_end == end {
        new_end -= 1.0;
    }

    let num_wins: u32 = (new_end + 1.0 - new_start) as u32;

    return num_wins;
}