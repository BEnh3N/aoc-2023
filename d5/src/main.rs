use std::fs;

#[derive(Debug)]
struct Mapping {
    dest_start: u64,
    source_start: u64,
    range: u64,
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    // Take first line of input and parse for integers (seeds)
    let seeds = lines[0]
        .split_whitespace()
        .filter_map(|x| x.parse::<u64>().ok())
        .collect::<Vec<u64>>();

    let mut mappings: Vec<Vec<Mapping>> = vec![];

    for line in lines {
        // Empty lines mean new group must be added to master array
        if line.is_empty() {
            mappings.push(vec![]);
        } else if line.starts_with(char::is_numeric) {
            // All lines starting with numbers are mappings
            let mut nums = line.split_whitespace().map(|x| x.parse::<u64>().unwrap());
            let mapping = Mapping {
                dest_start: nums.next().unwrap(),
                source_start: nums.next().unwrap(),
                range: nums.next().unwrap(),
            };
            // Push mapping to end of last group
            let len = mappings.len();
            mappings[len - 1].push(mapping);
        }
    }

    let mut lowest_location = u64::MAX;

    for seed in seeds {
        println!("SEED: {}", seed);
        let mut source = seed;

        for map in &mappings {
            for mapping in map {
                if source >= mapping.source_start && source < mapping.source_start + mapping.range {
                    print!("{}", source);
                    source = mapping.dest_start + (source - mapping.source_start);
                    println!(" -> {}", source);
                    break;
                }
            }
        }

        println!("----");

        let location = source;
        if location < lowest_location {
            lowest_location = location;
        }
    }

    println!("LOWEST LOCATION: {}", lowest_location);

    // input.lines();

    // let mut seeds = vec![];
    // let mut mappings = vec![];

    // let mut mapping_index = 0;
    // let mut start_indices = vec![];

    // Loop through all lines in input
    // for (i, line) in input.lines().enumerate() {
    //     // First line always contains seed info
    //     if i == 0 {
    //         // Parse first line for just seed nums
    //         seeds = line
    //             .split_once(':')
    //             .unwrap()
    //             .1
    //             .split_whitespace()
    //             .map(|x| x.parse::<u64>().unwrap())
    //             .collect::<Vec<u64>>();
    //     } else {
    //         // All mapping lines start with a number; filter that way
    //         if line.starts_with(char::is_numeric) {
    //             // Take all nums from line and put them into a vector
    //             let nums = line
    //                 .split_whitespace()
    //                 .map(|x| x.parse::<u64>().unwrap())
    //                 .collect::<Vec<u64>>();
    //             // Each number corresponds to a different part of the mapping
    //             let map = Mapping {
    //                 source_start: nums[1],
    //                 dest_start: nums[0],
    //                 range: nums[2],
    //             };

    //             mappings.push(map);
    //             mapping_index += 1;
    //         } else if line.starts_with(char::is_alphabetic) {
    //             // Use labeled lines to find each starting index
    //             start_indices.push(mapping_index);
    //         }
    //     }
    // }

    // start_indices.push(mapping_index);

    // let mut fertilizers = vec![];

    // Test each seed, seeing the path it takes down the mappings
    // for seed in 0..1 {
    //     let mut source = seeds[0]; // Initial source is the seed value; is later changed

    //     for j in 0..start_indices.len() - 1 {
    //         let start_i = start_indices[j];
    //         let end_i = start_indices[j + 1];

    //         let mut found = false;
    //         let mut i = start_i;

    //         while i < end_i && !found {
    //             let mapping = &mappings[i];
    //             if source >= mapping.source_start && source < mapping.source_start + mapping.range {
    //                 source = source - mapping.source_start + mapping.dest_start;
    //                 println!("FOUND! {:?}", mapping);
    //                 found = true;
    //             }
    //             i += 1;
    //         }

    //         // for i in start_i..end_i {
    //             let mapping = &mappings[i];
    //             println!("SOURCE: {}", source);
    //             if source >= mapping.source_start && source < mapping.source_start + mapping.range {
    //                 source = source - mapping.source_start + mapping.dest_start;
    //                 println!("{} Found in range: {:?}", source, mapping);
    //                 break;
    //             } else {
    //                 println!("Not found... Trying next!");
    //             }
    //         // }
    //         println!("GROUP DONE");
    //     }

    //     println!("SEED DONE");
    // }
}
