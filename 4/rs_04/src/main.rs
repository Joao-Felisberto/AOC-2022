use std::fs::File;
use std::io::{BufRead, BufReader};

/*
# Resources

- https://stackoverflow.com/questions/28079061/how-do-i-convert-a-string-to-an-integer-in-rust
*/

fn main() {
    let file = File::open("../input.txt").expect("Cannot read file.");
    let reader = BufReader::new(file);

    let mut fully_contained = 0;
    let mut intercept = 0;
    for (_, ln) in reader.lines().enumerate() {
        if let Ok(range_pair) = ln {
            let mut pair = range_pair.split(",");
            let mut left = pair
                .next()
                .expect("Malformed input: not enough pairs in line")
                .split("-");
            let mut right = pair
                .next()
                .expect("Malformed input: not enough pairs in line")
                .split("-");

            let l_start = left
                .next()
                .expect("Malformed input, range separation")
                .parse::<u32>()
                .expect("Malformed input, range must only contain numbers");
            let l_end = left
                .next()
                .expect("Malformed input, range separation")
                .parse::<u32>()
                .expect("Malformed input, range must only contain numbers");
            let r_start = right
                .next()
                .expect("Malformed input, range separation")
                .parse::<u32>()
                .expect("Malformed input, range must only contain numbers");
            let r_end = right
                .next()
                .expect("Malformed input, range separation")
                .parse::<u32>()
                .expect("Malformed input, range must only contain numbers");

            // PART 1
            if (l_start >= r_start && l_end <= r_end) || (r_start >= l_start && r_end <= l_end) {
                fully_contained += 1;
            }

            // PART 2
            if (l_end >= r_start && l_start <= r_end) || (r_end >= l_start && r_start <= l_end) {
                intercept += 1;
            }
        }
    }

    println!("{fully_contained} pairs are fully contained, {intercept} intercept");
}
