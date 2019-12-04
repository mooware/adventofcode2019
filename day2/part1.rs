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
        let mut code : Vec<usize> = line.split(',').map(|s| {
            return usize::from_str_radix(&s, 10).unwrap_or_default();
        }).collect();
        // before running the program, replace position 1 with the value 12 and replace position 2 with the value 2
        code[1] = 12;
        code[2] = 2;
        exec_intcode(&mut code);
        println!("{}", code[0]);
    }
}
