use std::{collections::VecDeque, fs};

fn are_all_unique_in(buff: &VecDeque<char>) -> bool {
    for i in 0..buff.len() {
        for j in 0..buff.len() {
            if j != i && buff[i] == buff[j] {
                return false;
            }
        }
    }
    return true;
}

fn main() {
    const BUFF_SIZE: usize = 14 - 1;
    
    let mut analyse: VecDeque<char> = VecDeque::with_capacity(BUFF_SIZE);
    let mut cnt = BUFF_SIZE;

    let line = fs::read_to_string("../input.txt").unwrap();

    for c in line.chars() {
        if analyse.len() < BUFF_SIZE {
            analyse.push_front(c);
        } else {
            cnt += 1;
            if are_all_unique_in(&analyse) && !analyse.contains(&c) {
                println!("{cnt}");
                break;
            }

            analyse.pop_back();
            analyse.push_front(c);
        }
    }
}
