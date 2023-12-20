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

    // Loop through every group of two in the parsed seed numbers (b/c part 2 is a b***)
    for seed_pair in seeds.chunks_exact(2) {
        let start = seed_pair[0];
        let end = start + seed_pair[1];

        for seed in start..end {
            // Source value always starts as the seed; mapped to each eventual destination
            let mut source = seed;

            // Loop through all mapping groups
            for map in &mappings {
                // Loop through each mapping in the mapping group, mapping as required
                for mapping in map {
                    // If the source falls within the mapping's source range, map it to the destination and break out
                    if source >= mapping.source_start
                        && source < mapping.source_start + mapping.range
                    {
                        source = mapping.dest_start + (source - mapping.source_start);
                        break;
                    }
                }
            }

            // Check if the location is the lowest so far
            let location = source;
            if location < lowest_location {
                lowest_location = location;
            }
        }
    }

    println!("LOWEST LOCATION: {}", lowest_location);
}
