use std::env;
use std::fs;
use std::collections::HashSet;

fn main () {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let points: Vec<u32> = 
        fs::read_to_string(&args[1])
            .unwrap()
            .lines()
            .map(line_to_points)
            .collect();

    println!("{}", points.iter().sum::<u32>());

}

fn line_to_points (mut line: &str) -> u32 {

    line = &line[line.split(": ").next().unwrap().len() + 2..].trim();
    let mut split = line.split(" | ");

    let mut winning_nums: HashSet<u32> = HashSet::new();

    let mut win_split = split.next().unwrap().split(" ");
    while let Some(num_str) = win_split.next() {
        if let Ok(winning_num) = num_str.parse::<u32>() {
            winning_nums.insert(winning_num);
        }
    }

    let mut score: u32 = 0;

    let mut my_split = split.next().unwrap().split(" ");
    while let Some(num_str) = my_split.next() {
        if let Ok(my_num) = num_str.parse::<u32>() {
            if !winning_nums.contains(&my_num) {
                continue;
            }
    
            if score == 0 {
                score = 1;
            } else {
                score *= 2;
            }
        }  
    }

    return score;
}