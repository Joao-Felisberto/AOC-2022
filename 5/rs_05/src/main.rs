use regex::Regex;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
# Resources

- https://stackoverflow.com/questions/28079061/how-do-i-convert-a-string-to-an-integer-in-rust
*/

enum Mode {
    Header,
    Data,
}

fn main() {
    let file = File::open("../input.txt").expect("Cannot read file.");
    let reader = BufReader::new(file);

    let mut mode = Mode::Header;
    let separator_regex = Regex::new(r"^ [0-9] ").expect("Couldn't compile regex.");
    let number_regex = Regex::new(r"[0-9]+").expect("Couldn't compile regex.");
    let mut header: Vec<Vec<char>> = Vec::new();

    for (_, ln) in reader.lines().enumerate() {
        if let Ok(line) = ln {
            match mode {
                Mode::Header => {
                    if separator_regex.is_match(&line) {
                        mode = Mode::Data;
                        for v in &mut header {
                            v.reverse();
                        }
                        for ln in &header {
                            for c in ln {
                                print!("{c}");
                            }
                            print!("\n");
                        }
                        continue;
                    }
                    if header.len() == 0 {
                        for _ in 0..(line.len() + 1) / 4 {
                            header.push(Vec::new());
                        }
                    }
                    let mut i = 0;
                    for c in line.chars().skip(1).step_by(4) {
                        print!("{c} ");
                        if c != ' ' {
                            header[i].push(c);
                        }
                        i += 1;
                    }
                    print!("\n");
                }
                Mode::Data => {
                    if line.len() == 0 {
                        continue;
                    }

                    if !number_regex.is_match(&line) {
                        panic!("Malformed line {} {}", line.len(), line);
                    }

                    let mut e = number_regex.find_iter(&line).take(3);
                    if let (Ok(amount), Ok(from), Ok(to)) = (
                        e.next().unwrap().as_str().parse::<usize>(),
                        e.next().unwrap().as_str().parse::<usize>(),
                        e.next().unwrap().as_str().parse::<usize>(),
                    ) {
                        let mut queue: VecDeque<char> = VecDeque::new();
                        for _ in 0..amount {
                            let c = header[from - 1].pop().unwrap();
                            queue.push_back(c);
                        }
                        for _ in 0..amount {
                            let c = queue.pop_back().unwrap();
                            header[to - 1].push(c);
                        }
                    }
                }
            }
        }
    }

    println!("============");
    for ln in &header {
        for c in ln {
            print!("{c}");
        }
        print!("\n");
    }

    println!("============");
    for ln in &header {
        print!("{}", ln[ln.len()-1]);
    }
    print!("\n");
}
