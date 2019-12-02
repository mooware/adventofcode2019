use std::io;
use std::io::prelude::*;

fn main() {
    let mut totalfuel = 0usize;
    for line in io::stdin().lock().lines() {
        let mass = usize::from_str_radix(&line.unwrap(), 10).unwrap_or_default();
        totalfuel += (mass / 3).saturating_sub(2);
    }
    println!("{}", totalfuel);
}
