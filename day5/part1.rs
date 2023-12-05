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

    let seeds: Vec<u64> = 
        sections[0][0][7..]
            .split(" ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();
    
    let transitions: Vec<Vec<(u64, u64, u64)>> =
        sections[1..].iter().map(str_vec_to_transition).collect();


    let mut min = u64::MAX;    
    for seed in seeds {

        let mut curr_val = seed;

        for transition in &transitions {
            for (dest, src, range) in transition {
                if curr_val >= *src && curr_val <= *src + *range {
                    curr_val = *dest + (curr_val - *src);
                    break;
                }
            }
        }

        if curr_val < min {
            min = curr_val
        }
    }

    println!("{min}");


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