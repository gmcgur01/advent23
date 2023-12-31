use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;

fn main () {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let grid: Vec<Vec<char>> = 
        fs::read_to_string(&args[1])
            .unwrap()
            .lines()
            .map(str::chars)
            .map(Iterator::collect)
            .collect();

    let mut gears: HashMap<(i32, i32), Vec<u32>> = HashMap::new(); 

    for (i, row) in grid.iter().enumerate() {
        let mut curr_num: u32 = 0;
        let mut curr_coords: HashSet<(i32, i32)> = HashSet::new();

        for (j, c) in row.iter().enumerate() {

            if c.is_numeric() {
                curr_num = (curr_num * 10) + c.to_digit(10).unwrap();
                for coord in is_adjacent_to_gear(&grid, i as i32, j as i32) {
                    curr_coords.insert(coord);
                }
            } else if curr_num != 0 {
                for coord in curr_coords {
                    if let Some(vec) = gears.get_mut(&coord) {
                        vec.push(curr_num);
                    } else {
                        gears.insert(coord, vec![curr_num]);
                    }
                }
                curr_num = 0;
                curr_coords = HashSet::new();
            } 
        }
        if curr_num != 0 {
            for coord in curr_coords {
                if let Some(vec) = gears.get_mut(&coord) {
                    vec.push(curr_num);
                } else {
                    gears.insert(coord, vec![curr_num]);
                }
            }
        } 
    }

    let mut sum = 0;
    for (_, nums) in gears {
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }

    println!("{}", sum);

}

fn is_adjacent_to_gear(grid: &Vec<Vec<char>>, row: i32, col: i32) -> Vec<(i32, i32)> {
    let row_indices = [row - 1, row, row + 1];
    let col_indices = [col - 1, col, col + 1];

    let mut gears: Vec<(i32, i32)> = Vec::new();

    for row_index in row_indices {
        if row_index < 0 || row_index >= grid[0].len() as i32 {
            continue;
        }

        for col_index in col_indices {
            if col_index < 0 || col_index >= grid.len() as i32 {
                continue;
            }

            let curr_char = grid[row_index as usize][col_index as usize];

            if curr_char == '*' {
                gears.push((row_index, col_index));
            }
        }
    }

    return gears;
}