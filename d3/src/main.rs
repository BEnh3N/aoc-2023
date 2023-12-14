use std::fs::{self};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let arr = input
        .split('\n')
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // Quite possibly the worst way to make a vector of the same size with all 0s
    let mut counted = Vec::from_iter(
        std::iter::repeat(Vec::from_iter(std::iter::repeat(false).take(arr[0].len())))
            .take(arr.len()),
    );

    let mut sum = 0;

    // Loop through every character
    for (row, line) in arr.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            // Test for any special symbols (only * for gears)
            if *c == '*' {
                print!("({}, {}) {} ", col, row, c);
                let mut nums = vec![];

                // Find characters surrounding each symbol
                for r in 0..=2 {
                    for c in 0..=2 {
                        let x = col + c - 1;
                        let y = row + r - 1;

                        // If the surrounding character is a number and hasn't already been counted, scan for rest of the number
                        if arr[y][x].is_numeric() && !counted[y][x] {
                            // Go backwards to find start of number
                            let mut i = x;
                            while i != 0 && arr[y][i - 1].is_numeric() {
                                i -= 1;
                            }

                            let mut num = 0;
                            // Loop forwards, accumulating each value into number
                            while i < arr[0].len() && arr[y][i].is_numeric() {
                                counted[y][i] = true;
                                num *= 10;
                                num += arr[y][i].to_digit(10).unwrap();
                                i += 1;
                            }

                            nums.push(num);
                            print!("{} ", num);
                        }
                    }
                }

                if nums.len() == 2 {
                    let ratio = nums[0] * nums[1];
                    sum += ratio;
                    print!("({})", ratio);
                }

                println!("");
            }
        }
    }

    println!("{}", sum);
}
