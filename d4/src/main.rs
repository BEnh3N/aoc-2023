use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut num_cards = vec![];

    let mut cards = vec![];
    for line in input.lines() {
        let nums = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning_nums = nums
            .0
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let current_nums = nums
            .1
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();

        num_cards.push(1);
        cards.push((winning_nums, current_nums));
    }

    for (i, card) in cards.iter().enumerate() {
        let winning_nums = &card.0;
        let current_nums = &card.1;

        for _ in 0..num_cards[i] {
            let mut count = 0;
            for num in current_nums {
                if winning_nums.contains(num) {
                    count += 1;
                }
            }

            for j in 1..=count {
                num_cards[i + j] += 1;
            }
        }
    }

    let mut sum = 0;
    for r in num_cards {
        println!("{}", r);
        sum += r;
    }
    println!("\n{}", sum);
}
