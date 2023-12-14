use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;

    input.lines().for_each(|line| {
        let nums = line.split_once(':').unwrap().1.split_once('|').unwrap();
        
        let winning_nums = nums.0.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let current_nums = nums.1.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        let mut points = 0;

        for num in winning_nums {
            if current_nums.contains(&num) {
                if points == 0 {
                    points += 1;
                } else {
                    points *= 2;
                }
            }
        }

        print!("{} ", points);
        sum += points;
    });

    println!("\n{}", sum);
}
