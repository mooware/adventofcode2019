use std::io;
use std::io::prelude::*;
use std::ops::AddAssign;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

fn main() {
    // NOTE: this solution is restricted to two wires
    // in order to simplify the implementation and because
    // the task description doesn't ask for more wires
    let mut wires = HashSet::new();
    let mut nearest_distance = isize::max_value();
    let mut nearest_crossed = Coordinate { x: 0, y: 0 };
    let mut first_wire = true;

    for line in io::stdin().lock().lines() {
        let mut pos = Coordinate { x: 0, y: 0 };

        for cmd in line.unwrap_or_default().split(',') {
            let direction = cmd.chars().nth(0).unwrap();
            let length = isize::from_str_radix(&cmd[1..], 10).unwrap();
            let step = match direction {
                'R' => Coordinate { x: 1, y: 0 },
                'L' => Coordinate { x: -1, y: 0 },
                'U' => Coordinate { x: 0, y: 1 },
                'D' => Coordinate { x: 0, y: -1 },
                _ => panic!()
            };

            for _ in 0..length {
                pos += step;
                if first_wire {
                    wires.insert(pos);
                } else if wires.contains(&pos) {
                    let distance = pos.x.abs() + pos.y.abs();
                    if distance < nearest_distance {
                        nearest_distance = distance;
                        nearest_crossed = pos;
                    }
                }
            }
        }

        first_wire = false;
    }
    println!("nearest cross: {:?}, distance: {}", nearest_crossed, nearest_distance);
}
