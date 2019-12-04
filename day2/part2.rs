use std::io;
use std::io::prelude::*;

const OPCODE_ADD : usize = 1;
const OPCODE_MULT : usize = 2;
const OPCODE_HALT : usize = 99;
const EXPECTED_RESULT : usize = 19690720;

fn exec_intcode(code : &mut [usize]) {
    let mut pos = 0;
    loop {
        match code[pos] {
            OPCODE_ADD => {
                let in1 = code[pos + 1];
                let in2 = code[pos + 2];
                let out = code[pos + 3];
                code[out] = code[in1] + code[in2];
                pos += 4;
            },
            OPCODE_MULT => {
                let in1 = code[pos + 1];
                let in2 = code[pos + 2];
                let out = code[pos + 3];
                code[out] = code[in1] * code[in2];
                pos += 4;
            },
            OPCODE_HALT => return,
            _ => panic!()
        }
    }
}

fn main() {
    let mut line = String::new();
    if let Ok(_) = io::stdin().lock().read_line(&mut line) {
        let code : Vec<usize> = line.split(',').map(|s| {
            return usize::from_str_radix(&s, 10).unwrap_or_default();
        }).collect();

        // don't use clone(), allocate once and copy
        let mut tmpcode = vec![0usize; code.len()];
        for noun in 0..100 {
            for verb in 0..100 {
                tmpcode.copy_from_slice(&code);
                tmpcode[1] = noun;
                tmpcode[2] = verb;
                exec_intcode(&mut tmpcode);
                let result = tmpcode[0];
                if result == EXPECTED_RESULT {
                    println!("100 * {} + {} = {}", noun, verb, (100 * noun + verb));
                    return;
                }
            }
        }
    }
}
