use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BinaryHeap;

fn main() {
    let input = "input.txt";
    let mut elves: BinaryHeap<u32> = BinaryHeap::new();
    elves.push(0);

    let file = File::open(input).expect("Could not read file");
    let reader = BufReader::new(file);

    let mut calories = 0;
    for (_, ln) in reader.lines().enumerate() {
        let ln = ln.expect("Could not read file line");
        
        if ln == "" {
            elves.push(calories);

            calories = 0;
        } else {
            let cal = ln.parse::<u32>().expect(&format!("Could not convert {} to u32", ln));

            calories += cal;
        }
    }

    let top1 = elves.pop().expect("There were no elves");
    let top2 = elves.pop().expect("There was only one elf");
    let top3 = elves.pop().expect("There were only 2 elves");
    let sum = top1 + top2 + top3;
    
    println!("top 3 elves carry {sum} calories");
}
