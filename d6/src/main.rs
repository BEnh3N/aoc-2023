use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    let times: Vec<u32> = lines[0].split_whitespace().filter_map(|x| x.parse().ok()).collect();
    let distances: Vec<u32> = lines[1].split_whitespace().filter_map(|x| x.parse().ok()).collect();

    let mut num_ways = 1;

    for i in 0..times.len() {
        let time = times[i];
        let distance_to_beat = distances[i];

        let mut count = 0;

        for time_held in 0..time {
            let time_to_move = time - time_held;
            let distance_traveled = time_to_move * time_held;
            if distance_traveled > distance_to_beat {
                count += 1;
            }
        }

        num_ways *= count;
    }

    println!("{}", num_ways);
}
