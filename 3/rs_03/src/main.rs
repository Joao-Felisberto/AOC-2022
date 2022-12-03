use std::fs::File;
use std::io::{BufRead, BufReader};

fn domain_hash(e: usize) -> usize {
    if e >= 65 && e <= 90 { // upper
        e - (65 - 27) - 1
    } else if e <= 122 {    // lower
        e - 96 - 1
    } else {
        panic!("Invalid element!") // error, should never happen
    }
}

fn main() {
    println!("{}", domain_hash('A' as usize));
    println!("{}", domain_hash('Z' as usize));
    println!("{}", domain_hash('a' as usize));
    println!("{}", domain_hash('z' as usize));
    let file = File::open("../input.txt").expect("Cannot read file.");
    let reader = BufReader::new(file);

    let mut score: usize = 0;
    for (_, ln) in reader.lines().enumerate() {
        if let Ok(rucksack) = ln {
            // A -> 65  27
            // Z -> 90  52
            // a -> 97  1
            // z -> 122 26
            let rucksack = rucksack.as_bytes();
            let mut left_hash_set = [false; 52]; //Vec::with_capacity(52);
            let mut right_hash_set = [false; 52];

            let half = rucksack.len() / 2;
            for i in 0..half {
                let left = domain_hash(rucksack[i] as usize);
                let right = domain_hash(rucksack[i + half] as usize);

                left_hash_set[left] = true;
                right_hash_set[right] = true;

                if right_hash_set[left] {
                    score += left + 1;
                    break;
                }
                if left_hash_set[right] {
                    score += right + 1;
                    break;
                }
            }
        }
    }

    println!("{score}");
}
