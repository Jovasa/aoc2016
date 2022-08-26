use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use crate::OpCode::{COPY, DEC, INC, JUMP};


enum OpCode {
    COPY,
    JUMP,
    INC,
    DEC
}


struct Op {
    opcode: OpCode,
    first_reg: Option<usize>,
    first_val: Option<i32>,
    second_reg: Option<usize>,
    second_val: Option<i32>,
}

fn parse_register(i: &str) -> Option<usize> {
    if i.len() == 1 {
        let temp = i.chars().next().unwrap();
        if temp.is_alphabetic() {
            let a = 'a' as usize;
            return Some(temp as usize - a);
        }
    }
    None
}

fn parse_value(i: &str) -> Option<i32> {
    match i.parse::<i32>() {
        Ok(i) => Some(i),
        Err(_) => None
    }
}


fn main() {
    let reader = BufReader::new(File::open("data/day12.txt").unwrap());
    let mut program = Vec::new();
    let mut register = [0; 4];

    register[2] = 1;

    for line in reader.lines() {
        let line = line.unwrap();
        let parts = line.split(" ").collect_vec();
        program.push(
            match parts[0] {
                "cpy" => Op{
                    opcode: COPY,
                    first_reg: parse_register(parts[1]),
                    first_val: parse_value(parts[1]),
                    second_reg: parse_register(parts[2]),
                    second_val: parse_value(parts[2]),
                },
                "jnz" => Op{
                    opcode: JUMP,
                    first_reg: parse_register(parts[1]),
                    first_val: parse_value(parts[1]),
                    second_reg: parse_register(parts[2]),
                    second_val: parse_value(parts[2]),
                },
                "inc"=> Op{
                    opcode: INC,
                    first_reg: parse_register(parts[1]),
                    first_val: parse_value(parts[1]),
                    second_reg: None,
                    second_val: None,
                },
                "dec"=> Op{
                    opcode: DEC,
                    first_reg: parse_register(parts[1]),
                    first_val: parse_value(parts[1]),
                    second_reg: None,
                    second_val: None,
                },
                _ => unreachable!()
            }
        )
    }

    let mut pc: i32 = 0;

    while (pc as usize) < program.len() {
        let temp = &program[pc as usize];
        match temp.opcode {
            COPY => {
                let value = match temp.first_val {
                    Some(i) => i,
                    None => register[temp.first_reg.unwrap()]
                };
                register[temp.second_reg.unwrap()] = value;
                pc += 1;
            }
            JUMP => {
                let value = match temp.first_val {
                    Some(i) => i,
                    None => register[temp.first_reg.unwrap()]
                };
                if value != 0 {
                    pc += temp.second_val.unwrap();
                }
                else {
                    pc += 1;
                }
            }
            INC => {
                register[temp.first_reg.unwrap()] += 1;
                pc += 1;
            },
            DEC => {
                register[temp.first_reg.unwrap()] -= 1 ;
                pc += 1;
            }
        }
    }
    println!("{}", register[0]);
}