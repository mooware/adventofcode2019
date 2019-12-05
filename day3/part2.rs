use std::io;
use std::io::prelude::*;
use std::ops::AddAssign;
use std::collections::HashMap;

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
    let mut wires = HashMap::new();
    let mut nearest_distance = usize::max_value();
    let mut nearest_crossed = Coordinate { x: 0, y: 0 };
    let mut first_wire = true;

    for line in io::stdin().lock().lines() {
        let mut pos = Coordinate { x: 0, y: 0 };
        let mut stepcount = 0;

        for cmd in line.unwrap_or_default().split(',') {
            let direction = cmd.chars().nth(0).unwrap();
            let length = usize::from_str_radix(&cmd[1..], 10).unwrap();
            let step = match direction {
                'R' => Coordinate { x: 1, y: 0 },
                'L' => Coordinate { x: -1, y: 0 },
                'U' => Coordinate { x: 0, y: 1 },
                'D' => Coordinate { x: 0, y: -1 },
                _ => panic!()
            };

            for _ in 0..length {
                pos += step;
                stepcount += 1;

                if first_wire {
                    // only insert if not contained
                    wires.entry(pos).or_insert(stepcount);
                } else if let Some(&crossed_stepcount) = wires.get(&pos) {
                    let distance = stepcount + crossed_stepcount;
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
