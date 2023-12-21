use std::{cmp::Ordering, fs};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: u32,
    hand_type: HandType,
}

const LABELS: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut hands = vec![];

    for line in input.lines() {
        let line = line.split_once(" ").unwrap();

        let hand = line.0;
        let bid = line.1.parse::<u32>().unwrap();

        let mut cards: Vec<char> = vec![];
        let mut nums: Vec<u32> = vec![];

        for card in hand.chars() {
            // Check to see if list already contains card
            match cards.iter().position(|c| c == &card) {
                // If so, increment existing card number
                Some(i) => nums[i] += 1,
                // If not, add card to list
                None => {
                    cards.push(card);
                    nums.push(1);
                }
            }
        }

        let has_j = cards.contains(&'J');
        let num_j = match cards.iter().position(|c| c == &'J') {
            Some(i) => nums[i],
            None => 0
        };

        // Categorize hand by number of unique cards && number of cards
        // (there may be a more efficient way; i don't know it)
        let hand_type = match nums.len() {
            1 => HandType::FiveKind,
            2 => {
                if has_j {
                    // Four of a kinds and full houses will both turn into Five of a kinds if a J is present
                    HandType::FiveKind
                } else {
                    if nums.contains(&4) {
                        HandType::FourKind
                    } else {
                        HandType::FullHouse
                    }
                }
            }
            3 => {
                if nums.contains(&3) {
                    if has_j {
                        // Three of a kinds can turn to Four of a kind if J is present
                        HandType::FourKind
                    } else {
                        HandType::ThreeKind
                    }
                } else {
                    if has_j {
                        if num_j == 2 {
                            HandType::FourKind
                        } else {
                            HandType::FullHouse
                        }
                    } else {
                        HandType::TwoPair
                    }
                }
            }
            4 => {
                if has_j {
                    HandType::ThreeKind
                } else {
                    HandType::OnePair
                }
            }
            _ => {
                if has_j {
                    HandType::OnePair
                } else {
                    HandType::HighCard
                }
            }
        };

        hands.push(Hand {
            cards: hand.chars().collect(),
            bid,
            hand_type,
        });
    }

    // Handle the ordering of the hands by the stupid little rules
    hands.sort_by(|a, b| {
        if a.hand_type < b.hand_type {
            Ordering::Less
        } else if a.hand_type > b.hand_type {
            Ordering::Greater
        } else {
            for i in 0..a.cards.len() {
                let a_card = a.cards[i];
                let b_card = b.cards[i];

                let a_pos = LABELS.iter().position(|c| c == &a_card).unwrap();
                let b_pos = LABELS.iter().position(|c| c == &b_card).unwrap();

                if a_pos < b_pos {
                    return Ordering::Less;
                } else if a_pos > b_pos {
                    return Ordering::Greater;
                }
            }

            Ordering::Equal
        }
    });

    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        println!("{}: {:?}", i + 1, hand);

        winnings += hand.bid * (i as u32 + 1);
    }

    println!("Winnings: {}", winnings);
}
