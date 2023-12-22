use std::fs;

#[derive(Debug)]
struct Node {
    start: String,
    left: String,
    right: String,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    // Collect instructionss into a vector of chars (for convenience)
    let instructions = lines[0].chars().collect::<Vec<char>>();
    let mut nodes = vec![];

    for i in 2..lines.len() {
        let line = lines[i];

        // Filter out line into just characters and whitespace
        let filtered = line
            .chars()
            .filter(|c| c.is_alphabetic() || c.is_whitespace())
            .collect::<String>();
        // Split filtered string and assign each number to corresponding parts of node struct
        let node = filtered.split_whitespace().collect::<Vec<&str>>();
        nodes.push(Node {
            start: node[0].to_string(),
            left: node[1].to_string(),
            right: node[2].to_string(),
        });
    }

    // Steps always starts at "AAA"; find corresponding node in "tree"
    let mut current_i = nodes.iter().position(|n| n.start == "AAA").unwrap();
    let mut current_node = &nodes[current_i];

    let mut iterations = 0;

    // Repeat until we reach the end goal
    while current_node.start != "ZZZ" {
        print!("ITERATION {}: ", iterations + 1);
        // Loop through instruction list and follow each one
        for i in &instructions {
            let new_start = match *i {
                'L' => current_node.left.as_str(),
                'R' => current_node.right.as_str(),
                _ => current_node.start.as_str(),
            };

            // Set new current to new node
            current_i = nodes.iter().position(|n| n.start == new_start).unwrap();
            current_node = &nodes[current_i];

            print!("{} ", current_node.start)
        }
        println!("");
        iterations += 1;
    }

    println!("STEPS: {}", iterations * instructions.len());
}
