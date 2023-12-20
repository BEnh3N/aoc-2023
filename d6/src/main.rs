use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let lines = input.lines().collect::<Vec<&str>>();

    let time: u64 = lines[0].chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap();
    let distance_to_beat: u64 = lines[1].chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap();

    println!("{}, {}", time, distance_to_beat);

    let mut count = 0;

    for time_held in 0..time {
        let time_to_move = time - time_held;
        let distance_traveled = time_to_move * time_held;
        if distance_traveled > distance_to_beat {
            count += 1;
        }
    }

    println!("{}", count);
}
