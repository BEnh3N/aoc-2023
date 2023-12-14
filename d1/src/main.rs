use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file).lines();

    let mut sum = 0;

    let string_nums = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in reader {
        if let Ok(input) = line {
            let mut matches = vec![];
            input
                .match_indices(char::is_numeric)
                .for_each(|(i, c)| matches.push((i, c.parse::<usize>().unwrap())));
            for (i, num) in string_nums.iter().enumerate() {
                input
                    .match_indices(num)
                    .for_each(|x| matches.push((x.0, i + 1)))
            }

            matches.sort_by_key(|x| x.0);
            let nums = matches.iter().map(|x| x.1).collect::<Vec<usize>>();


            let d1 = nums[0];
            let d2 = nums[nums.len()-1];
            let num: usize = d1 * 10 + d2;

            println!("{}, {}: {}", d1, d2, num);

            sum += num;

        }
    }

    println!("{}", sum);
}
