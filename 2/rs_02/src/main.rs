use std::fs::File;
use std::io::{BufRead, BufReader};

use game::Choice;

mod game;

fn main() {
    let file = File::open("../input.txt").expect("Cannot read file.");
    let reader = BufReader::new(file);

    let mut score: usize = 0;
    for (_, ln) in reader.lines().enumerate() {
        if let Ok(round) = ln {
            let round = round.as_bytes();
            /* Part 1
            let adv = Choice::new(round[0] as char).expect("Malformed Option");
            let mine = Choice::new(round[2] as char).expect("Malformed Option");

            score += mine.play_against(&adv);
            */
            let adv = Choice::new(round[0] as char).expect("Malformed Option");
            let outcome = round[2] as char;

            let mine = Choice::to_get_result(&adv, outcome).expect("Malformed input");
            score += mine.play_against(&adv);
        }
    }

    println!("I got {score} points!");
}
