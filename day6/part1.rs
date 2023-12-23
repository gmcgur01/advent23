use std::env;
use std::fs;
use std::iter::zip;

fn main () {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let lines: Vec<String> = 
        fs::read_to_string(&args[1])
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

    let mut times:     Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    for slice in lines[0][11..].split(" ") {
        if let Ok(num) = slice.parse::<u64>() {
            times.push(num);
        }
    }

    for slice in lines[1][11..].split(" ") {
        if let Ok(num) = slice.parse::<u64>() {
            distances.push(num);
        }
    }

    let res: Vec<u32> = zip(times, distances).map(calc_num_wins).collect();

    println!("{}", res.iter().product::<u32>());
    
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