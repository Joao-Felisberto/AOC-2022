use std::fs::File;
use std::io::{BufRead, BufReader};

fn domain_hash(e: usize) -> usize {
    if e >= 65 && e <= 90 {
        // upper
        e - (65 - 27) - 1
    } else if e <= 122 {
        // lower
        e - 96 - 1
    } else {
        panic!("Invalid element!") // error, should never happen
    }
}

fn main() {
    /*
    // DEBUG 2510
    println!("{}", domain_hash('A' as usize));
    println!("{}", domain_hash('Z' as usize));
    println!("{}", domain_hash('a' as usize));
    println!("{}", domain_hash('z' as usize));
    */

    let file = File::open("../input.txt").expect("Cannot read file.");
    let reader = BufReader::new(file);

    let mut score: usize = 0;

    let mut badge_score: usize = 0;
    let mut badge_hash_set: [u8; 52] = [0; 52];
    let mut mod_3_cnt = 0;
    for (_, ln) in reader.lines().enumerate() {
        if let Ok(rucksack) = ln {
            // A -> 65  27
            // Z -> 90  52
            // a -> 97  1
            // z -> 122 26
            let rucksack = rucksack.as_bytes();
            let mut left_hash_set = [false; 52]; //Vec::with_capacity(52);
            let mut right_hash_set = [false; 52];

            let mut j = 0;
            let half = rucksack.len() / 2;
            for i in 0..half {
                let left = domain_hash(rucksack[i] as usize);
                let right = domain_hash(rucksack[i + half] as usize);

                left_hash_set[left] = true;
                right_hash_set[right] = true;

                if right_hash_set[left] {
                    score += left + 1;
                    j = i;
                    break;
                }
                if left_hash_set[right] {
                    score += right + 1;
                    j = i;
                    break;
                }
            }

            for i in j..half {
                let left = domain_hash(rucksack[i] as usize);
                let right = domain_hash(rucksack[i + half] as usize);

                left_hash_set[left] = true;
                right_hash_set[right] = true;
            }

            for i in 0..badge_hash_set.len() {
                if left_hash_set[i] || right_hash_set[i] {
                    badge_hash_set[i] += 1;
                }
            }

            if mod_3_cnt == 2 {
                for i in 0..badge_hash_set.len() {
                    if badge_hash_set[i] == 3 {
                        badge_score += i + 1;
                        break;
                    }
                }
                badge_hash_set = [0; 52];
            }
            mod_3_cnt = (mod_3_cnt + 1) % 3;
        }
    }

    println!("Score of misplaced items: {score}");
    println!("Score of badges: {badge_score}");
}
