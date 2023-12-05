use std::env;
use std::fs;

fn main () {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let sections: Vec<Vec<String>> = 
        fs::read_to_string(&args[1])
            .unwrap()
            .split("\n\n")
            .map(|x| x.lines().map(String::from).collect())
            .collect();

    let seed_ranges: Vec<(u64,u64)> = seed_ranges(&sections[0][0][7..]); 
    
    let transitions: Vec<Vec<(u64, u64, u64)>> =
        sections[1..].iter().rev().map(str_vec_to_transition).collect();


    let mut i: u64 = 0;

    loop {
        let mut curr_val = i;

        for transition in &transitions {
            for (dest, src, range) in transition {
                if curr_val >= *dest && curr_val <= *dest + *range {
                    curr_val = *src + (curr_val - *dest); 
                    break;
                }
            }
        }

        if seed_ranges.iter().any(|(start, range)| curr_val >= *start && curr_val <= *start + *range) {
            println!("{i}");
            break;
        }

        i += 1;
    }


}

fn seed_ranges(seed_str: &str) -> Vec<(u64, u64)> {

    let mut seed_ranges: Vec<(u64, u64)> = Vec::new();

    let mut split = seed_str.split(" ");

    while let Some(start_str) = split.next() {
        let start = start_str.parse::<u64>().unwrap();
        let range = split.next().unwrap().parse::<u64>().unwrap();

        seed_ranges.push((start, range));
    }

    return seed_ranges;
}

fn str_vec_to_transition(str_vec: &Vec<String>) -> Vec<(u64, u64, u64)> {

    let mut transition: Vec<(u64, u64, u64)> = Vec::new();

    for line in &str_vec[1..] {
        let mut strs = line.split(" ");
        let dest = strs.next().unwrap().parse::<u64>().unwrap();
        let source = strs.next().unwrap().parse::<u64>().unwrap();
        let range = strs.next().unwrap().parse::<u64>().unwrap();

        transition.push((dest, source, range));
    }

    return transition;
}