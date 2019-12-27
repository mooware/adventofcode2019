use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::num::ParseIntError;
use std::str::FromStr;

use permutohedron::LexicalPermutation;

const OPCODE_ADD: isize = 1;
const OPCODE_MULT: isize = 2;
const OPCODE_INPUT: isize = 3;
const OPCODE_OUTPUT: isize = 4;
const OPCODE_JUMP_IF_TRUE: isize = 5;
const OPCODE_JUMP_IF_FALSE: isize = 6;
const OPCODE_LESS_THAN: isize = 7;
const OPCODE_EQUALS: isize = 8;
const OPCODE_HALT: isize = 99;

const PMODE_POSITION: isize = 0;
const PMODE_IMMEDIATE: isize = 1;

#[derive(Clone)]
struct IntCodeProgram {
    code: Vec<isize>,
    ip: usize, // instruction pointer
    input: Vec<isize>,
    output: Vec<isize>
}

impl IntCodeProgram {
    fn execute(&mut self) {
        loop {
            let opcode = self.code[self.ip] % 100;
            match opcode {
                OPCODE_ADD => {
                    let p1 = self.read_param(1);
                    let p2 = self.read_param(2);
                    let out = self.code[self.ip + 3];
                    self.code[out as usize] = p1 + p2;
                    self.ip += 4;
                }
                OPCODE_MULT => {
                    let p1 = self.read_param(1);
                    let p2 = self.read_param(2);
                    let out = self.code[self.ip + 3];
                    self.code[out as usize] = p1 * p2;
                    self.ip += 4;
                }
                OPCODE_INPUT => {
                    let out = self.code[self.ip + 1];
                    let input = self.input.remove(0);
                    self.code[out as usize] = input;
                    self.ip += 2;
                }
                OPCODE_OUTPUT => {
                    let p1 = self.read_param(1);
                    self.output.push(p1);
                    self.ip += 2;
                }
                OPCODE_JUMP_IF_TRUE => {
                    let condition = self.read_param(1);
                    let target = self.read_param(2);
                    if condition != 0 {
                        self.ip = target as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                OPCODE_JUMP_IF_FALSE => {
                    let condition = self.read_param(1);
                    let target = self.read_param(2);
                    if condition == 0 {
                        self.ip = target as usize;
                    } else {
                        self.ip += 3;
                    }
                }
                OPCODE_LESS_THAN => {
                    let p1 = self.read_param(1);
                    let p2 = self.read_param(2);
                    let out = self.code[self.ip + 3];
                    self.code[out as usize] = (p1 < p2) as isize;
                    self.ip += 4;
                }
                OPCODE_EQUALS => {
                    let p1 = self.read_param(1);
                    let p2 = self.read_param(2);
                    let out = self.code[self.ip + 3];
                    self.code[out as usize] = (p1 == p2) as isize;
                    self.ip += 4;
                }
                OPCODE_HALT => return,
                _ => panic!(),
            }
        }
    }

    fn read_param(&self, param_num: u32) -> isize {
        let opcode = self.code[self.ip];
        let mode = (opcode / 10isize.pow(param_num + 1)) % 10;
        let param = self.code[self.ip + param_num as usize];
        match mode {
            PMODE_POSITION => self.code[param as usize],
            PMODE_IMMEDIATE => param,
            _ => panic!(),
        }
    }
}

impl FromStr for IntCodeProgram {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iter = s.split(',');
        let mut code = Vec::new();
        for s in iter {
            let i = s.parse::<isize>()?;
            code.push(i);
        }
        Ok(IntCodeProgram { code: code, ip: 0, input: Vec::new(), output: Vec::new() })
    }
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    if let Some(filename) = env::args().nth(1) {
        let mut file = File::open(&filename)?;
        file.read_to_string(&mut line)?;
    } else {
        io::stdin().read_line(&mut line)?;
    }

    let program = line
        .trim()
        .parse::<IntCodeProgram>()
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut phases = [0, 1, 2, 3, 4];
    let mut max_output = isize::min_value();

    loop {
        let mut prev_output = None;
        for &phase in phases.iter() {
            let mut tmp_program = program.clone();
            tmp_program.input.push(phase);
            tmp_program.input.push(prev_output.unwrap_or_default());
            tmp_program.execute();
            let output = tmp_program.output[0];
            prev_output = Some(output);
        }
        if let Some(output) = prev_output {
            if max_output < output {
                max_output = output;
            }
        }
        if !phases.next_permutation() {
            break;
        }
    }

    println!("{}", max_output);
    Ok(())
}
