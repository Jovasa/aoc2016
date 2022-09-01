use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use crate::OpCode::{COPY, DEC, INC, JUMP, OUT, TOGGLE};
use crate::day12::{parse_register, parse_value};

mod day12;

enum OpCode {
    COPY,
    JUMP,
    INC,
    DEC,
    TOGGLE,
    OUT,
}


struct Op {
    opcode: OpCode,
    first_reg: Option<usize>,
    first_val: Option<i32>,
    second_reg: Option<usize>,
    second_val: Option<i32>,
}


fn main() {
    let reader = BufReader::new(File::open("data/day25.txt").unwrap());
    let mut program = Vec::new();
    let mut register = [0; 4];

    register[0] = 12;

    for line in reader.lines() {
        let line = line.unwrap();
        let parts = line.split(" ").collect_vec();
        program.push(
            match parts[0] {
                "cpy" => Op {
                    opcode: COPY,
                    first_reg: parse_register(parts[1]),
                    first_val: parse_value(parts[1]),
                    second_reg: parse_register(parts[2]),
                    second_val: parse_value(parts[2]),
                },
                "jnz" => Op {
                    opcode: JUMP,
                    first_reg: parse_register(parts[1]),
                    first_val: parse_value(parts[1]),
                    second_reg: parse_register(parts[2]),
                    second_val: parse_value(parts[2]),
                },
                "inc" => Op {
                    opcode: INC,
                    first_reg: parse_register(parts[1]),
                    first_val: parse_value(parts[1]),
                    second_reg: None,
                    second_val: None,
                },
                "dec" => Op {
                    opcode: DEC,
                    first_reg: parse_register(parts[1]),
                    first_val: parse_value(parts[1]),
                    second_reg: None,
                    second_val: None,
                },
                "out" => Op {
                    opcode: OUT,
                    first_reg: parse_register(parts[1]),
                    first_val: None,
                    second_reg: None,
                    second_val: None,
                },
                "tgl" => {
                    Op {
                        opcode: TOGGLE,
                        first_reg: parse_register(parts[1]),
                        first_val: None,
                        second_reg: None,
                        second_val: None,
                    }
                }
                _ => unreachable!()
            }
        )
    }

    for start_code in 0.. {
        register[0] = start_code;
        register[1] = 0;
        register[2] = 0;
        register[3] = 0;

        let mut pc: usize = 0;

        let mut expected_out = 0;
        let mut total_found = 0;

        let mut states = HashSet::new();

        let mut tries = 0;

        while (pc as usize) < program.len() {
            let state = (register[0], register[1], register[2], register[3], pc);
            if states.contains(&state) { break };
            states.insert(state);
            let temp = &program[pc as usize];
            tries += 1;
            match temp.opcode {
                COPY => {
                    let value = match temp.first_val {
                        Some(i) => i,
                        None => register[temp.first_reg.unwrap()]
                    };
                    if temp.second_val.is_some() { continue; }
                    register[temp.second_reg.unwrap()] = value;
                    pc += 1;
                }
                JUMP => {
                    let value = match temp.first_val {
                        Some(i) => i,
                        None => register[temp.first_reg.unwrap()]
                    };
                    if value != 0 {
                        let jmp = match temp.second_val {
                            Some(i) => i,
                            None => register[temp.second_reg.unwrap()]
                        };
                        if jmp == 0 {
                            pc = 1000;
                        }
                        pc = (pc as i64 + jmp as i64) as usize;
                    } else {
                        pc += 1;
                    }
                }
                INC => {
                    if temp.first_val.is_some() { continue; };
                    register[temp.first_reg.unwrap()] += 1;
                    pc += 1;
                }
                DEC => {
                    if temp.first_val.is_some() { continue; };
                    register[temp.first_reg.unwrap()] -= 1;
                    pc += 1;
                }
                OUT => {
                    if register[temp.first_reg.unwrap()] == expected_out {
                        expected_out = if expected_out == 0 { 1 } else { 0 };
                        total_found += 1;
                        if total_found > 10 {
                            println!("FOUND {start_code}");
                            return;
                        }
                        pc += 1;
                    }
                    else {
                        pc = 1000;
                    }
                }
                TOGGLE => {
                    let value = pc + register[temp.first_reg.unwrap()] as usize;
                    if value >= program.len() {
                        pc += 1;
                        continue;
                    }
                    let op = &program[value].opcode;
                    match op {
                        INC => program[value].opcode = DEC,
                        DEC => program[value].opcode = INC,
                        JUMP => program[value].opcode = COPY,
                        COPY => program[value].opcode = JUMP,
                        TOGGLE => program[value].opcode = INC,
                        OUT => unreachable!(),
                    }
                    pc += 1;
                }
            }
        }
    }
}