use std::env;

fn check_number(num : u32) -> bool {
    if num < 100_000 || num > 999_999 {
        return false;
    }
    let mut equal_digits = 1;
    let mut same_pair = false;
    let mut prev_digit = num / 100_000;
    let mut divisor = 10_000;
    while divisor > 0 {
        let digit = (num / divisor) % 10;
        if digit < prev_digit {
            return false;
        }
        if digit == prev_digit {
            equal_digits += 1;
        } else {
            if equal_digits == 2 {
                same_pair = true;
            }
            equal_digits = 1;
        }
        prev_digit = digit;
        divisor /= 10;
    }
    if equal_digits == 2 {
        same_pair = true;
    }
    return same_pair;
}

fn main() {
    let minstr = env::args().nth(1).unwrap();
    let maxstr = env::args().nth(2).unwrap();
    let min = u32::from_str_radix(&minstr, 10).unwrap();
    let max = u32::from_str_radix(&maxstr, 10).unwrap();
    let mut count = 0;
    for i in min..=max {
        if check_number(i) {
            println!("found non-match: {}", i);
            count += 1;
        }
    }
    println!("{} matches", count);
}
