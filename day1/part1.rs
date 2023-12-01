use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let lines: Vec<String> = 
        fs::read_to_string(&args[1])
            .unwrap()
            .lines()
            .map(String::from)
            .collect();


    let mut count: u32 = 0;

    for line in lines {
        let mut num: String = "".to_string();

        for c in line.chars() {
            if c.is_numeric() {
                num.push(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_numeric() {
                num.push(c);
                break;
            }
        }

        assert!(num.len() == 2);

        count += num.parse::<u32>().unwrap();
    }

    println!("{count}");

}