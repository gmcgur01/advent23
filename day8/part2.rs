use std::env;
use std::fs;
use std::collections::HashMap;

fn main () {
    let args: Vec<String> = env::args().collect();
    assert!(args.len() == 2);

    let lines: Vec<String> = 
        fs::read_to_string(&args[1])
            .unwrap()
            .lines()
            .map(String::from)
            .collect();

    let instructions: Vec<char> = lines[0].chars().collect();

    let mut node_map: HashMap<&str, (&str, &str)> = HashMap::new();

    for line in &lines[2..] {
        let my_split: Vec<&str> = line.split(" = ").collect();

        let node_id = my_split[0];
        let mut dest_split = my_split[1].trim_matches(|p| p == '(' || p == ')').split(", ");

        let left = dest_split.next().unwrap();
        let right = dest_split.next().unwrap();

        node_map.insert(node_id, (left, right));
    }

    let mut curr_nodes: Vec<&str> = node_map.keys().filter(|x| ends_with(x, 'A')).map(|x| *x).collect();

    println!("{:?}", curr_nodes);

    let mut num_steps: usize = 0;

    while !curr_nodes.iter().all(|x| ends_with(x, 'Z')) {

        for i in 0..curr_nodes.len() {
            let (left, right) = node_map[curr_nodes[i]];
            if instructions[num_steps % instructions.len()] == 'L' {
                curr_nodes[i] = left;
            } else {
                curr_nodes[i] = right;
            }
        }
        num_steps += 1;
    }

    println!("{}", num_steps);
}

fn ends_with(node_id: &str, letter: char) -> bool {
    return node_id.as_bytes()[2] == (letter as u8);
}