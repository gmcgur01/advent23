use std::fs;
use std::env;
use std::collections::HashMap;

fn main () {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let lines: Vec<Vec<HashMap<String, i32>>> = 
        fs::read_to_string(&args[1])
            .unwrap()
            .lines()
            .map(String::from)
            .map(str_to_game)
            .collect();

    let mut sum = 0;
    for (i, line) in lines.iter().enumerate() {
        if verify_game(line) {
            sum += i + 1;
        }
    }

    println!("{}", sum);
}

fn str_to_game(input: String) -> Vec<HashMap<String, i32>> {

    let games = &input[(input.split(": ").next().unwrap().len() + 2)..];

    let mut game_split = games.split("; ");
    let mut vec = Vec::new();

    while let Some(game_str) = game_split.next() {

        let mut color_split = game_str.split(", ");

        let mut game = HashMap::new();
        while let Some(color_str) = color_split.next() {
            let mut cube_split = color_str.split(" ");

            let number: i32 = cube_split.next().unwrap().parse().unwrap();
            let color: &str = cube_split.next().unwrap();

            game.insert(
                color.to_string(),
                number,
            );
        }

        vec.push(game);
    }

    return vec;

}

fn verify_game(game: &Vec<HashMap<String, i32>>) -> bool {
    for round in game {
        if let Some(num) = round.get("red") {
            if *num > 12 {
                return false;
            }
        }

        if let Some(num) = round.get("green") {
            if *num > 13 {
                return false;
            }
        }

        if let Some(num) = round.get("blue") {
            if *num > 14 {
                return false;
            }
        }
    }

    return true;
}