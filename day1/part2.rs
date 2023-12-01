use std::env;
use std::fs;
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let lines: Vec<String> = 
        fs::read_to_string(&args[1])
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

    let digits: HashMap<&str, char> = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9')
    ]);

    let mut count: u32 = 0;

    for line in lines {
        let mut num: String = "".to_string();

        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                num.push(c);
                break;
            } else if let Ok(d) = digit_prefix(&line[i..], &digits) {
                num.push(d);
                break;
            }
        }

        for (i, c) in line.chars().rev().enumerate() {
            if c.is_numeric() {
                num.push(c);
                break;
            } else if let Ok(d) = digit_prefix(&line[(line.len() - 1 - i)..], &digits) {
                num.push(d);
                break;
            }
        }

        assert!(num.len() == 2);

        count += num.parse::<u32>().unwrap();
    }

    println!("{count}");

}

fn is_prefix(big: &str, small: &str) -> bool {
    if big.len() < small.len() {
        return false;
    }

    let big_bytes = big.as_bytes();

    for (i, c) in small.chars().enumerate() {
        if big_bytes[i] as char != c {
            return false;
        }
    }

    return true; 
}

fn digit_prefix(string: &str, digits: &HashMap<&str, char>) -> Result<char, String> {
    for (key, value) in digits {
        if is_prefix(string, key) {
            return Ok(*value);
        }
    }

    return Err("No digit prefix was found.".to_string());
}