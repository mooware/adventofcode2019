use std::io;
use std::str::FromStr;
use std::num::ParseIntError;

const OPCODE_ADD : isize = 1;
const OPCODE_MULT : isize = 2;
const OPCODE_INPUT : isize = 3;
const OPCODE_OUTPUT : isize = 4;
const OPCODE_JUMP_IF_TRUE : isize = 5;
const OPCODE_JUMP_IF_FALSE : isize = 6;
const OPCODE_LESS_THAN : isize = 7;
const OPCODE_EQUALS : isize = 8;
const OPCODE_HALT : isize = 99;

const PMODE_POSITION : isize = 0;
const PMODE_IMMEDIATE : isize = 1;

struct IntCodeProgram {
    code : Vec<isize>,
    ip : usize // instruction pointer
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
                },
                OPCODE_MULT => {
                    let p1 = self.read_param(1);
                    let p2 = self.read_param(2);
                    let out = self.code[self.ip + 3];
                    self.code[out as usize] = p1 * p2;
                    self.ip += 4;
                },
                OPCODE_INPUT => {
                    let out = self.code[self.ip + 1];
                    let input = self.get_input();
                    self.code[out as usize] = input;
                    self.ip += 2;
                },
                OPCODE_OUTPUT => {
                    let p1 = self.read_param(1);
                    println!("output: {}", p1);
                    self.ip += 2;
                },
                OPCODE_JUMP_IF_TRUE => {
                    let condition = self.read_param(1);
                    let target = self.read_param(2);
                    if condition != 0 {
                        self.ip = target as usize;
                    } else {
                        self.ip += 3;
                    }
                },
                OPCODE_JUMP_IF_FALSE => {
                    let condition = self.read_param(1);
                    let target = self.read_param(2);
                    if condition == 0 {
                        self.ip = target as usize;
                    } else {
                        self.ip += 3;
                    }
                },
                OPCODE_LESS_THAN => {
                    let p1 = self.read_param(1);
                    let p2 = self.read_param(2);
                    let out = self.code[self.ip + 3];
                    self.code[out as usize] = (p1 < p2) as isize;
                    self.ip += 4;
                },
                OPCODE_EQUALS => {
                    let p1 = self.read_param(1);
                    let p2 = self.read_param(2);
                    let out = self.code[self.ip + 3];
                    self.code[out as usize] = (p1 == p2) as isize;
                    self.ip += 4;    
                },
                OPCODE_HALT => return,
                _ => panic!()
            }
        }
    }

    fn read_param(&self, param_num : u32) -> isize {
        let opcode = self.code[self.ip];
        let mode = (opcode / 10isize.pow(param_num + 1)) % 10;
        let param = self.code[self.ip + param_num as usize];
        match mode {
            PMODE_POSITION => self.code[param as usize],
            PMODE_IMMEDIATE => param,
            _ => panic!()
        }
    }

    fn get_input(&self) -> isize {
        print!("input: ");
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        line.trim().parse().unwrap_or_default()
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
        Ok(IntCodeProgram { code: code, ip : 0 })
    }
}

fn main() -> io::Result<()> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    let mut program = line.trim().parse::<IntCodeProgram>().map_err(|e|
        io::Error::new(io::ErrorKind::InvalidData, e)
    )?;
    program.execute();
    Ok(())
}
