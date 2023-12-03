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
    for line in lines {
        sum += get_power(&line);
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

fn get_power(game: &Vec<HashMap<String, i32>>) -> i32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;


    for round in game {
        if let Some(num) = round.get("red") {
            if *num > min_red {
                min_red = *num;
            }
        }

        if let Some(num) = round.get("green") {
            if *num > min_green {
                min_green = *num;
            }
        }

        if let Some(num) = round.get("blue") {
            if *num > min_blue {
                min_blue = *num;
            }
        }
    }

    return min_red * min_green * min_blue;
}