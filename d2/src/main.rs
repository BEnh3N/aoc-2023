use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut sum = 0;

    for line in reader {
        if let Ok(input) = line {
            let input = input.split_once(':').unwrap();
            // let game_id = input.0.split_at(5).1.parse::<usize>().unwrap();

            let mut red_max = 1;
            let mut green_max = 1;
            let mut blue_max = 1;

            input
                .1
                .split([';', ','])
                .map(|x| x.trim().split_once(' ').unwrap())
                .for_each(|(value, color)| {
                    let value = value.parse::<u32>().unwrap();
                    match color {
                        "red" => red_max = red_max.max(value),
                        "green" => green_max = green_max.max(value),
                        "blue" => blue_max = blue_max.max(value),
                        _ => (),
                    }
                });

            let power = red_max * green_max * blue_max;
            sum += power;
        }
    }

    println!("{}", sum);
}
